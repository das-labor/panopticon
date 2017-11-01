use petgraph::Direction;
use petgraph::graph::{NodeIndex};
use petgraph::algo::dominators;
use petgraph::algo::dominators::Dominators;
use bit_set::BitSet;
use smallvec::SmallVec;

use panopticon_core::neo::{Function, Result, CfgNode, Statement, Operation, BasicBlockIndex, Value, Variable};

use neo::Globals;

pub fn phi_functions<'a>(func: &'a Function, globals: &Globals<'a>, domfronts: &Vec<BitSet>) -> Result<Vec<BitSet>> {
    let mut ret = vec![BitSet::with_capacity(globals.variables.len()); func.basic_blocks().len()];

    for g_idx in globals.globals.iter() {
        let mut worklist = globals.usage[g_idx].clone();

        while !worklist.is_empty() {
            let bb_idx = worklist.iter().next().unwrap();

            worklist.remove(bb_idx);
            for df_idx in domfronts[bb_idx].iter() {
                if !ret[df_idx].contains(g_idx) {
                    ret[df_idx].insert(g_idx);
                    worklist.insert(df_idx);
                }
            }
        }
    }

    Ok(ret)
}

fn dominance_frontiers(func: &Function, doms: &Dominators<NodeIndex>) -> Result<Vec<BitSet>> {
    let cfg = func.cflow_graph();
    let num_bb = func.basic_blocks().len();
    let mut ret = vec![BitSet::with_capacity(num_bb); num_bb];

    for n in cfg.node_indices() {
        if let Some(&CfgNode::BasicBlock(bb_idx)) = cfg.node_weight(n) {
            let preds = cfg.neighbors_directed(n,Direction::Incoming).collect::<Vec<_>>();

            if preds.len() > 1 {
                for p in preds {
                    let mut runner = p;
                    let idom = doms.immediate_dominator(n);

                    while idom.is_some() && Some(runner) != idom {
                        if let Some(&CfgNode::BasicBlock(run_idx)) = cfg.node_weight(runner) {
                            ret[run_idx.index()].insert(bb_idx.index());
                            runner = doms.immediate_dominator(runner).unwrap();
                        } else {
                            return Err("Internal error: cfg node has a predecessor that's not a basic block".into());
                        }
                    }
                }
            }
        }
    }

    Ok(ret)
}

fn fix_uninitialized_variables(func: &mut Function, uninit: BitSet, variables: &Vec<Variable>) -> Result<()> {
    let stmts = uninit.iter().map(|bit| {
        let var = &variables[bit];

        Statement::Expression{
            op: Operation::Initialize(var.name.clone(),var.bits),
            result: var.clone()
        }
    }).collect::<Vec<_>>();

    let entry = func.entry_point();
    let remove_first = match func.mnemonics(entry).next() {
        Some((_,mne)) if mne.opcode == "__init" => { true }
        _ => { false }
    };

    if remove_first { func.remove_mnemonic(entry)?; }
    func.prepend_mnemonic(entry,"__init",stmts)
}

struct NameStack {
    counter: Vec<usize>,
    stack: Vec<Vec<usize>>,
}

impl NameStack {
    pub fn new(num_vars: usize) -> NameStack {
        NameStack{
            counter: vec![0; num_vars],
            stack: vec![vec![]; num_vars],
        }
    }

    pub fn new_name(&mut self, var: usize) -> Result<usize> {
        let i = *self.counter.get(var)
            .ok_or("Internal error: unknown variable")?;

        self.counter[var] += 1;
        self.stack[var].push(i);

        Ok(i)
    }

    pub fn pop(&mut self, var: usize) -> Result<()> {
        self.stack[var].pop()
            .map(|_| ())
            .ok_or("Internal error: empty name stack".into())
    }

    pub fn top(&self, var: usize) -> Result<usize> {
        self.stack.get(var)
            .ok_or("Internal error: unknown variable".into())
            .and_then(|x| {
                x.last().cloned()
                 .ok_or("Internal error: empty name stack".into())
            })
    }
}

#[derive(Debug,PartialEq,Eq,PartialOrd,Ord)]
enum DomTreeEvent {
    Enter{
        index: BasicBlockIndex,
        successors: SmallVec<[BasicBlockIndex; 2]>,
        start: u64,
        num_in_edges: usize
    },
    Leave(BasicBlockIndex),
}

fn insert_phi_operations(func: &mut Function, phis: Vec<BitSet>,
                         dom_events: &Vec<DomTreeEvent>, variables: &Vec<Variable>) -> Result<()> {
    // Remove all Phi functions
    let indices = func.basic_blocks().map(|(i,_)| i).collect::<Vec<_>>();
    for idx in indices {
        loop {
            let remove = match func.mnemonics(idx).next() {
                Some((_,mne)) if mne.opcode == "__phi" => true,
                _ => false,
            };

            if remove {
                func.remove_mnemonic(idx)?;
            } else {
                break;
            }
        }
    }

    for ev in dom_events {
        match ev {
            &DomTreeEvent::Enter{ index: bb_idx, num_in_edges,.. } => {
                // Insert new Phi functions
                for var_idx in phis[bb_idx.index()].iter() {
                    let &Variable{ ref name, bits,.. } = &variables[var_idx];
                    let num_phis = if num_in_edges <= 3 { 1 } else { (num_in_edges + 1) / 2 };
                    let mut stmts = Vec::with_capacity(num_phis);

                    for i in 0..num_phis {
                        let prev = if i > 0 {
                            Value::var(name.clone(),bits,None)?
                        } else {
                            Value::undef()
                        };

                        stmts.push(Statement::Expression{
                            op: Operation::Phi(prev,Value::undef(),Value::undef()),
                            result: Variable::new(name.clone(),bits,None)?,
                        });
                    }

                    func.prepend_mnemonic(bb_idx,"__phi",stmts)?;
                }
            }
            &DomTreeEvent::Leave(_) => { /* skip */ }
        }
    }

    Ok(())
}

fn assign_subscripts(func: &mut Function,
                     dom_events: Vec<DomTreeEvent>, variables: &Vec<Variable>) -> Result<()> {
    let find_variable = |v: &Variable| -> Result<usize> {
        variables.iter()
            .position(|w| v.name == w.name && v.bits == w.bits)
            .ok_or(format!("Internal error: unknown variable: {:?}",v).into())
    };
    let mut name_stack = NameStack::new(variables.len());

    for ev in dom_events {
        match ev {
            DomTreeEvent::Enter{ index: bb_idx, successors,.. } => {
                // Rewrite operations
                func.rewrite_mnemonics(bb_idx,|stmt| {
                    use panopticon_core::neo::Statement::*;

                    match stmt {
                        &mut Expression{ op: Operation::Phi(_,_,_), ref mut result } => {
                            let var_idx = find_variable(&*result)?;
                            result.subscript = Some(name_stack.new_name(var_idx)?);
                        }
                        &mut Expression{ ref mut op, ref mut result } => {
                            for v in op.reads_mut() {
                                if let &mut Value::Variable(ref mut var) = v {
                                    let var_idx = find_variable(&*var)?;
                                    var.subscript = Some(name_stack.top(var_idx)?);
                                }
                            }

                            let var_idx = find_variable(&*result)?;
                            result.subscript = Some(name_stack.new_name(var_idx)?);
                        }
                        &mut Call{ .. } => { /* skip */}
                        &mut IndirectCall{ .. } => { /* skip */}
                        &mut Return => { /* skip */}
                        &mut Store{ ref mut address, ref mut value,.. } => {
                            if let &mut Value::Variable(ref mut var) = address {
                                let var_idx = find_variable(&*var)?;
                                var.subscript = Some(name_stack.top(var_idx)?);
                            }
                            if let &mut Value::Variable(ref mut var) = value {
                                let var_idx = find_variable(&*var)?;
                                var.subscript = Some(name_stack.top(var_idx)?);
                            }
                        }
                    }

                    Ok(())
                })?;

                // Fill Phi function args of all successor nodes
                'outer: for succ in successors {
                    debug!("in {} handle successor {}",bb_idx.index(),succ.index());

                    func.rewrite_mnemonics(succ,|stmt| {
                        use panopticon_core::neo::Statement::*;
                        use panopticon_core::neo::Operation::Phi;

                        match stmt {
                            &mut Expression{ op: Phi(ref mut a, ref mut b, ref mut c), ref result } => {
                                debug!("  phi for {:?}",result);
                                let var_idx = find_variable(result)?;
                                if *a == Value::Undefined {
                                    *a = Value::var(result.name.clone(),result.bits,name_stack.top(var_idx)?)?;
                                } else if *b == Value::Undefined {
                                    *b = Value::var(result.name.clone(),result.bits,name_stack.top(var_idx)?)?;
                                } else if *c == Value::Undefined {
                                    *c = Value::var(result.name.clone(),result.bits,name_stack.top(var_idx)?)?;
                                } else {
                                    /* fall-thru to next Phi */
                                }
                            }
                            _ => { /* skip non __phi mnemonics */ }
                        }

                        Ok(())
                    })?;
                }
            }
            DomTreeEvent::Leave(bb_idx) => {
                // Pop ssa names
                func.rewrite_mnemonics(bb_idx,|stmt| {
                    use panopticon_core::neo::Statement::*;

                    match stmt {
                        &mut Expression{ ref mut result,.. } => {
                            let var_idx = find_variable(&*result)?;
                            name_stack.pop(var_idx)?;
                        }
                        &mut Call{ .. } => { /* skip */ }
                        &mut IndirectCall{ .. } => { /* skip */ }
                        &mut Return => { /* skip */ }
                        &mut Store{ .. } => { /* skip */ }
                    }

                    Ok(())
                })?;
            }
        }
    }

    Ok(())
}

fn dominator_tree(func: &Function, doms: &Dominators<NodeIndex>) -> Result<Vec<DomTreeEvent>> {
    let num_bb = func.basic_blocks().len();
    let cfg = func.cflow_graph();
    let mut tree = vec![BitSet::with_capacity(num_bb); num_bb];

    for (bb_idx,bb) in func.basic_blocks() {
        if let Some(idom) = doms.immediate_dominator(bb.node) {
            if let Some(&CfgNode::BasicBlock(p_idx)) = cfg.node_weight(idom) {
                tree[p_idx.index()].insert(bb_idx.index());
            }
        }
    }

    let mut completed = BitSet::with_capacity(num_bb);
    let mut processing = BitSet::with_capacity(num_bb);
    let mut stack = Vec::with_capacity(num_bb);
    let entry_idx = match cfg.node_weight(doms.root()) {
        Some(&CfgNode::BasicBlock(i)) => i,
        _ => { return Err("Internal error: dominator tree root isn't a basic block".into()); }
    };
    let mut ret = Vec::with_capacity(num_bb * 2);

    stack.push(entry_idx.index());

    {
        let entry = func.basic_block(entry_idx);
        let in_edges = cfg.edges_directed(entry.node,Direction::Incoming).count();
        let succ = cfg.neighbors_directed(entry.node,Direction::Outgoing).filter_map(|x| {
            match cfg.node_weight(x) {
                Some(&CfgNode::BasicBlock(i)) => Some(i),
                _ => None
            }
        }).collect::<SmallVec<[BasicBlockIndex; 2]>>();

        ret.push(DomTreeEvent::Enter{
            index: entry_idx,
            start: entry.area.start,
            num_in_edges: in_edges,
            successors: succ,
        });
    }

    'outer: while !stack.is_empty() {
        let n = *stack.last().unwrap();

        for m in tree[n].iter() {
            if !processing.contains(m) {
                // Enter node
                let bb_idx = BasicBlockIndex::new(m);
                let bb = func.basic_block(bb_idx);
                let in_edges = cfg.edges_directed(bb.node,Direction::Incoming).count();
                let succ = cfg.neighbors_directed(bb.node,Direction::Outgoing).filter_map(|x| {
                    match cfg.node_weight(x) {
                        Some(&CfgNode::BasicBlock(i)) => Some(i),
                        _ => None
                    }
                }).collect::<SmallVec<[BasicBlockIndex; 2]>>();

                ret.push(DomTreeEvent::Enter{
                    index: bb_idx,
                    start: bb.area.start,
                    num_in_edges: in_edges,
                    successors: succ,
                });
                stack.push(m);
                processing.insert(m);
                continue 'outer;
            }
        }

        // Leave node
        ret.push(DomTreeEvent::Leave(BasicBlockIndex::new(n)));
        completed.insert(n);
        stack.pop();
    }

    Ok(ret)
}

pub fn rewrite_to_ssa(func: &mut Function) -> Result<()> {
    use neo::{Liveness,Globals};

    let entry = func.entry_point();
    let doms = dominators::simple_fast(func.cflow_graph(),func.basic_block(entry).node);
    let dom_events = dominator_tree(func,&doms)?;
    let (phis,uninit,variables) = {
        let live = Liveness::new(&func).unwrap();
        let globals = Globals::new(&func,&live).unwrap();
        let df = dominance_frontiers(&func,&doms).unwrap();
        let phis = phi_functions(&func,&globals,&df).unwrap();
        let mut uninit = live.ue_var[entry.index()].clone();

        uninit.union_with(&globals.globals.difference(&live.var_kill[entry.index()]).collect());
        (phis,uninit,live.variables.clone())
    };
    fix_uninitialized_variables(func,uninit,&variables)?;
    insert_phi_operations(func,phis,&dom_events,&variables)?;
    assign_subscripts(func,dom_events,&variables)
}

#[cfg(test)]
mod tests {
    use super::*;
    use panopticon_core::{OpaqueLayer, Region, TestArch};
    use panopticon_core::neo::{Function,Variable};
    use neo::{Liveness,Globals};
    use env_logger;
    use std::iter;
    use std::iter::FromIterator;

    /*
     * (B0)
     * 0:  Mx1  ; mov x 1
     *
     * (B1)
     * 3:  Mx1  ; mov x 1
     * 6:  Cfx1 ; cmp f x 1
     * 10: Bf28 ; brle f (B5)
     *
     * (B2)
     * 14: Mx1  ; mov x 1
     *
     * (B3)
     * 17: Mx1  ; mov x 1
     * 20: Cfx1 ; cmp f x 1
     * 24: Bf3  ; brle f (B1)
     *
     * (B4)
     * 27: R    ; ret
     *
     * (B5)
     * 28: Mx1  ; mov x 1
     * 31: Cfx1 ; cmp f x 1
     * 35: Bf48 ; brle f (B8)
     *
     * (B6)
     * 39:  Mx1 ; mov x 1
     *
     * (B7)
     * 42:  Mx1 ; mov x 1
     * 45:  J17 ; jmp (B3)
     *
     * (B8)
     * 48:  J42 ; jmp (B7)
     */
    #[test]
    fn dom_frontiers() {
        let _ = env_logger::init();
        let data = OpaqueLayer::wrap(b"Mx1Mx1Cfx1Bf28Mx1Mx1Cfx1Bf3RMx1Cfx1Bf48Mx1Mx1J17J42".to_vec());
        let reg = Region::new("".to_string(), data);
        let func = Function::new::<TestArch>((), 0, &reg, None).unwrap();
        let ent_idx = func.entry_point();
        let doms = dominators::simple_fast(func.cflow_graph(),func.basic_block(ent_idx).node);
        let df = dominance_frontiers(&func,&doms).unwrap();
        let mut bb1 = None;
        let mut bb3 = None;
        let mut bb7 = None;

        for (idx,bb) in func.basic_blocks() {
            match bb.area.start {
                3 => { bb1 = Some(idx); }
                17 => { bb3 = Some(idx); }
                42 => { bb7 = Some(idx); }
                _ => {}
            };
        }

        assert!(bb1.is_some() && bb3.is_some() && bb7.is_some());

        let set_1 = BitSet::from_iter(iter::once(bb1.unwrap().index()));
        let set_3 = BitSet::from_iter(iter::once(bb3.unwrap().index()));
        let set_7 = BitSet::from_iter(iter::once(bb7.unwrap().index()));

        for (idx,bb) in func.basic_blocks() {
            let expected_df = match bb.area.start {
                0 => BitSet::default(),
                3 => set_1.clone(),
                14 => set_3.clone(),
                17 => set_1.clone(),
                27 => BitSet::default(),
                28 => set_3.clone(),
                39 => set_7.clone(),
                42 => set_3.clone(),
                48 => set_7.clone(),
                a => unreachable!("unexpected address {}",a),
            };

            debug!("check {:?}: {:?}",idx,bb);
            assert_eq!(df[idx.index()], expected_df);
        }
    }

    /*
     * (B0)
     * 0:  Mi1  ; mov i 1
     *
     * (B1)
     * 3:  Ma1  ; mov a 1
     * 6:  Mc1  ; mov c 1
     * 9:  Cfac ; cmp f a c
     * 13: Bf46 ; br f (B5)
     *
     * (B2)
     * 17: Mb1  ; mov b 1
     * 20: Mc1  ; mov c 1
     * 23: Md1  ; mov d 1
     *
     * (B3)
     * 26: Ayab ; add y a b
     * 30: Azcd ; add z c d
     * 34: Aii1 ; add i i 1
     * 38: Cfi1 ; cmp f i 1
     * 42: Bf3  ; br f (B1)
     *
     * (B4)
     * 45: R    ; ret
     *
     * (B5)
     * 46: Ma1  ; mov a 1
     * 49: Md1  ; mov d 1
     * 52: Cfad ; cmp f a d
     * 56: Bf69 ; br f (B8)
     *
     * (B6)
     * 60:  Md1 ; mov d 1
     *
     * (B7)
     * 63:  Mb1 ; mov b 1
     * 66:  J26 ; jmp (B3)
     *
     * (B8)
     * 69:  Mc1 ; mov c 1
     * 72:  J63 ; jmp (B7)
     */
    #[test]
    fn phi_placement() {
        let _ = env_logger::init();
        let data = OpaqueLayer::wrap(b"Mi1Ma1Mc1CfacBf46Mb1Mc1Md1AyabAzcdAii1Cfi1Bf3RMa1Md1CfadBf69Md1Mb1J26Mc1J63".to_vec());
        let reg = Region::new("".to_string(), data);
        let func = Function::new::<TestArch>((), 0, &reg, None).unwrap();
        let ent_idx = func.entry_point();
        let live = Liveness::new(&func).unwrap();
        let globals = Globals::new(&func,&live).unwrap();
        let doms = dominators::simple_fast(func.cflow_graph(),func.basic_block(ent_idx).node);
        let df = dominance_frontiers(&func,&doms).unwrap();
        let phis = phi_functions(&func,&globals,&df).unwrap();
        let i = globals.index_for(&Variable::new("i",32,None).unwrap()).unwrap();
        let a = globals.index_for(&Variable::new("a",32,None).unwrap()).unwrap();
        let b = globals.index_for(&Variable::new("b",32,None).unwrap()).unwrap();
        let c = globals.index_for(&Variable::new("c",32,None).unwrap()).unwrap();
        let d = globals.index_for(&Variable::new("d",32,None).unwrap()).unwrap();
        let abcdi_set = BitSet::from_iter(vec![a,b,c,d,i].into_iter());
        let abcd_set = BitSet::from_iter(vec![a,b,c,d].into_iter());
        let cd_set = BitSet::from_iter(vec![c,d].into_iter());

        for (idx,bb) in func.basic_blocks() {
            let expected_phi = match bb.area.start {
                0 => BitSet::default(),
                3 => abcdi_set.clone(),
                17 => BitSet::default(),
                26 => abcd_set.clone(),
                45 => BitSet::default(),
                46 => BitSet::default(),
                60 => BitSet::default(),
                63 => cd_set.clone(),
                69 => BitSet::default(),
                a => unreachable!("unexpected address {}",a),
            };

            debug!("check {:?}: {:?}",idx,bb);
            assert_eq!(phis[idx.index()], expected_phi);
        }
    }

    /*
     * (B0)
     * 0:  Mi1  ; mov i s
     * 3:  Cfi0 ; cmp f i 0
     * 7:  Bf18 ; br f (B2)
     *
     * (B1)
     * 11: Aii3 ; add i i 3
     * 15: J22  ; jmp (B3)
     *
     * (B2)
     * 18: Ai23 ; add i 2 3
     *
     * (B3)
     * 22: Ms3  ; mov s 3
     * 25: Aisx ; add i s x
     * 29: R    ; ret
     */
    #[test]
    fn uninitialized_variables() {
        let _ = env_logger::init();
        let data = OpaqueLayer::wrap(b"MisCfi0Bf18Aii3J22Ai23Ms3AisxR".to_vec());
        let reg = Region::new("".to_string(), data);
        let mut func = Function::new::<TestArch>((), 0, &reg, None).unwrap();
        let ent_idx = func.entry_point();
        let (uninit,vars) = {
            let live = Liveness::new(&func).unwrap();
            let s = live.index_for(&Variable::new("s", 32, None).unwrap()).unwrap();
            let x = live.index_for(&Variable::new("x", 32, None).unwrap()).unwrap();
            let xs_set = BitSet::from_iter(vec![x,s].into_iter());
            let uninit = live.ue_var[ent_idx.index()].clone();

            assert_eq!(uninit, xs_set);

            (uninit,live.variables.clone())
        };
        let _ = fix_uninitialized_variables(&mut func,uninit,&vars).unwrap();

        let mnes = func.mnemonics(ent_idx).collect::<Vec<_>>();
        assert_eq!(mnes.len(), 4);

        assert_eq!(mnes[0].1.opcode, "__init");
        let stmts = func.statements(ent_idx).collect::<Vec<_>>();
        assert_eq!(stmts.len(), 4);
        if let Statement::Expression{ op: Operation::Initialize(ref name,len), result: Variable{ name: ref res_name, bits: res_bits,.. } } = stmts[0] {
            assert_eq!(name, res_name);
            assert_eq!(len, res_bits);
            assert!(name == "x" || name == "s");
        } else {
            unreachable!()
        }
        if let Statement::Expression{ op: Operation::Initialize(ref name,len), result: Variable{ name: ref res_name, bits: res_bits,.. } } = stmts[1] {
            assert_eq!(name, res_name);
            assert_eq!(len, res_bits);
            assert!(name == "x" || name == "s");
        } else {
            unreachable!()
        }

        let live = Liveness::new(&func).unwrap();
        let uninit = live.ue_var[ent_idx.index()].clone();

        assert!(uninit.is_empty());
    }

    #[test]
    fn uninitialized_variables_reentrant() {
        let _ = env_logger::init();
        let data = OpaqueLayer::wrap(b"MisCfi0Bf18Aii3J22Ai23Ms3AisxR".to_vec());
        let reg = Region::new("".to_string(), data);
        let mut func = Function::new::<TestArch>((), 0, &reg, None).unwrap();
        let ent_idx = func.entry_point();
        let (uninit,vars) = {
            let live = Liveness::new(&func).unwrap();
            let s = live.index_for(&Variable::new("s", 32, None).unwrap()).unwrap();
            let x = live.index_for(&Variable::new("x", 32, None).unwrap()).unwrap();
            let xs_set = BitSet::from_iter(vec![x,s].into_iter());
            let uninit = live.ue_var[ent_idx.index()].clone();

            assert_eq!(uninit, xs_set);

            (uninit,live.variables.clone())
        };
        debug!("before 1st");
        let _ = fix_uninitialized_variables(&mut func,uninit.clone(),&vars).unwrap();
        debug!("after 1st");

        {
            let mnes = func.mnemonics(ent_idx).collect::<Vec<_>>();
            assert_eq!(mnes.len(), 4);

            assert_eq!(mnes[0].1.opcode, "__init");
            let stmts = func.statements(ent_idx).collect::<Vec<_>>();
            assert_eq!(stmts.len(), 4);
            if let Statement::Expression{ op: Operation::Initialize(ref name,len), result: Variable{ name: ref res_name, bits: res_bits,.. } } = stmts[0] {
                assert_eq!(name, res_name);
                assert_eq!(len, res_bits);
                assert!(name == "x" || name == "s");
            } else {
                unreachable!()
            }
            if let Statement::Expression{ op: Operation::Initialize(ref name,len), result: Variable{ name: ref res_name, bits: res_bits,.. } } = stmts[1] {
                assert_eq!(name, res_name);
                assert_eq!(len, res_bits);
                assert!(name == "x" || name == "s");
            } else {
                unreachable!()
            }

            let live = Liveness::new(&func).unwrap();
            let uninit = live.ue_var[ent_idx.index()].clone();

            assert!(uninit.is_empty());
        }

        debug!("before 2nd");
        let _ = fix_uninitialized_variables(&mut func,uninit,&vars).unwrap();
        debug!("done");

        {
            let mnes = func.mnemonics(ent_idx).collect::<Vec<_>>();
            assert_eq!(mnes.len(), 4);

            assert_eq!(mnes[0].1.opcode, "__init");
            let stmts = func.statements(ent_idx).collect::<Vec<_>>();
            assert_eq!(stmts.len(), 4);
            if let Statement::Expression{ op: Operation::Initialize(ref name,len), result: Variable{ name: ref res_name, bits: res_bits,.. } } = stmts[0] {
                assert_eq!(name, res_name);
                assert_eq!(len, res_bits);
                assert!(name == "x" || name == "s");
            } else {
                unreachable!()
            }
            if let Statement::Expression{ op: Operation::Initialize(ref name,len), result: Variable{ name: ref res_name, bits: res_bits,.. } } = stmts[1] {
                assert_eq!(name, res_name);
                assert_eq!(len, res_bits);
                assert!(name == "x" || name == "s");
            } else {
                unreachable!()
            }


            let live = Liveness::new(&func).unwrap();
            let uninit = live.ue_var[ent_idx.index()].clone();

            assert!(uninit.is_empty());
        }
    }

    /*
     * (B0)
     * 0:  Mi1  ; mov i 1
     *
     * (B1)
     * 3:  Ma1  ; mov a 1
     * 6:  Mc1  ; mov c 1
     * 9:  Cfac ; cmp f a c
     * 13: Bf46 ; br f (B5)
     *
     * (B2)
     * 17: Mb1  ; mov b 1
     * 20: Mc1  ; mov c 1
     * 23: Md1  ; mov d 1
     *
     * (B3)
     * 26: Ayab ; add y a b
     * 30: Azcd ; add z c d
     * 34: Aii1 ; add i i 1
     * 38: Cfi1 ; cmp f i 1
     * 42: Bf3  ; br f (B1)
     *
     * (B4)
     * 45: R    ; ret
     *
     * (B5)
     * 46: Ma1  ; mov a 1
     * 49: Md1  ; mov d 1
     * 52: Cfad ; cmp f a d
     * 56: Bf69 ; br f (B8)
     *
     * (B6)
     * 60:  Md1 ; mov d 1
     *
     * (B7)
     * 63:  Mb1 ; mov b 1
     * 66:  J26 ; jmp (B3)
     *
     * (B8)
     * 69:  Mc1 ; mov c 1
     * 72:  J63 ; jmp (B7)
     */
    #[test]
    fn dominator_tree_events() {
        let _ = env_logger::init();
        let data = OpaqueLayer::wrap(b"Mi1Ma1Mc1CfacBf46Mb1Mc1Md1AyabAzcdAii1Cfi1Bf3RMa1Md1CfadBf69Md1Mb1J26Mc1J63".to_vec());
        let reg = Region::new("".to_string(), data);
        let func = Function::new::<TestArch>((), 0, &reg, None).unwrap();
        let ent_idx = func.entry_point();
        let doms = dominators::simple_fast(func.cflow_graph(),func.basic_block(ent_idx).node);
        let mut events = dominator_tree(&func,&doms).unwrap();

        let mut ctx = BitSet::with_capacity(func.basic_blocks().len());
        for ev in events.iter() {
            match ev {
                &DomTreeEvent::Enter{ ref index, num_in_edges, ref successors, start } => {
                    debug!("enter {}",index.index());
                    assert!(ctx.insert(index.index()));
                    match start {
                        0 => {
                            assert_eq!(num_in_edges, 0);
                            assert_eq!(successors.len(), 1);
                            assert_eq!(func.basic_block(successors[0]).area.start, 3);
                        }
                        3 => {
                            assert_eq!(num_in_edges, 2);
                            assert_eq!(successors.len(), 2);
                            let succ0_addr = func.basic_block(successors[0]).area.start;
                            let succ1_addr = func.basic_block(successors[1]).area.start;
                            assert!(succ0_addr == 17 || succ0_addr == 46);
                            assert!(succ1_addr == 17 || succ1_addr == 46);
                        }
                        17 => {
                            assert_eq!(num_in_edges, 1);
                            assert_eq!(successors.len(), 1);
                            assert_eq!(func.basic_block(successors[0]).area.start, 26);
                        }
                        26 => {
                            assert_eq!(num_in_edges, 2);
                            assert_eq!(successors.len(), 2);
                            let succ0_addr = func.basic_block(successors[0]).area.start;
                            let succ1_addr = func.basic_block(successors[1]).area.start;
                            assert!(succ0_addr == 3 || succ0_addr == 45);
                            assert!(succ1_addr == 3 || succ1_addr == 45);
                        }
                        45 => {
                            assert_eq!(num_in_edges, 1);
                            assert_eq!(successors.len(), 0);
                        }
                        46 => {
                            assert_eq!(num_in_edges, 1);
                            assert_eq!(successors.len(), 2);
                            let succ0_addr = func.basic_block(successors[0]).area.start;
                            let succ1_addr = func.basic_block(successors[1]).area.start;
                            assert!(succ0_addr == 60 || succ0_addr == 69);
                            assert!(succ1_addr == 60 || succ1_addr == 69);
                        }
                        60 => {
                            assert_eq!(num_in_edges, 1);
                            assert_eq!(successors.len(), 1);
                            assert_eq!(func.basic_block(successors[0]).area.start, 63);
                        }
                        63 => {
                            assert_eq!(num_in_edges, 2);
                            assert_eq!(successors.len(), 1);
                            assert_eq!(func.basic_block(successors[0]).area.start, 26);
                        }
                        69 => {
                            assert_eq!(num_in_edges, 1);
                            assert_eq!(successors.len(), 1);
                            assert_eq!(func.basic_block(successors[0]).area.start, 63);
                        }
                        _ => { unreachable!() }
                    }
                }
                &DomTreeEvent::Leave(ref index) => {
                    debug!("leave {}",index.index());
                    assert!(index.index() <= 8);
                    assert!(ctx.remove(index.index()));
                }
            }
        }

        assert!(ctx.is_empty());

        assert_eq!(events.len(), 9*2);
        events.sort();
        events.dedup();
        assert_eq!(events.len(), 9*2);
    }

    #[test]
    fn phi_rename() {
        let _ = env_logger::init();
        let data = OpaqueLayer::wrap(b"Mi1Ma1Mc1CfacBf46Mb1Mc1Md1AyabAzcdAii1Cfi1Bf3RMa1Md1CfadBf69Md1Mb1J26Mc1J63".to_vec());
        let reg = Region::new("".to_string(), data);
        let mut func = Function::new::<TestArch>((), 0, &reg, None).unwrap();
        let _ = rewrite_to_ssa(&mut func).unwrap();
        for s in func.statements(..) {
            debug!("{:?}",s);
        }
    }
}

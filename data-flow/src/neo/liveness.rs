use std::marker::PhantomData;
use panopticon_core::neo::{Function,Operation,Statement,Variable,Value,Result};
use panopticon_core::{Guard,Rvalue};
use bit_set::BitSet;
use petgraph::Direction;

pub struct LivenessSets<'a> {
    variables: Vec<Variable>,
    var_kill: Vec<BitSet>,
    ue_var: Vec<BitSet>,
    phantom: PhantomData<&'a Function>,
}

impl<'a> LivenessSets<'a> {
    fn new(func: &'a Function) -> LivenessSets<'a> {
        let num_bb = func.basic_blocks().len();
        LivenessSets{
            variables: vec![],
            var_kill: vec![BitSet::default(); num_bb],
            ue_var: vec![BitSet::default(); num_bb],
            phantom: PhantomData::default(),
        }
    }

    fn record_read(&mut self, bb: usize, var: &Variable) -> Result<()> {
        let &Variable{ ref name, bits,.. } = var;
        let vk = &mut self.var_kill[bb];
        let uev = &mut self.ue_var[bb];
        let var = Variable::new(name.clone(),bits,None)?;
        let var_idx = match self.variables.iter().position(|x| *x == var) {
            Some(p) => p,
            None => { self.variables.push(var); self.variables.len() - 1 }
        };

        if !vk.contains(var_idx) {
            uev.insert(var_idx);
        }

        Ok(())
    }

    fn record_write(&mut self, bb: usize, var: &Variable) -> Result<()> {
        let &Variable{ ref name, bits,.. } = var;
        let vk = &mut self.var_kill[bb];
        let var = Variable::new(name.clone(),bits,None)?;
        let var_idx = match self.variables.iter().position(|x| *x == var) {
            Some(p) => p,
            None => { self.variables.push(var); self.variables.len() - 1 }
        };

        vk.insert(var_idx);
        Ok(())
    }
}

/// Computes the set of killed (VarKill) and upward exposed variables (UEvar) for each basic block
/// in `func`. Returns (VarKill,UEvar).
pub fn liveness_sets<'a>(func: &'a Function) -> Result<LivenessSets<'a>> {
    let mut ret = LivenessSets::new(func);
    let cfg = func.cflow_graph();

    for (idx,bb) in func.basic_blocks() {
        for stmt in func.statements(bb) {
            match stmt {
                Statement::Expression{ op: Operation::Phi(_,_,_),.. } => { /* skip */ }
                Statement::Expression{ op, result } => {
                    for value in op.reads() {
                        if let &Value::Variable(ref var) = value {
                            ret.record_read(idx.index(),var)?;
                        }
                    }
                    ret.record_write(idx.index(),&result)?;
                }
                Statement::IndirectCall{ target: Value::Variable(ref var) } => {
                    ret.record_read(idx.index(),var)?;
                }
                Statement::Store{ address, value,.. } => {
                    if let Value::Variable(ref var) = address {
                        ret.record_read(idx.index(),var)?;
                    }
                    if let Value::Variable(ref var) = value {
                        ret.record_read(idx.index(),var)?;
                    }
                }
                _ => { /* skip */ }
            }
        }

        for e in cfg.edges_directed(bb.node,Direction::Outgoing) {
            match e.weight() {
                &Guard::Predicate{ flag: Rvalue::Variable{ ref name, size,.. },.. } => {
                    ret.record_read(idx.index(),&Variable::new(name.clone(),size,None)?)?;
                }
                _ => { /* skip */ }
            }
        }
    }

    Ok(ret)
}

pub struct Globals<'a> {
    pub variables: Vec<Variable>,
    pub globals: BitSet,
    pub usage: Vec<BitSet>,
    phantom: PhantomData<&'a Function>,
}

impl<'a> Globals<'a> {
    fn new(func: &'a Function, liveness: &LivenessSets<'a>) -> Globals<'a> {
        let num_bb = func.basic_blocks().len();
        Globals{
            variables: liveness.variables.clone(),
            globals: BitSet::with_capacity(liveness.variables.len()),
            usage: vec![BitSet::with_capacity(num_bb); liveness.variables.len()],
            phantom: PhantomData::default(),
        }
    }
}

pub fn globals<'a>(func: &'a Function, liveness: &LivenessSets<'a>) -> Result<Globals<'a>> {
    let mut ret = Globals::new(func,liveness);

    for uevar in liveness.ue_var.iter() {
        ret.globals.union(uevar);
    }

    for (bb,varkill) in liveness.var_kill.iter().enumerate() {
        for i in varkill.iter() { ret.usage[i].insert(bb); }
    }

    Ok(ret)
}

/*
/// Computes for each basic block in `func` the set of live variables using simple fixed point
/// iteration.
pub fn liveness<'a>(func: &'a Function, &LivenessSets<'a>) -> HashMap<ControlFlowRef, HashSet<Cow<'static, str>>> {
    let (varkill, uevar) = liveness_sets(func);
    let mut liveout = HashMap::<ControlFlowRef, HashSet<&str>>::new();
    let ord = func.postorder();
    let cfg = func.cfg();

    for &vx in ord.iter() {
        if let Some(&ControlFlowTarget::Resolved(_)) = cfg.vertex_label(vx) {
            liveout.insert(vx, HashSet::<&str>::new());
        }
    }

    // compute LiveOut sets
    let mut fixpoint = false;
    while !fixpoint {
        let mut new_liveout = HashMap::<ControlFlowRef, HashSet<&str>>::new();

        fixpoint = true;
        for &vx in ord.iter() {
            let mut s = HashSet::<&str>::new();

            if let Some(&ControlFlowTarget::Resolved(_)) = cfg.vertex_label(vx) {
                for e in cfg.out_edges(vx) {
                    let m = cfg.target(e);

                    for x in uevar[&m].iter() {
                        s.insert(x);
                    }

                    if let Some(&ControlFlowTarget::Resolved(_)) = cfg.vertex_label(m) {
                        for x in liveout[&m].iter() {
                            if !varkill[&m].contains(*x) {
                                s.insert(x);
                            }
                        }
                    }
                }

                if liveout[&vx] != s {
                    fixpoint = false;
                }

                new_liveout.insert(vx, s);
            }
        }

        liveout = new_liveout;
    }

    HashMap::from_iter(liveout.iter().map(|(&k, v)| (k, HashSet::from_iter(v.iter().map(|x| Cow::Owned(x.to_string()))))))
}*/

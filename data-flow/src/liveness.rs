use std::marker::PhantomData;
use panopticon_core::il::neo::{Operation,Statement};
use panopticon_core::il::{RREIL, Language};
use panopticon_core::{Function,Guard,Rvalue,Variable,Value,Result,CfgNode};
use bit_set::BitSet;
use petgraph::Direction;
use std::borrow::{Cow};

/// Computes the set of killed (VarKill) and upward exposed variables (UEvar) for each basic block
/// in `func`. Returns (VarKill,UEvar).
#[derive(Debug)]
pub struct Liveness<'a> {
    pub variables: Vec<Variable>,
    pub var_kill: Vec<BitSet>,
    pub ue_var: Vec<BitSet>,
    phantom: PhantomData<&'a Function>,
}

impl<'a> Liveness<'a> {
    pub fn new_rreil(func: &'a Function<RREIL>) -> Result<Liveness<'a>> {
        use panopticon_core::{Operation, Statement, Rvalue};
        let num_bb = func.basic_blocks().len();
        let mut ret = Liveness{
            variables: vec![],
            var_kill: vec![BitSet::default(); num_bb],
            ue_var: vec![BitSet::default(); num_bb],
            phantom: PhantomData::default(),
        };
        let cfg = func.cfg();

        for (idx, bb )in func.basic_blocks() {
            for (mne, _) in func.mnemonics(bb) {
                for statement in func.statements(mne) {
                    match statement {
                        Statement { assignee, op: Operation::Phi(_) } => { /* skip */ },
                        Statement { assignee, op } => {
                            for value in op.operands() {
                                match value {
                                    &Rvalue::Variable { ref name, ref subscript, size, .. } => {
                                        let var = Variable::new(Cow::Owned(name.to_string()), size, subscript.clone())?;
                                        ret.record_read(idx.index(), &var)?;
                                    },
                                    _ => ()
                                }
                            }
                        }
                    }
                }
            }

            for e in cfg.edges_directed(bb.node, Direction::Outgoing) {
                match e.weight() {
                    &Guard::Predicate { flag: Rvalue::Variable { ref name, size, .. }, .. } => {
                        ret.record_read(idx.index(), &Variable::new(name.clone(), size, None)?)?;
                    }
                    _ => { /* skip */ }
                }
            }
        }

        Self::propagate(func,&mut ret);

        Ok(ret)
    }
    pub fn new(func: &'a Function) -> Result<Liveness<'a>> {
        let num_bb = func.basic_blocks().len();
        let mut ret = Liveness{
            variables: vec![],
            var_kill: vec![BitSet::default(); num_bb],
            ue_var: vec![BitSet::default(); num_bb],
            phantom: PhantomData::default(),
        };
        let cfg = func.cfg();

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

        Self::propagate(func,&mut ret);

        Ok(ret)
    }

    fn propagate<IL: Language>(func: &Function<IL>, liveness_sets: &mut Self) {
        let cfg = func.cfg();
        let mut fixedpoint = false;

        while !fixedpoint {
            fixedpoint = true;
            for (bb_idx,bb) in func.basic_blocks().rev() {
                let vk = &liveness_sets.var_kill[bb_idx.index()];

                for succ in cfg.neighbors_directed(bb.node,Direction::Outgoing) {
                    match cfg.node_weight(succ) {
                        Some(&CfgNode::BasicBlock(succ_idx)) => {
                            let in_ue = liveness_sets.ue_var[succ_idx.index()].clone();

                            for var in in_ue.into_iter() {
                                // XXX
                                if !vk.contains(var) {
                                    fixedpoint &= !liveness_sets.ue_var[bb_idx.index()].insert(var);
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
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

        // XXX
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

    pub fn index_for(&self, v: &Variable) -> Result<usize> {
        self.variables.iter()
            .position(|x| x == v)
            .ok_or("variable not part of this liveness set".into())
    }
}

#[derive(Debug)]
pub struct Globals<'a> {
    pub variables: Vec<Variable>,
    pub globals: BitSet,
    pub usage: Vec<BitSet>,
    phantom: PhantomData<&'a Function>,
}

impl<'a> Globals<'a> {
    pub fn new<IL: Language>(func: &'a Function<IL>, liveness: &Liveness<'a>) -> Result<Globals<'a>> {
        let num_bb = func.basic_blocks().len();
        let mut ret = Globals{
            variables: liveness.variables.clone(),
            globals: BitSet::with_capacity(liveness.variables.len()),
            usage: vec![BitSet::with_capacity(num_bb); liveness.variables.len()],
            phantom: PhantomData::default(),
        };

        Self::compute(&mut ret,liveness);
        Ok(ret)
    }

    fn compute(globals: &mut Self, liveness: &Liveness<'a>) {
        for uevar in liveness.ue_var.iter() {
            globals.globals.union_with(uevar);
        }

        for (bb,varkill) in liveness.var_kill.iter().enumerate() {
            for i in varkill.iter() { globals.usage[i].insert(bb); }
        }
    }

    pub fn index_for(&self, v: &Variable) -> Result<usize> {
        self.variables.iter()
            .position(|x| x == v)
            .ok_or("variable not part of this globals set".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use panopticon_core::{Function, Guard, OpaqueLayer, Region, TestArch};
    use ssa::{phi_functions, rename_variables};
    use std::borrow::Cow;
    use std::collections::HashSet;
    use std::iter::FromIterator;
    use env_logger;

    /*
     * (B0)
     * 0:  Mi1   ; mov i 1
     *
     * (B1)
     * 3:  Cxi1  ; cmp x i 1
     * 7:  Bx14  ; brlt x 14
     *
     * (B2)
     * 11: Ms0   ; mov s 0
     *
     * (B3)
     * 14: Assi  ; add s s i
     * 18: Aiii  ; add i i i
     * 22: Cxi1  ; cmp x i 1
     * 26: Bx3   ; brlt x 3
     *
     * (B4)
     * 29: Mss   ; use s
     * 32: R     ; ret
     */
    #[test]
    fn live_variables_analysis() {
        use std::iter;

        let _ = env_logger::init();
        let data = OpaqueLayer::wrap(b"Mi1Cxi1Bx14Ms0AssiAiiiCxi1Bx3Mss".to_vec());
        let reg = Region::new("".to_string(), data);
        let func = Function::new::<TestArch>((), 0, &reg, None).unwrap();
        let lvs = Liveness::new(&func).unwrap();
        let i = lvs.index_for(&Variable::new("i", 32, None).unwrap()).unwrap();
        let x = lvs.index_for(&Variable::new("x", 1, None).unwrap()).unwrap();
        let s = lvs.index_for(&Variable::new("s", 32, None).unwrap()).unwrap();
        let i_set = BitSet::from_iter(iter::once(i));
        let x_set = BitSet::from_iter(iter::once(x));
        let s_set = BitSet::from_iter(iter::once(s));
        let is_set = BitSet::from_iter(vec![i,s].into_iter());
        let isx_set = BitSet::from_iter(vec![i,s,x].into_iter());

        assert_eq!(lvs.ue_var.len(), 5);
        assert_eq!(lvs.ue_var[0], s_set);
        assert_eq!(lvs.ue_var[1], is_set);
        assert_eq!(lvs.ue_var[2], i_set);
        assert_eq!(lvs.ue_var[3], is_set);
        assert_eq!(lvs.ue_var[4], s_set);

        assert_eq!(lvs.var_kill.len(), 5);
        assert_eq!(lvs.var_kill[0], i_set);
        assert_eq!(lvs.var_kill[1], x_set);
        assert_eq!(lvs.var_kill[2], s_set);
        assert_eq!(lvs.var_kill[3], isx_set);
        assert_eq!(lvs.var_kill[4], s_set);
    }

    /*
     * (B0)
     * 0:  Mix   ; mov i x
     * 3:  J9    ; B1
     *
     * (B1)
     * 9:  Mis   ; mov i s
     * 12: R     ; ret
     */
    #[test]
    fn ue_vars_trans() {
        use std::iter;

        let _ = env_logger::init();
        let data = OpaqueLayer::wrap(b"MixJ9xxxxMisR".to_vec());
        let reg = Region::new("".to_string(), data);
        let func = Function::new::<TestArch>((), 0, &reg, None).unwrap();
        let lvs = Liveness::new(&func).unwrap();
        let x = lvs.index_for(&Variable::new("x", 32, None).unwrap()).unwrap();
        let s = lvs.index_for(&Variable::new("s", 32, None).unwrap()).unwrap();
        let s_set = BitSet::from_iter(iter::once(s));
        let xs_set = BitSet::from_iter(vec![x,s].into_iter());

        assert_eq!(lvs.ue_var.len(), 2);
        assert_eq!(lvs.ue_var[0], xs_set);
        assert_eq!(lvs.ue_var[1], s_set);
    }

    /*
     * (B0)
     * 0:  Mi1   ; mov i 1
     *
     * (B1)
     * 3:  Cxi1  ; cmp x i 1
     * 7:  Bx3   ; brlt x 3
     *
     * (B2)
     * 10: Ms0   ; mov s 0
     * 13: Assi  ; add s s i
     * 17: Aiii  ; add i i i
     * 21: My1   ; mov y 1
     * 24: Aiiy  ; add i i y
     * 28: Cxi1  ; cmp x i 1
     * 32: Bx3   ; brlt x _
     *
     * (B3)
     * 37: My4   ; mov y 4
     * 41: Mss   ; use s
     * 44: R     ; ret
     */
    #[test]
    fn global_variables() {
        let _ = env_logger::init();
        let data = OpaqueLayer::wrap(b"Mi1Cxi1Bx3Ms0AssiAiiiMy1AiiyCxi1Bx3My4MssR".to_vec());
        let reg = Region::new("".to_string(), data);
        let func = Function::new::<TestArch>((), 0, &reg, None).unwrap();
        let lvs = Liveness::new(&func).unwrap();
        let gbls = Globals::new(&func,&lvs).unwrap();
        let i = gbls.index_for(&Variable::new("i", 32, None).unwrap()).unwrap();
        let x = gbls.index_for(&Variable::new("x", 1, None).unwrap()).unwrap();
        let s = gbls.index_for(&Variable::new("s", 32, None).unwrap()).unwrap();
        let y = gbls.index_for(&Variable::new("y", 32, None).unwrap()).unwrap();
        let isx_set = BitSet::from_iter(vec![i,s].into_iter());
        let i_usage = BitSet::from_iter(vec![0,2].into_iter());
        let s_usage = BitSet::from_iter(vec![2,3].into_iter());
        let x_usage = BitSet::from_iter(vec![1,2].into_iter());
        let y_usage = BitSet::from_iter(vec![2,3].into_iter());

        assert_eq!(gbls.globals, isx_set);
        assert_eq!(gbls.usage.len(), 4);
        assert_eq!(gbls.usage[i], i_usage);
        assert_eq!(gbls.usage[s], s_usage);
        assert_eq!(gbls.usage[x], x_usage);
        assert_eq!(gbls.usage[y], y_usage);
    }
}

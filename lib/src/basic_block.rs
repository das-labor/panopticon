use std::ops::Range;
use std::cmp::{min,max};

use mnemonic::Mnemonic;
use instr::Instr;

#[derive(PartialEq,Eq,Debug)]
pub struct BasicBlock {
    pub area: Range<u64>,
    pub mnemonics: Vec<Mnemonic>,
}

impl BasicBlock {
    pub fn new() -> BasicBlock {
        BasicBlock{
            area: 0..0,
            mnemonics: Vec::new(),
        }
    }

    pub fn from_vec(ms: Vec<Mnemonic>) -> BasicBlock {
        let a = ms.iter().fold(None,|acc: Option<Range<u64>>,m| {
            if acc == None {
                return Some(m.area.clone());
            } else {
                let r1 = &acc.unwrap();
                let r2 = &m.area;
                return Some(min(r1.start,r2.start)..max(r1.end,r2.end));
            }
        });
        return BasicBlock{ area: a.unwrap_or(0..0), mnemonics: ms };
    }

    pub fn execute<F>(&self,mut f: F) where F: FnMut(&Instr) {
        for mne in self.mnemonics.iter() {
            for i in mne.instructions.iter() {
                f(&i);
            }
        }
    }

    pub fn rewrite<F>(&mut self,f: F) where F: Fn(&Instr) -> Instr {
        for mne in self.mnemonics.iter_mut() {
            for i in mne.instructions.iter_mut() {
                *i = f(&i);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use value::{Rvalue,Lvalue};
    use instr::{Instr,Operation};
    use mnemonic::Mnemonic;

    #[test]
    fn construct() {
        let ops1 = vec!(Rvalue::Constant(1),Rvalue::Variable{ name: "a".to_string(), width: 3, subscript: None });
        let i1 = vec!(
            Instr{ op: Operation::IntAdd(Rvalue::Constant(1),Rvalue::Constant(2)), assignee: Lvalue::Variable{ name: "a".to_string(), width: 8, subscript: Some(2) }},
            Instr{ op: Operation::IntAdd(Rvalue::Constant(4),Rvalue::Constant(2)), assignee: Lvalue::Variable{ name: "a".to_string(), width: 8, subscript: Some(1) }},
            Instr{ op: Operation::Phi(vec!(
                Rvalue::Variable{ name: "a".to_string(), width: 8, subscript: Some(2) },
                Rvalue::Variable{ name: "a".to_string(), width: 8, subscript: Some(1) })), assignee: Lvalue::Variable{ name: "a".to_string(), width: 8, subscript: Some(3) }});
        let mne1 = Mnemonic::new(0..10,"op1".to_string(),"{8:-:eax} nog".to_string(),ops1.iter(),i1.iter());

        let ops2 = vec!(Rvalue::Constant(1),Rvalue::Variable{ name: "a".to_string(), width: 3, subscript: None });
        let i2 = vec!(
            Instr{ op: Operation::IntAdd(Rvalue::Constant(1),Rvalue::Constant(2)), assignee: Lvalue::Variable{ name: "a".to_string(), width: 8, subscript: Some(2) }},
            Instr{ op: Operation::IntAdd(Rvalue::Constant(4),Rvalue::Constant(2)), assignee: Lvalue::Variable{ name: "a".to_string(), width: 8, subscript: Some(1) }},
            Instr{ op: Operation::Phi(vec!(
                Rvalue::Variable{ name: "a".to_string(), width: 8, subscript: Some(2) },
                Rvalue::Variable{ name: "a".to_string(), width: 8, subscript: Some(1) })), assignee: Lvalue::Variable{ name: "a".to_string(), width: 8, subscript: Some(3) }});
        let mne2 = Mnemonic::new(10..13,"op3".to_string(),"{8:-:eax} nog".to_string(),ops2.iter(),i2.iter());

        let ops3 = vec!(Rvalue::Constant(1),Rvalue::Variable{ name: "a".to_string(), width: 3, subscript: None });
        let i3 = vec!(
            Instr{ op: Operation::IntAdd(Rvalue::Constant(1),Rvalue::Constant(2)), assignee: Lvalue::Variable{ name: "a".to_string(), width: 8, subscript: Some(2) }},
            Instr{ op: Operation::IntAdd(Rvalue::Constant(4),Rvalue::Constant(2)), assignee: Lvalue::Variable{ name: "a".to_string(), width: 8, subscript: Some(1) }},
            Instr{ op: Operation::Phi(vec!(
                Rvalue::Variable{ name: "a".to_string(), width: 8, subscript: Some(2) },
                Rvalue::Variable{ name: "a".to_string(), width: 8, subscript: Some(1) })), assignee: Lvalue::Variable{ name: "a".to_string(), width: 8, subscript: Some(3) }});
        let mne3 = Mnemonic::new(13..20,"op3".to_string(),"{8:-:eax} nog".to_string(),ops3.iter(),i3.iter());

        let ms = vec!(mne1,mne2,mne3);
        let bb1 = BasicBlock::from_vec(ms);

        assert_eq!(bb1.area, 0..20);

        let bb2 = BasicBlock::new();
        assert!(bb1 != bb2);

        let bb3 = BasicBlock::new();
        assert_eq!(bb3, bb2);
    }

    #[test]
    fn execute() {
        let ops1 = vec!(Rvalue::Constant(1),Rvalue::Variable{ name: "a".to_string(), width: 3, subscript: None });
        let i1 = vec!(
            Instr{ op: Operation::IntAdd(Rvalue::Constant(1),Rvalue::Constant(2)), assignee: Lvalue::Variable{ name: "a".to_string(), width: 8, subscript: Some(2) }},
            Instr{ op: Operation::IntAdd(Rvalue::Constant(4),Rvalue::Constant(2)), assignee: Lvalue::Variable{ name: "a".to_string(), width: 8, subscript: Some(1) }},
            Instr{ op: Operation::Phi(vec!(
                Rvalue::Variable{ name: "a".to_string(), width: 8, subscript: Some(2) },
                Rvalue::Variable{ name: "a".to_string(), width: 8, subscript: Some(1) })), assignee: Lvalue::Variable{ name: "a".to_string(), width: 8, subscript: Some(3) }});
        let mne1 = Mnemonic::new(0..10,"op1".to_string(),"{8:-:eax} nog".to_string(),ops1.iter(),i1.iter());

        let ops2 = vec!(Rvalue::Constant(1),Rvalue::Variable{ name: "a".to_string(), width: 3, subscript: None });
        let i2 = vec!(
            Instr{ op: Operation::IntAdd(Rvalue::Constant(1),Rvalue::Constant(2)), assignee: Lvalue::Variable{ name: "a".to_string(), width: 8, subscript: Some(2) }},
            Instr{ op: Operation::IntAdd(Rvalue::Constant(4),Rvalue::Constant(2)), assignee: Lvalue::Variable{ name: "a".to_string(), width: 8, subscript: Some(1) }},
            Instr{ op: Operation::Phi(vec!(
                Rvalue::Variable{ name: "a".to_string(), width: 8, subscript: Some(2) },
                Rvalue::Variable{ name: "a".to_string(), width: 8, subscript: Some(1) })), assignee: Lvalue::Variable{ name: "a".to_string(), width: 8, subscript: Some(3) }});
        let mne2 = Mnemonic::new(10..13,"op3".to_string(),"{8:-:eax} nog".to_string(),ops2.iter(),i2.iter());

        let ms = vec!(mne1,mne2);
        let bb = BasicBlock::from_vec(ms);
        let mut vs2 = i1;
        let mut vs1 = Vec::<Instr>::new();

        bb.execute(|i| vs1.push(i.clone()));

        for i in i2.iter() {
            vs2.push(i.clone());
        }

        assert_eq!(vs1, vs2);
    }

    #[test]
    fn rewrite() {
        let ops1 = vec!(Rvalue::Constant(1),Rvalue::Variable{ name: "a".to_string(), width: 3, subscript: None });
        let i1 = vec!(
            Instr{ op: Operation::IntAdd(Rvalue::Constant(1),Rvalue::Constant(2)), assignee: Lvalue::Variable{ name: "a".to_string(), width: 8, subscript: Some(2) }},
            Instr{ op: Operation::IntAdd(Rvalue::Constant(4),Rvalue::Constant(2)), assignee: Lvalue::Variable{ name: "a".to_string(), width: 8, subscript: Some(1) }},
            Instr{ op: Operation::Phi(vec!(
                Rvalue::Variable{ name: "a".to_string(), width: 8, subscript: Some(2) },
                Rvalue::Variable{ name: "a".to_string(), width: 8, subscript: Some(1) })), assignee: Lvalue::Variable{ name: "a".to_string(), width: 8, subscript: Some(3) }});
        let mne1 = Mnemonic::new(0..10,"op1".to_string(),"{8:-:eax} nog".to_string(),ops1.iter(),i1.iter());

        let ops2 = vec!(Rvalue::Constant(1),Rvalue::Variable{ name: "a".to_string(), width: 3, subscript: None });
        let i2 = vec!(
            Instr{ op: Operation::IntAdd(Rvalue::Constant(1),Rvalue::Constant(2)), assignee: Lvalue::Variable{ name: "a".to_string(), width: 8, subscript: Some(2) }},
            Instr{ op: Operation::IntAdd(Rvalue::Constant(4),Rvalue::Constant(2)), assignee: Lvalue::Variable{ name: "a".to_string(), width: 8, subscript: Some(1) }},
            Instr{ op: Operation::Phi(vec!(
                Rvalue::Variable{ name: "a".to_string(), width: 8, subscript: Some(2) },
                Rvalue::Variable{ name: "a".to_string(), width: 8, subscript: Some(1) })), assignee: Lvalue::Variable{ name: "a".to_string(), width: 8, subscript: Some(3) }});
        let mne2 = Mnemonic::new(10..13,"op3".to_string(),"{8:-:eax} nog".to_string(),ops2.iter(),i2.iter());

        let ms = vec!(mne1,mne2);
        let mut bb = BasicBlock::from_vec(ms);

        bb.rewrite(|i| match &i.assignee {
            &Lvalue::Variable{ name: ref n, width: ref w, subscript: _ } =>
                Instr{ op: i.op.clone(), assignee: Lvalue::Variable{ name: n.clone(), width: *w, subscript: None } },
            _ => i.clone()
        });

        let mut ok = true;

        bb.execute(|i| match i.assignee {
            Lvalue::Variable{ subscript: None,.. } => ok &= true,
            _ => ok = false
        });

        assert!(ok);
    }
}

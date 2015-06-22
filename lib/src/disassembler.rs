use value::Rvalue;
use mnemonic::{Mnemonic,Bound};
use guard::Guard;
use std::rc::Rc;
use num::traits::*;
use std::fmt::Debug;
use std::slice::Iter;
use std::ops::{BitOr,Shl,Not};
use std::collections::HashMap;
use std::mem::size_of;
use codegen::CodeGen;

pub trait Token: Clone + Zero + One + Debug + Not + BitOr + Shl<usize> + NumCast
where <Self as Not>::Output: NumCast,
      <Self as BitOr>::Output: NumCast,
      <Self as Shl<usize>>::Output: NumCast
{}

impl Token for u8 {}

pub type Action<I: Token> = fn(&State<I>) -> bool;

#[derive(Debug)]
pub struct State<I: Clone> {
    // in
    pub address: u64,
    pub tokens: Vec<I>,
    pub groups: Vec<(String,I)>,

    // out
    pub mnemonics: Vec<Mnemonic>,
    pub jumps: Vec<(Rvalue,Guard)>,

    next_address: u64,
}

impl<I: Clone> State<I> {
    pub fn new(a: u64, t: Vec<I>, g: Vec<(String,I)>) -> State<I> {
        State{
            address: a,
            tokens: t,
            groups: g,
            mnemonics: Vec::new(),
            jumps: Vec::new(),
            next_address: a,
        }
    }

    pub fn mnemonic<F: Fn(&CodeGen) -> ()>(&mut self,len: usize, n: &str, fmt: &str, ops: Vec<Rvalue>, f: F) {
        self.mnemonic_dynargs(len,n,fmt,|cg: &CodeGen| -> Vec<Rvalue> {
            f(cg);
            ops.clone()
        });
    }

    pub fn mnemonic_dynargs<F>(&mut self,len: usize, n: &str, fmt: &str, f: F)
    where F: Fn(&CodeGen) -> Vec<Rvalue> {
        let mut cg = CodeGen::new();
        let ops = f(&cg);

        self.mnemonics.push(Mnemonic::new(
                self.next_address..(self.next_address + (len as u64)),
                n.to_string(),
                fmt.to_string(),
                ops.iter(),
                cg.instructions.iter()));
        self.next_address += len as u64;
    }

    pub fn jump(&mut self,v: Rvalue,g: Guard) {
        self.jumps.push((v,g));
    }
}

#[derive(Clone)]
pub struct Match<I: Token> {
    patterns: Vec<(I,I)>,
    actions: Vec<Rc<Action<I>>>,
    groups: Vec<(String,Vec<I>)>
}

pub enum Expr<I: Token> {
    Pattern(String),
    Terminal(I),
    Subdecoder(Rc<Disassembler<I>>)
}

pub trait ToExpr<I: Token> {
    fn to_expr(&self) -> Expr<I>;
}

impl<'a,I: Token> ToExpr<I> for &'a str {
    fn to_expr(&self) -> Expr<I> {
        Expr::Pattern(self.to_string())
    }
}

impl<'a,I: Token> ToExpr<I> for Rc<Disassembler<I>> {
    fn to_expr(&self) -> Expr<I> {
        Expr::Subdecoder(self.clone())
    }
}

impl<I: Token> ToExpr<I> for usize {
    fn to_expr(&self) -> Expr<I> {
        Expr::Terminal(I::from::<usize>(*self).unwrap().clone())
    }
}

impl<I: Token> Expr<I> {
    pub fn matches(&self) -> Vec<Match<I>>
    where <I as Not>::Output: NumCast,
          <I as BitOr>::Output: NumCast,
          <I as Shl<usize>>::Output: NumCast
    {
        let mut pats = Vec::<(I,I)>::new();
        let mut grps = HashMap::<String,Vec<I>>::new();

        match self {
            &Expr::Pattern(ref s) => {
                let mut groups = HashMap::<String,I>::new();
                let mut cur_group = "".to_string();
                let mut read_pat = true; // false while reading torwards @
                let mut bit = size_of::<I>() * 8;
                let mut invmask = I::zero();
                let mut pat = I::zero();

                for c in s.chars() {
                    match c {
                        '@' => {
                            if read_pat {
                                error!("Pattern syntax error: read '@' w/o name in '{}'",s);
                                return Vec::new();
                            } else {
                                read_pat = true;

                                if cur_group == "" {
                                    error!("Pattern syntax error: anonymous groups not allowed in '{}'",s);
                                    return Vec::new();
                                }

                                groups.insert(cur_group.clone(),I::zero());
                            }
                        },
                        ' ' => (),
                        '.' => {
                            if read_pat {
                                invmask = cast(invmask | cast(I::one() << (bit - 1)).unwrap()).unwrap();
                                bit -= 1;
                            } else {
                                error!("Pattern syntax error: read '.' while expecting '@' in '{}'",s);
                                return Vec::new();
                            }
                        },
                        '0'...'1' => {
                            if read_pat {
                                if c == '1' {
                                    pat = cast(pat | cast(I::one() << (bit - 1)).unwrap()).unwrap();
                                }

                                if cur_group != "" {
                                    *groups.get_mut(&cur_group).unwrap() = cast(groups.get(&cur_group).unwrap().clone() | cast(I::one() << (bit - 1)).unwrap()).unwrap();
                                }

                                bit -= 1;
                            } else {
                                error!("Pattern syntax error: pattern start without '@' delimiter in '{}'",s);
                                return Vec::new();
                            }
                        },
                        'a'...'z' | 'A'...'Z' => {
                            if read_pat {
                                cur_group = c.to_string();
                                read_pat = false;
                            } else {
                                cur_group.push(c);
                            }
                        },
                        _ => {
                            error!("Pattern syntax error: invalid character '{}' in '{}'",c,s);
                            return Vec::new();
                        }
                    }
                }

                if bit != 0 {
                    error!("Pattern syntax error: invalid pattern length");
                    return Vec::new();
                }

                pats.push((pat,cast(!invmask).unwrap()));

                for g in groups {
                    if grps.contains_key(&g.0) {
                        grps.get_mut(&g.0).unwrap().push(g.1)
                    } else {
                        grps.insert(g.0,vec!(g.1));
                    }
                }
            },
            &Expr::Terminal(ref i) => pats.push((i.clone(),cast(!I::zero()).unwrap())),
            &Expr::Subdecoder(ref m) => return m.matches.clone(),
        }

        vec!(Match::<I>{
            patterns: pats,
            groups: grps.iter().map(|x| (x.0.clone(),x.1.clone())).collect(),
            actions: vec!()
        })
    }
}

pub struct Disassembler<I: Token> {
    matches: Vec<Match<I>>
}

impl<I: Token> Disassembler<I> {
    pub fn new() -> Disassembler<I> {
        Disassembler::<I> {
            matches: Vec::new()
        }
    }

    fn combine_expr(mut i: Iter<Expr<I>>, a: Action<I>) -> Vec<Match<I>>
    where <I as Not>::Output: NumCast,
          <I as BitOr>::Output: NumCast,
          <I as Shl<usize>>::Output: NumCast
    {
        match i.next() {
            Some(e) => {
                let mut rest = Self::combine_expr(i,a);
                let mut ret = Vec::new();


                for mut _match in (*e).matches() {
                    for pre in &rest {
                        for x in &pre.patterns {
                            _match.patterns.push(x.clone());
                        }

                        for x in &pre.actions {
                            _match.actions.push(x.clone());
                        }
                        for x in &pre.groups {
                            for y in _match.groups.iter_mut() {
                                if y.0 == x.0 {
                                    for p in &x.1 {
                                        y.1.push(p.clone());
                                    }
                                }
                            }
                        }
                    }

                    ret.push(Match::<I>{
                        patterns: _match.patterns,
                        actions:_match.actions,
                        groups: _match.groups
                    });
                }

                ret
            },
            None => Vec::new()
        }
    }

    pub fn add_expr(&mut self, e: Vec<Expr<I>>, a: Action<I>)
    where <I as Not>::Output: NumCast,
          <I as BitOr>::Output: NumCast,
          <I as Shl<usize>>::Output: NumCast
    {
        for x in Self::combine_expr(e.iter(),a) {
            self.matches.push(x);
        }
    }
}

macro_rules! new_disassembler {
    ($ty:ty => $( [ $( $t:expr ),+ ] = $f:expr),+) => {
        {
            let mut dis = Disassembler::<$ty>::new();

            $({
                let mut __x = Vec::new();
                $(
                    __x.push($t.to_expr());
                )+
                fn a(a: &State<$ty>) -> bool { ($f)(a) };
                let fuc: Action<$ty> = a;
                dis.add_expr(__x,fuc);
            })+

            Rc::<Disassembler<$ty>>::new(dis)
        }
    };
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;

    #[test]
    fn decode_macro() {
        let lock_prfx = new_disassembler!(u8 =>
            [ 0x06 ] = |x| { true }
        );

        let main = new_disassembler!(u8 =>
            [ 22 , 21, lock_prfx ] = |x| { true },
            [ "....11 d@00"         ] = |x| true,
            [ "....11 d@00", ".. d@0011. 0" ] = |x| true
        );
    }
}

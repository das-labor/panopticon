use value::Rvalue;
use mnemonic::Mnemonic;
use guard::Guard;
use std::rc::Rc;
use num::traits::*;

struct State<I> {
    // in
    address: u64,
    tokens: Vec<I>,
    groups: Vec<(String,I)>,

    // out
    mnemonics: Vec<Mnemonic>,
    jumps: Vec<(Rvalue,Guard)>,
}

struct Match<I> {
    patterns: Vec<(I,I)>,
    actions: Vec<Box<Fn(&State<I>) -> bool>>,
    groups: Vec<(String,Vec<I>)>
}

enum Expr<I> {
    Pattern(String),
    Terminal(I),
    Subdecoder(Rc<Disassembler<I>>)
}

trait ToExpr<I> {
    fn to_expr(&self) -> Expr<I>;
}

impl<'a,I> ToExpr<I> for &'a str {
    fn to_expr(&self) -> Expr<I> {
        Expr::Pattern(self.to_string())
    }
}

impl<'a,I> ToExpr<I> for Rc<Disassembler<I>> {
    fn to_expr(&self) -> Expr<I> {
        Expr::Subdecoder(self.clone())
    }
}

impl<I: NumCast + Clone> ToExpr<I> for usize {
    fn to_expr(&self) -> Expr<I> {
        Expr::Terminal(I::from::<usize>(*self).unwrap().clone())
    }
}

impl<I> Expr<I> {
    fn to_match() -> Vec<Match<I>> {
        unimplemented!();
    }
}

pub struct Disassembler<I> {
    matches: Vec<Match<I>>
}

impl<I> Disassembler<I> {
    fn new() -> Disassembler<I> {
        Disassembler::<I> {
            matches: Vec::new()
        }
    }

    fn add_expr<F>(&mut self, e: Vec<Expr<I>>, a: F) where F: Sized + Fn(&State<I>) -> bool{
        unimplemented!();
    }
}

macro_rules! new_disassembler {
    ($ty:ty => $( [ $( $t:expr ),+ ] = $f:expr),+) => {
        {
            let mut dis = Disassembler::<$ty>::new();

            $({
                let mut x = Vec::new();
                $(
                    x.push($t.to_expr());
                )+
                dis.add_expr(x,$f);
            })+

            Rc::<Disassembler<$ty>>::new(dis)
        }
    };
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::Expr;
    use std::rc::Rc;
    use super::ToExpr;

    #[test]
    fn add_expr() {
        let mut d = Disassembler::<u8>::new();

        d.add_expr(vec!(Expr::Pattern("11...0011".to_string())),|x| true);
    }

    #[test]
    fn decode_macro() {
        let lock_prfx = new_disassembler!(u8 =>
            [ 0x06 ] = |x| true
        );

        let main = new_disassembler!(u8 =>
            [ 22 , 21, lock_prfx ] = |x| true,
            [ "....1100"         ] = |x| true
        );
    }
}

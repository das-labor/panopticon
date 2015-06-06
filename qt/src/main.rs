extern crate panopticon;

use panopticon::guard::*;
use panopticon::value::*;

fn main() {
    let v = Guard::new(Relation::Equal(Rvalue::Constant(222),Rvalue::Undefined));

    let w = v.negation();
    println!("{:?}",w);
}

use value::{Rvalue};

#[derive(Debug,Clone,PartialEq,Eq,Hash,RustcDecodable,RustcEncodable)]
pub enum Relation {
    UnsignedLessOrEqual(Rvalue,Rvalue),
    SignedLessOrEqual(Rvalue,Rvalue),
    UnsignedGreaterOrEqual(Rvalue,Rvalue),
    SignedGreaterOrEqual(Rvalue,Rvalue),
    UnsignedLess(Rvalue,Rvalue),
    SignedLess(Rvalue,Rvalue),
    UnsignedGreater(Rvalue,Rvalue),
    SignedGreater(Rvalue,Rvalue),
    Equal(Rvalue,Rvalue),
    NotEqual(Rvalue,Rvalue),
    True,
    False,
}

#[derive(Clone,Debug,PartialEq,RustcDecodable,RustcEncodable)]
pub struct Guard {
    relation: Relation,
}

impl Guard {
    pub fn from_relation(r: Relation) -> Guard {
        Guard{ relation: r }
    }

    pub fn new() -> Guard {
        Guard{ relation: Relation::True }
    }

    pub fn negation(&self) -> Guard {
        Guard::from_relation(match self.relation {
            Relation::UnsignedLessOrEqual(ref a,ref b) => Relation::UnsignedGreater(a.clone(),b.clone()),
            Relation::SignedLessOrEqual(ref a,ref b) => Relation::SignedGreater(a.clone(),b.clone()),
            Relation::UnsignedGreaterOrEqual(ref a,ref b) => Relation::UnsignedLess(a.clone(),b.clone()),
            Relation::SignedGreaterOrEqual(ref a,ref b) => Relation::SignedLess(a.clone(),b.clone()),
            Relation::UnsignedLess(ref a,ref b) => Relation::UnsignedGreaterOrEqual(a.clone(),b.clone()),
            Relation::SignedLess(ref a,ref b) => Relation::SignedGreaterOrEqual(a.clone(),b.clone()),
            Relation::UnsignedGreater(ref a,ref b) => Relation::UnsignedLessOrEqual(a.clone(),b.clone()),
            Relation::SignedGreater(ref a,ref b) => Relation::SignedLessOrEqual(a.clone(),b.clone()),
            Relation::Equal(ref a,ref b) => Relation::NotEqual(a.clone(),b.clone()),
            Relation::NotEqual(ref a,ref b) => Relation::Equal(a.clone(),b.clone()),
            Relation::True => Relation::False,
            Relation::False => Relation::True,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use value::{Rvalue};

    #[test]
    fn construct() {
        let g = Guard::from_relation(Relation::UnsignedGreater(Rvalue::Undefined,Rvalue::Undefined));
        let g2 = Guard::from_relation(Relation::Equal(Rvalue::Undefined,Rvalue::Undefined));

        assert!(g != g2);
    }

    #[test]
    fn negation() {
        let g1 = Guard::from_relation(Relation::UnsignedLessOrEqual(Rvalue::Undefined,Rvalue::Undefined));
        let g2 = Guard::from_relation(Relation::SignedLessOrEqual(Rvalue::Undefined,Rvalue::Undefined));
        let g3 = Guard::from_relation(Relation::UnsignedGreaterOrEqual(Rvalue::Undefined,Rvalue::Undefined));
        let g4 = Guard::from_relation(Relation::SignedGreaterOrEqual(Rvalue::Undefined,Rvalue::Undefined));
        let g5 = Guard::from_relation(Relation::UnsignedLess(Rvalue::Undefined,Rvalue::Undefined));
        let g6 = Guard::from_relation(Relation::SignedLess(Rvalue::Undefined,Rvalue::Undefined));
        let g7 = Guard::from_relation(Relation::UnsignedGreater(Rvalue::Undefined,Rvalue::Undefined));
        let g8 = Guard::from_relation(Relation::SignedGreater(Rvalue::Undefined,Rvalue::Undefined));
        let g9 = Guard::from_relation(Relation::Equal(Rvalue::Undefined,Rvalue::Undefined));
        let g10 = Guard::from_relation(Relation::NotEqual(Rvalue::Undefined,Rvalue::Undefined));
        let g11 = Guard::from_relation(Relation::True);
        let g12 = Guard::from_relation(Relation::False);

        let not_g1 = g1.negation();
        let not_g2 = g2.negation();
        let not_g3 = g3.negation();
        let not_g4 = g4.negation();
        let not_g5 = g5.negation();
        let not_g6 = g6.negation();
        let not_g7 = g7.negation();
        let not_g8 = g8.negation();
        let not_g9 = g9.negation();
        let not_g10 = g10.negation();
        let not_g11 = g11.negation();
        let not_g12 = g12.negation();

        assert!(g1 != not_g1);
        assert!(g2 != not_g2);
        assert!(g3 != not_g3);
        assert!(g4 != not_g4);
        assert!(g5 != not_g5);
        assert!(g6 != not_g6);
        assert!(g7 != not_g7);
        assert!(g8 != not_g8);
        assert!(g9 != not_g9);
        assert!(g10 != not_g10);
        assert!(g11 != not_g11);
        assert!(g12 != not_g12);

        assert_eq!(g1,not_g1.negation());
        assert_eq!(g2,not_g2.negation());
        assert_eq!(g3,not_g3.negation());
        assert_eq!(g4,not_g4.negation());
        assert_eq!(g5,not_g5.negation());
        assert_eq!(g6,not_g6.negation());
        assert_eq!(g7,not_g7.negation());
        assert_eq!(g8,not_g8.negation());
        assert_eq!(g9,not_g9.negation());
        assert_eq!(g10,not_g10.negation());
        assert_eq!(g11,not_g11.negation());
        assert_eq!(g12,not_g12.negation());
    }
}

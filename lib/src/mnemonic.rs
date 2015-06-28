use std::str::{Chars,FromStr};
use value::Rvalue;
use instr::Instr;
use std::ops::Range;

#[derive(Debug,Clone,PartialEq,Eq,RustcEncodable,RustcDecodable)]
pub struct Bound {
    pub start: u64,
    pub end: u64
}

impl Bound {
    pub fn new(a: u64, b: u64) -> Bound {
        Bound{ start: a, end: b }
    }
}

#[derive(Clone,Debug,PartialEq,Eq,RustcEncodable,RustcDecodable)]
pub enum MnemonicFormatToken {
    Literal(char),
    Variable{ has_sign: bool, width: u16, alias: Option<String> },
}

impl MnemonicFormatToken {
    fn parse_modifiers<'a>(i: &Chars<'a>,w: &u16) -> Option<(Chars<'a>,MnemonicFormatToken)> {
        let mut j = i.clone();
        let mut q = j.clone().peekable();
        let maybe_s = q.peek();
        let s = maybe_s.is_some() && maybe_s.unwrap() == &'-';

        if s {
            j.next();
        }

        match j.next() {
            Some(':') => Self::parse_alias(&j,&w,&s),
            Some('}') => Some((j,MnemonicFormatToken::Variable{ has_sign: s, width: *w, alias: None })),
            _ => None,
        }
    }

    fn parse_alias<'a>(i: &Chars<'a>,w: &u16, s: &bool) -> Option<(Chars<'a>,MnemonicFormatToken)> {
        let mut j = i.clone();

        match j.position(|x| x == '}') {
            Some(0) => Some((j,MnemonicFormatToken::Variable{ has_sign: *s, width: *w, alias: None })),
            Some(p) => {
                let a = i.clone().take(p).collect::<String>();
                Some((j,MnemonicFormatToken::Variable{ has_sign: *s, width: *w, alias: Some(a) }))
            }
            None => None
        }
    }

    fn parse_width<'a>(i: &Chars<'a>) -> Option<(Chars<'a>,MnemonicFormatToken)> {
        let mut j = i.clone();
        let p = i.clone().position(|x| !(x >= '0' && x <= '9'));

        if !p.is_some() {
            None
        } else {
            let d = i.clone().take(p.unwrap()).collect::<String>();
            match u16::from_str(&d) {
                Ok(u) => match j.nth(p.unwrap()) {
                    Some(':') => Self::parse_modifiers(&j,&u),
                    Some('}') => Some((j,MnemonicFormatToken::Variable{ has_sign: false, width: u, alias: None })),
                    _ => None
                },
                Err(_) => None
            }
        }
    }

    pub fn parse(i: &Chars) -> Vec<MnemonicFormatToken> {
        let mut j = i.clone();

        match j.next() {
            None => Vec::new(),
            Some(a) => {
                let p = if a == '{' {
                    match Self::parse_width(&j) {
                        Some((k,tok)) => {
                            j = k;
                            Some(tok)
                        },
                        None => return Vec::new()
                    }
                } else {
                    Some(MnemonicFormatToken::Literal(a))
                };

                let mut ret = Self::parse(&j);

                if let Some(x) = p {
                    ret.insert(0,x);
                }
                ret
            },
        }
    }
}

#[derive(Clone,PartialEq,Eq,Debug,RustcEncodable,RustcDecodable)]
pub struct Mnemonic {
    pub area: Bound,
    pub opcode: String,
    pub operands: Vec<Rvalue>,
    pub instructions: Vec<Instr>,
    pub format_string: Vec<MnemonicFormatToken>,
}

impl Mnemonic {
    pub fn new<'a,I1,I2> (a: Range<u64>, code: String, fmt: String, ops: I1, instr: I2) -> Mnemonic
        where I1: Iterator<Item=&'a Rvalue>,I2: Iterator<Item=&'a Instr> {
        Mnemonic{
            area: Bound::new(a.start,a.end),
            opcode: code,
            operands: ops.cloned().collect(),
            instructions: instr.cloned().collect(),
            format_string: MnemonicFormatToken::parse(&fmt.chars()),
        }
    }

    #[cfg(test)]
    pub fn dummy(a: Range<u64>) -> Mnemonic {
        Mnemonic{
            area: Bound::new(a.start,a.end),
            opcode: "dummy".to_string(),
            operands: vec!(),
            instructions: vec!(),
            format_string: vec!(),
        }
    }

    pub fn format(&self) -> String {
        self.format_string.iter().fold("".to_string(),|acc,x| -> String {
            let t: String = match x {
                &MnemonicFormatToken::Literal(ref s) => s.to_string(),
                &MnemonicFormatToken::Variable{ has_sign: b, width: w, alias: ref a } =>
                    format!("{{{}:{}:{}}}",w,if b { "-".to_string() } else { "".to_string() },a.clone().unwrap_or("".to_string()))
            };
            acc + &t
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use value::{Rvalue,Lvalue};
    use instr::{Operation,Instr};
    use msgpack;

    #[test]
    fn parse_format_string() {
        let fmt = "doe{69::eax3}io{8:-}øiq{88:-:sss}   {9::} sasq {32:}".to_string();
        let val = MnemonicFormatToken::parse(&fmt.chars());

        assert_eq!(vec!(
                MnemonicFormatToken::Literal('d'),
                MnemonicFormatToken::Literal('o'),
                MnemonicFormatToken::Literal('e'),
                MnemonicFormatToken::Variable{ has_sign: false, width: 69, alias: Some("eax3".to_string()) },
                MnemonicFormatToken::Literal('i'),
                MnemonicFormatToken::Literal('o'),
                MnemonicFormatToken::Variable{ has_sign: true, width: 8, alias: None },
                MnemonicFormatToken::Literal('ø'),
                MnemonicFormatToken::Literal('i'),
                MnemonicFormatToken::Literal('q'),
                MnemonicFormatToken::Variable{ has_sign: true, width: 88, alias: Some("sss".to_string()) },
                MnemonicFormatToken::Literal(' '),
                MnemonicFormatToken::Literal(' '),
                MnemonicFormatToken::Literal(' '),
                MnemonicFormatToken::Variable{ has_sign: false, width: 9, alias: None },
                MnemonicFormatToken::Literal(' '),
                MnemonicFormatToken::Literal('s'),
                MnemonicFormatToken::Literal('a'),
                MnemonicFormatToken::Literal('s'),
                MnemonicFormatToken::Literal('q'),
                MnemonicFormatToken::Literal(' '),
                MnemonicFormatToken::Variable{ has_sign: false, width: 32, alias: None }
            ),val);

        assert_eq!(MnemonicFormatToken::parse(&"{69:+}".to_string().chars()),Vec::new());
        assert_eq!(MnemonicFormatToken::parse(&"{-69:+}".to_string().chars()),Vec::new());
        assert_eq!(MnemonicFormatToken::parse(&"{69::".to_string().chars()),Vec::new());
        assert_eq!(MnemonicFormatToken::parse(&"{}".to_string().chars()),Vec::new());
        assert_eq!(MnemonicFormatToken::parse(&"{".to_string().chars()),Vec::new());
        assert_eq!(MnemonicFormatToken::parse(&"{69".to_string().chars()),Vec::new());
        assert_eq!(MnemonicFormatToken::parse(&"{69:".to_string().chars()),Vec::new());
        assert_eq!(MnemonicFormatToken::parse(&"{69:-".to_string().chars()),Vec::new());
        assert_eq!(MnemonicFormatToken::parse(&"{69::".to_string().chars()),Vec::new());
        assert_eq!(MnemonicFormatToken::parse(&"{69:-:".to_string().chars()),Vec::new());
        assert_eq!(MnemonicFormatToken::parse(&"{69::ddd".to_string().chars()),Vec::new());
        assert_eq!(MnemonicFormatToken::parse(&"{69}".to_string().chars()),vec!(MnemonicFormatToken::Variable{ has_sign: false, width: 69, alias: None }));
    }

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

        assert_eq!(mne1.format(),"{8:-:eax} nog".to_string());
        assert_eq!(mne1.area, Bound::new(0,10));
        assert_eq!(mne1.opcode, "op1");
        assert_eq!(mne1.operands, ops1);
        assert_eq!(mne1.instructions, i1);
    }

    #[test]
    fn marshal() {
        let ops1 = vec!(Rvalue::Constant(1),Rvalue::Variable{ name: "a".to_string(), width: 3, subscript: None });
        let i1 = vec!(
            Instr{ op: Operation::IntAdd(Rvalue::Constant(1),Rvalue::Constant(2)), assignee: Lvalue::Variable{ name: "a".to_string(), width: 8, subscript: Some(2) }},
            Instr{ op: Operation::IntAdd(Rvalue::Constant(4),Rvalue::Constant(2)), assignee: Lvalue::Variable{ name: "a".to_string(), width: 8, subscript: Some(1) }},
            Instr{ op: Operation::Phi(vec!(
                Rvalue::Variable{ name: "a".to_string(), width: 8, subscript: Some(2) },
                Rvalue::Variable{ name: "a".to_string(), width: 8, subscript: Some(1) })), assignee: Lvalue::Variable{ name: "a".to_string(), width: 8, subscript: Some(3) }});
        let mne1 = Mnemonic::new(0..10,"op1".to_string(),"{8:-:eax} nog".to_string(),ops1.iter(),i1.iter());

        let a = msgpack::Encoder::to_msgpack(&mne1).ok().unwrap();
        println!("{:?}", a);
        let mne2 = msgpack::from_msgpack(&a).ok().unwrap();

        assert_eq!(mne1, mne2);
    }
}

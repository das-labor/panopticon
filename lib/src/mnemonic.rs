use std::str::{Chars,FromStr};
use std::ops::Range;
use std::collections::HashSet;

use value::Rvalue;
use instr::Instr;
use rdf::*;
use marshal::{Marshal,Archive,Blob};
use uuid::Uuid;


#[derive(Debug,PartialEq,Eq)]
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

#[derive(PartialEq,Eq,Debug)]
pub struct Mnemonic {
    pub area: Range<u64>,
    pub opcode: String,
    pub operands: Vec<Rvalue>,
    pub instructions: Vec<Instr>,
    pub format_string: Vec<MnemonicFormatToken>,
}

impl Mnemonic {
    pub fn new<'a,I1,I2> (a: Range<u64>, code: String, fmt: String, ops: I1, instr: I2) -> Mnemonic
        where I1: Iterator<Item=&'a Rvalue>,I2: Iterator<Item=&'a Instr> {
        Mnemonic{
            area: a,
            opcode: code,
            operands: ops.cloned().collect(),
            instructions: instr.cloned().collect(),
            format_string: MnemonicFormatToken::parse(&fmt.chars()),
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

/*template<>
archive po::marshal(mnemonic const& mn, const uuid& uu)
{
	size_t rv_cnt = 0;
	boost::uuids::name_generator ng(uu);
	rdf::statements ret;
	std::list<blob> bl;
	std::function<rdf::node(const rvalue&)> map_rvs = [&](const rvalue &rv)
	{
		uuid u = ng(to_string(rv_cnt++));
		rdf::node r = rdf::iri(u);
		auto st = marshal(rv,u);

		ensure(st.triples.size());
		std::move(st.triples.begin(),st.triples.end(),back_inserter(ret));
		std::move(st.blobs.begin(),st.blobs.end(),back_inserter(bl));
		return r;
	};
	rdf::node r = rdf::iri(uu);

	ret.emplace_back(r,rdf::ns_po("opcode"),rdf::lit(mn.opcode));
	ret.emplace_back(r,rdf::ns_po("format"),rdf::lit(mn.format_string));
	ret.emplace_back(r,rdf::ns_po("begin"),rdf::lit(mn.area.lower()));
	ret.emplace_back(r,rdf::ns_po("end"),rdf::lit(mn.area.upper()));

	rdf::nodes n_ops, n_ex;

	std::transform(mn.operands.begin(),mn.operands.end(),back_inserter(n_ops),map_rvs);

	std::transform(mn.instructions.begin(),mn.instructions.end(),back_inserter(n_ex),[&](const instr& i)
	{
		uuid u = ng(to_string(rv_cnt++));
		rdf::node r = rdf::iri(u), rl = rdf::node::blank(), rr = rdf::node::blank();
		rdf::statements rs;

		rl = map_rvs(i.assignee);

		rdf::nodes rn;
		std::vector<rvalue> right = operands(i);
		std::transform(right.begin(),right.end(),back_inserter(rn),map_rvs);
		tie(rr,rs) = write_list(rn.begin(),rn.end(),u);
		std::move(rs.begin(),rs.end(),back_inserter(ret));

		ret.emplace_back(r,rdf::ns_po("function"),rdf::iri(symbolic(i.function)));
		ret.emplace_back(r,rdf::ns_po("left"),rl);
		ret.emplace_back(r,rdf::ns_po("right"),rr);

		return r;
	});

	auto p_ops = write_list(n_ops.begin(),n_ops.end(),ng(to_string(uu) + "-operands"));
	auto p_ex = write_list(n_ex.begin(),n_ex.end(),ng(to_string(uu) + "-instrs"));

	std::move(p_ops.second.begin(),p_ops.second.end(),back_inserter(ret));
	std::move(p_ex.second.begin(),p_ex.second.end(),back_inserter(ret));

	ret.emplace_back(r,rdf::ns_po("operands"),p_ops.first);
	ret.emplace_back(r,rdf::ns_po("executes"),p_ex.first);

	return archive(ret,bl);
}*/


impl Marshal for Mnemonic {
    fn marshal(&self, r: &Node) -> Archive {
        let mut ret = Archive{ statements: HashSet::new(), blobs: HashSet::new() };
        let mut cnt: u64 = 0;
        let fold_rv = |acc: (Archive, Vec<Node>),x: &Rvalue| -> (Archive,Vec<Node>) {
            let rv = Node::from_ns(r,format!("{}",cnt).as_bytes());
            let a = x.marshal(&rv);

            cnt += 1;
            (Archive{
                statements: acc.0.statements.union(&a.statements).cloned().collect(),
                blobs: acc.0.blobs.union(&a.blobs).cloned().collect()
            },vec!(rv))
        };
        /*let fold_i = |acc: Archive,x: &Instr| -> Archive {
            let i = Node::from_ns(r,format!("{}",cnt).as_bytes());
            let mut ret = Archive{ statements: HashSet::new(), blobs: HashSet::new() };

            let left = fold_rv((ret,Vec::new()),x.assignee);
            let right = x.operands().iter().fold((left.0,Vec::new()),fold_rv);

            let rh = write_list(right.1,i);

            cnt += 1;
            ret.statements = right.0;
            ret.statements.insert(Statement::new(i.clone(),Node::ns_po("function"),Node::Iri(x.symbolic())));
            ret.statements.insert(Statement::new(i.clone(),Node::ns_po("left"),left.1.first()));
            ret.statements.insert(Statement::new(i.clone(),Node::ns_po("right"),rh));

            ret
        };*/

        ret.statements.insert(Statement::new(r.clone(),Node::ns_po("opcode"),Node::lit(&self.opcode)));
        ret.statements.insert(Statement::new(r.clone(),Node::ns_po("format"),Node::lit(&self.format())));
        ret.statements.insert(Statement::new(r.clone(),Node::ns_po("begin"),Node::unsigned(self.area.start)));
        ret.statements.insert(Statement::new(r.clone(),Node::ns_po("end"),Node::unsigned(self.area.end)));

        ret = self.operands.iter().fold((ret,Vec::<Node>::new()),fold_rv).0;
        //ret = self.instructions.iter().fold(ret,fold_i);

        ret
    }

    fn unmarshal(a: &Archive) -> Mnemonic {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use value::{Rvalue,Lvalue};
    use instr::{Operation,Instr};
    use uuid::Uuid;
    use marshal::Marshal;
    use rdf::Node;

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
        assert_eq!(mne1.area, 0..10);
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

        let a = mne1.marshal(&Node::from_uuid(&Uuid::new_v4()));
        let mne2 = Mnemonic::unmarshal(&a);

        assert_eq!(mne1, mne2);
    }
}

/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2014,2015,2016  Panopticon authors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

//! A disassembler in Panopticon is responsible to translate a sequence of tokens
//! into mnemonics.
//!
//! A token is a fixed width byte sequence. The width depends on the
//! instruction set architection and is the shortest possible machine code
//! instruction (on IA32 this would be 1 byte, on ARM 4 bytes). A mnemonic includes
//! the syntax of the machine code instruction, its semantics in RREIL and a collection
//! of locations where the CPU will look for the next instruction to execute. For each
//! supported instruction set architecture a seperate disassembler needs to be
//! implemented.
//!
//! Implementer can either built their own diassembler or use the `Disassembler` type.
//! The type parameter identifies the instruction set. When machine code
//! needs to be disassembled, a new instance of `Disassembler` is allocated and its
//! :try_match()` method is repeantly called. Each call returns either a set of mnemonics
//! and a set of new locations of an error. Disassembly is finished when no new locations
//! are left.
//!
//! The `Disassembler` type provides fuctions to make disassembly easier.
//! The programmer only need to write one decode function for each instruction in
//! the instruction set. This decode function translates the byte representation
//! into one or more mnemonic instances with instruction name, operand count and
//! instruction semantics expressed as a RREIL instruction sequence. Each decode
//! functions is paired with a token pattern. The disassembler instance will look
//! for this pattern and call the decode function for each match. The menmonic
//! instances allocated in the decode function are assembled into a program.
//!
//! Token Patterns
//! --------------
//!
//! The token pattern is a string that defines sequence on bits to look for. Each
//! bit in a pattern is either ``0``, ``1`` or ``.`` when we accept both. The pattern
//! ``10001001`` matches the byte ``0x89``, the pattern ``11.100.0`` matches ``0xd0``
//! (``11010000``), ``0xd2`` (``11010010``), ``0xf0`` (``11110000``) and ``0xf2`` (``11110010``). Pattern must
//! have one pattern character for each bit in the token. Patters allow named groups
//! of bits so called capture groups. These start with a character except ``0``, ``1``,
//! ``.`` and `` `` (space), followed by a ``@``, followed by a pattern. The capture group
//! extend until the next space character or the end of the pattern string. The
//! pattern ``10 a@110 011`` has the capture group named `a` that is always equal to
//! ``0x6`` (``110``). The pattern ``001 a@.....`` matches all tokens larger than or equal to
//! ``0x20``, the least significant 5 bits form the capture group `a`. When a pattern is
//! paired with a decode function in the disassembler the function receives the
//! contents of capture groups a an argument.
//!
//! Example
//! -------
//!
//! An example pair of decoder function token pattern for the AVR ``pop`` instruction
//! looks like this:
//!
//! ```
//! #[macro_use] extern crate panopticon_core;
//! # extern crate panopticon_avr;
//! # use panopticon_core::{Rvalue,State,Statement,Result};
//! # use panopticon_avr::Avr;
//! # fn main() {
//! let main = new_disassembler!(Avr =>
//!    ["1001000 d@..... 1111"] = |st: &mut State<Avr>| {
//!       let val = Rvalue::Variable{ name: format!("R{}",st.get_group("d")).into(),
//!                                   size: 8, offset: 0, subscript: None };
//!       st.mnemonic(2,"pop","{u}",vec![val],&|_| -> Result<Vec<Statement>> { Ok(vec![]) });
//!       true
//!    }
//! );
//! # }
//! ```

#![macro_use]


use {Guard, Mnemonic, Region, Result, Rvalue, Statement};

use num::traits::{NumCast, One, Zero};
use panopticon_graph_algos::{AdjacencyList, EdgeListGraphTrait, GraphTrait, IncidenceGraphTrait, MutableGraphTrait, VertexListGraphTrait};
use panopticon_graph_algos::adjacency_list::AdjacencyListVertexDescriptor;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;
use std::mem::size_of;
use std::ops::{BitAnd, BitOr, Not, Shl, Shr};
use std::sync::Arc;

/// CPU architecture and instruction set.
pub trait Architecture: Clone {
    /// Unsigned integer type. This is tells [`Disassembler`] whenever mnemonics are read byte or
    /// word wise.
    type Token: Not<Output = Self::Token> + Clone + Zero + One + Debug + NumCast + BitOr<Output = Self::Token> + BitAnd<Output = Self::Token> + Shl<usize, Output = Self::Token> + Shr<usize, Output = Self::Token> + PartialEq + Eq + Send + Sync;

    /// This type can describes the CPU state. For x86 this would be the mode, for ARM whenever
    /// Thumb is active.
    type Configuration: Clone + Send;

    /// Given a memory image and a configuration the functions extracts a set of entry points.
    /// # Return
    /// Tuples of entry point name, offset form the start of the region and optional comment.
    fn prepare(&Region, &Self::Configuration) -> Result<Vec<(&'static str, u64, &'static str)>>;

    /// Start to disassemble a single Opcode inside a given region at a given address.
    fn decode(&Region, u64, &Self::Configuration) -> Result<Match<Self>>;
}

/// Result of a single disassembly operation.
#[derive(Debug,Clone)]
pub struct Match<A: Architecture> {
    /// Matched tokens
    pub tokens: Vec<A::Token>,
    /// Recognized mnemonics
    pub mnemonics: Vec<Mnemonic>,
    /// Jumps/branches originating from the recovered mnemonics
    pub jumps: Vec<(u64, Rvalue, Guard)>,

    /// New CPU state
    pub configuration: A::Configuration,
}

impl<A: Architecture> From<State<A>> for Match<A> {
    fn from(st: State<A>) -> Self {
        Match::<A> {
            tokens: st.tokens,
            mnemonics: st.mnemonics,
            jumps: st.jumps,
            configuration: st.configuration,
        }
    }
}

/// Semantic action function type. See [`Disassembler`].
pub type Action<A> = fn(&mut State<A>) -> bool;

/// Disassembler state. This struct passes data about matched tokes from the Disassembler to the
/// semantic function. The function also uses the type to pass back recognized mnemonics and jumps.
/// See [`Disassembler`].
#[derive(Debug,Clone)]
pub struct State<A: Architecture> {
    // in
    /// Start of the token sequence
    pub address: u64,
    /// Matched tokens
    pub tokens: Vec<A::Token>,
    /// Extracted capture groups
    pub groups: Vec<(String, u64)>,

    // out
    /// Mnemonics recognized in the token sequence
    pub mnemonics: Vec<Mnemonic>,
    /// Jumps/branches originating from the recognized mnemonics
    pub jumps: Vec<(u64, Rvalue, Guard)>,

    mnemonic_origin: u64,
    jump_origin: u64,

    /// Current CPU state
    pub configuration: A::Configuration,
}

impl<A: Architecture> State<A> {
    /// Create a new `State` for a token sequence starting at `a`.
    pub fn new(a: u64, c: A::Configuration) -> State<A> {
        State {
            address: a,
            tokens: vec![],
            groups: vec![],
            mnemonics: Vec::new(),
            jumps: Vec::new(),
            mnemonic_origin: a,
            jump_origin: a,
            configuration: c,
        }
    }

    /// Returns the value of capture group `n`.
    /// # Panics
    /// Panics if no such capture group was defined.
    pub fn get_group(&self, n: &str) -> u64 {
        self.groups.iter().find(|x| x.0 == n.to_string()).unwrap().1.clone()
    }

    /// Returns true if capture group `n` was defined.
    pub fn has_group(&self, n: &str) -> bool {
        self.groups.iter().find(|x| x.0 == n.to_string()).is_some()
    }

    /// Append a new mnemonic.
    /// The mnemonic starts after the end of the last, or the start of the matched token sequence
    /// if it is the first. The new mnemonic `n` will be `len` *bytes* in size.
    /// Arguments for the mnemonic are given in `ops` and formatted according to `fmt`. The
    /// function `f` is called with the current CPU state and expected to return the IL statementes
    /// that implement the mnemonic.
    pub fn mnemonic<'a, F>(&mut self, len: usize, n: &str, fmt: &str, ops: Vec<Rvalue>, f: &F) -> Result<()>
    where
        F: Fn(&mut A::Configuration) -> Result<Vec<Statement>>,
    {
        self.mnemonic_dynargs(
            len,
            n,
            fmt,
            &|cfg: &mut A::Configuration| -> Result<(Vec<Rvalue>, Vec<Statement>)> {
                let stmts = f(cfg)?;
                Ok((ops.clone(), stmts))
            },
        )
    }

    /// Append a new mnemonic
    /// Same a `mnemonic` but with `f` returning the mnemonic IL and the arguments.
    pub fn mnemonic_dynargs<F>(&mut self, len: usize, n: &str, fmt: &str, f: &F) -> Result<()>
    where
        F: Fn(&mut A::Configuration) -> Result<(Vec<Rvalue>, Vec<Statement>)>,
    {
        let (ops, stmts) = f(&mut self.configuration)?;

        self.mnemonics
            .push(
                Mnemonic::new(
                    self.mnemonic_origin..(self.mnemonic_origin + (len as u64)),
                    n.to_string(),
                    fmt.to_string(),
                    ops.iter(),
                    stmts.iter(),
                )?
            );
        self.jump_origin = self.mnemonic_origin;
        self.mnemonic_origin += len as u64;

        Ok(())
    }

    /// Append a jump/branch from the end of the last mnemonic to `v`, guarded by `g`.
    pub fn jump(&mut self, v: Rvalue, g: Guard) -> Result<()> {
        if !(self.mnemonics.is_empty() || self.mnemonics.last().unwrap().area.len() > 0) {
            return Err("A basic block mustn't end w/ a zero sized mnemonic".into());
        }

        let o = self.jump_origin;
        self.jump_from(o, v, g)?;

        Ok(())
    }

    /// Append a jump/branch from `origin` to `v`, guarded by `g`.
    pub fn jump_from(&mut self, origin: u64, v: Rvalue, g: Guard) -> Result<()> {
        self.jumps.push((origin, v, g));
        Ok(())
    }
}

/// Single matching rule.
#[derive(Clone)]
pub enum Rule<A: Architecture> {
    /// Matches a fixed set of bits of a single token
    Terminal {
        /// Bit mask of all fixed bits in the pattern
        mask: A::Token,
        /// Bit pattern we are looking for
        pattern: A::Token,
        /// Pair of capture group name and bit mask
        capture_group: Vec<(String, A::Token)>,
    },
    /// Matches one of the sub-disassemblers' rules
    Sub(Arc<Disassembler<A>>),
}

impl<A: Architecture> PartialEq for Rule<A> {
    fn eq(&self, other: &Rule<A>) -> bool {
        match (self, other) {
            (&Rule::Terminal { mask: ref ma, pattern: ref pa, capture_group: ref ca },
             &Rule::Terminal { mask: ref mb, pattern: ref pb, capture_group: ref cb }) => ma == mb && pa == pb && ca == cb,
            (&Rule::Sub(ref a), &Rule::Sub(ref b)) => a.as_ref() as *const Disassembler<A> as usize == b.as_ref() as *const Disassembler<A> as usize,
            _ => false,
        }
    }
}

/// Ready made disassembler for simple instruction sets.
///
/// Disassembler instances are creates using the `new_disassembler!` macro. The resulting
/// disassembler can then be used to produce `Match`es.
pub struct Disassembler<A: Architecture> {
    graph: AdjacencyList<(), Rule<A>>,
    start: AdjacencyListVertexDescriptor,
    end: HashMap<AdjacencyListVertexDescriptor, Arc<Action<A>>>,
    default: Option<Action<A>>,
}

impl<A: Architecture> Disassembler<A> {
    /// Creates a new, empty, disassembler instance. You probably want to use `new_disassembler!`
    /// instead.
    pub fn new() -> Disassembler<A> {
        let mut g = AdjacencyList::new();
        let s = g.add_vertex(());

        Disassembler { graph: g, start: s, end: HashMap::new(), default: None }
    }
    /// Converts to a dot file; useful for debugging
    pub fn to_dot(&self) {
        println!("digraph G {{");
        for v in self.graph.vertices() {
            let lb = self.graph.vertex_label(v).unwrap();

            if self.end.contains_key(&v) {
                println!(
                    "{} [label=\"{}, prio: {:?}\",shape=doublecircle]",
                    v.0,
                    v.0,
                    lb
                );
            } else {
                println!("{} [label=\"{}, prio: {:?}\",shape=circle]", v.0, v.0, lb);
            }
        }
        for e in self.graph.edges() {
            let lb = match self.graph.edge_label(e) {
                Some(&Rule::Sub(_)) => "SUB".to_string(),
                Some(&Rule::Terminal::<A> { ref pattern, ref mask, .. }) => format!("{:?}/{:?}", pattern, mask),
                None => "".to_string(),
            };
            println!(
                "{} -> {} [label={:?}]",
                self.graph.source(e).0,
                self.graph.target(e).0,
                lb
            );
        }
        println!("}}");
    }

    /// Adds the matching rule and associated semantic action.
    /// Panics if a is empty.
    pub fn add(&mut self, a: &Vec<Rule<A>>, b: Arc<Action<A>>) {
        assert!(!a.is_empty());

        let mut v = self.start;
        for r in a.iter() {
            let mut found = false;

            for out in self.graph.out_edges(v) {
                if let Some(ref t) = self.graph.edge_label(out) {
                    if **t == *r {
                        v = self.graph.target(out);
                        found = true;
                        break;
                    }
                }
            }

            if !found {
                let tmp = self.graph.add_vertex(());
                self.graph.add_edge(r.clone(), v, tmp);
                v = tmp;
            }
        }

        self.end.insert(v, b);
    }

    /// Sets the default semantic action. This action will be called for each token that failed to
    /// match.
    pub fn set_default(&mut self, a: Action<A>) {
        self.default = Some(a);
    }

    /// Trys to match the token sequence `i`. If successful, the state after the semantic function
    /// was called is returned and None otherwise.
    pub fn next_match<Iter>(&self, i: &mut Iter, offset: u64, cfg: A::Configuration) -> Option<State<A>>
    where
        Iter: Iterator<Item = Option<u8>> + Clone,
        A::Configuration: Clone + Debug,
        A: Debug,
    {
        let mut matches = self.find(i.clone(), &State::<A>::new(offset, cfg.clone()));
        let l = matches.len();

        match l {
            0 => {
                if let Some(ref def) = self.default {
                    let mut state = State::<A>::new(offset, cfg);
                    let mut iter = i.clone();
                    if let Some(tok) = Self::read_token(&mut iter) {
                        state.tokens.push(tok);

                        if def(&mut state) {
                            return Some(state);
                        }
                    }
                }

                None
            }
            1 => Some(matches[0].clone().1),
            _ => {
                // return longest match
                matches.sort_by(|b, a| a.1.tokens.len().cmp(&b.1.tokens.len()));
                Some(matches[0].clone().1)
            }
        }
    }

    fn read_token<Iter>(i: &mut Iter) -> Option<A::Token>
    where
        Iter: Iterator<Item = Option<u8>>,
    {
        let mut tok = A::Token::zero();
        // XXX: Hardcoded to little endian for AVR. Make configurable in Architecture trait
        let cells = {
            let mut x = i.take(size_of::<A::Token>()).collect::<Vec<_>>();
            x.reverse();
            x
        };
        let mut j = cells.iter();

        for _ in 0..size_of::<A::Token>() {
            if tok != A::Token::zero() {
                tok = tok << 8;
            }
            if let Some(&Some(byte)) = j.next() {
                tok = tok | <A::Token as NumCast>::from(byte).unwrap();
            } else {
                return None;
            }
        }

        Some(tok)
    }

    fn find<Iter>(&self, i: Iter, initial_state: &State<A>) -> Vec<(Vec<&()>, State<A>, Iter)>
    where
        Iter: Iterator<Item = Option<u8>> + Clone,
        A::Configuration: Clone,
    {
        let mut states = Vec::<(Vec<&()>, State<A>, AdjacencyListVertexDescriptor, Iter)>::new();
        let mut ret = vec![];

        states.push((vec![], initial_state.clone(), self.start, i.clone()));
        while !states.is_empty() {
            for &(ref pats, ref state, ref v, ref iter) in states.iter() {
                if let Some(act) = self.end.get(v) {
                    let mut st = state.clone();
                    if act(&mut st) {
                        ret.push((pats.clone(), st, iter.clone()));
                    }
                }
            }

            let mut new_states = Vec::<(Vec<&()>, State<A>, AdjacencyListVertexDescriptor, Iter)>::new();


            for &(ref pats, ref state, ref vx, ref iter) in states.iter() {
                if let Some(a) = self.graph.vertex_label(*vx) {
                    for e in self.graph.out_edges(*vx) {
                        match self.graph.edge_label(e) {
                            Some(&Rule::Terminal { ref mask, ref pattern, capture_group: ref capture }) => {
                                let mut i = iter.clone();
                                if let Some(tok) = Self::read_token(&mut i) {
                                    if mask.clone() & tok.clone() == *pattern {
                                        let mut p = pats.clone();
                                        let mut st = state.clone();

                                        // capture group
                                        for &(ref name, ref mask) in capture.iter() {
                                            let mut res = if let Some(p) = st.groups.iter().position(|x| x.0 == *name) {
                                                st.groups[p].1
                                            } else {
                                                0u64
                                            };

                                            for rbit in 0..(size_of::<A::Token>() * 8) {
                                                let bit = (size_of::<A::Token>() * 8) - rbit - 1;
                                                let bit_mask = if bit > 0 {
                                                    A::Token::one() << bit
                                                } else {
                                                    A::Token::one()
                                                };

                                                let a = bit_mask.clone() & mask.clone();

                                                if a != A::Token::zero() {
                                                    res <<= 1;

                                                    if tok.clone() & a != A::Token::zero() {
                                                        res |= 1;
                                                    }
                                                }
                                            }

                                            if let Some(p) = st.groups.iter().position(|x| x.0 == *name) {
                                                st.groups[p].1 = res;
                                            } else {
                                                st.groups.push((name.clone(), res));
                                            }
                                        }

                                        p.push(a);
                                        st.tokens.push(tok);
                                        new_states.push((p, st, self.graph.target(e), i));
                                    }
                                }
                            }
                            Some(&Rule::Sub(ref sub)) => {
                                let i = iter.clone();
                                let mut v = sub.find(i.clone(), state);

                                new_states.extend(v.drain(..).map(|(a, b, i)| (a, b, self.graph.target(e), i.clone())));
                            }
                            None => {}
                        };
                    }
                }
            }

            states = new_states;
        }

        ret
    }
}

impl<A: Architecture> Debug for Disassembler<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Disassembler")
    }
}

impl<A: Architecture> Into<Rule<A>> for usize {
    fn into(self) -> Rule<A> {
        Rule::Terminal {
            mask: !A::Token::zero(),
            pattern: <A::Token as NumCast>::from(self).unwrap(),
            capture_group: vec![],
        }
    }
}

impl<A: Architecture> From<Arc<Disassembler<A>>> for Rule<A> {
    fn from(s: Arc<Disassembler<A>>) -> Self {
        Rule::Sub(s)
    }
}

impl<'a, A: Architecture> Into<Rule<A>> for &'a str {
    fn into(self) -> Rule<A> {
        let mut groups = HashMap::<String, A::Token>::new();
        let mut cur_group = "".to_string();
        let mut read_pat = false; // false while reading torwards @
        let mut bit: isize = (size_of::<A::Token>() * 8) as isize;
        let mut mask = A::Token::zero();
        let mut pat = A::Token::zero();

        for c in self.chars() {
            match c {
                '@' => {
                    if read_pat {
                        panic!("Pattern syntax error: read '@' w/o name in '{}'", self);
                    } else {
                        read_pat = true;

                        if cur_group == "" {
                            panic!(
                                "Pattern syntax error: anonymous groups not allowed in '{}'",
                                self
                            );
                        }

                        if !groups.contains_key(&cur_group) {
                            groups.insert(cur_group.clone(), A::Token::zero());
                        }
                    }
                }
                ' ' => {
                    read_pat = false;
                    cur_group = "".to_string();
                }
                '.' => {
                    if bit <= 0 {
                        panic!("too long bit pattern: '{}'", self);
                    }

                    if read_pat && cur_group != "" {
                        *groups.get_mut(&cur_group).unwrap() = groups.get(&cur_group).unwrap().clone() | (A::Token::one() << ((bit - 1) as usize));
                    }

                    bit -= 1;
                }
                '0' | '1' => {
                    if bit <= 0 {
                        panic!("too long bit pattern: '{}'", self);
                    }

                    if bit - 1 > 0 {
                        mask = mask | (A::Token::one() << ((bit - 1) as usize));
                    } else {
                        mask = mask | A::Token::one();
                    }

                    if c == '1' {
                        pat = pat | (A::Token::one() << ((bit - 1) as usize));
                    }

                    if read_pat && cur_group != "" {
                        *groups.get_mut(&cur_group).unwrap() = groups.get(&cur_group).unwrap().clone() | (A::Token::one() << ((bit - 1) as usize));
                    }

                    bit -= 1;
                }
                'a'...'z' | 'A'...'Z' => {
                    if read_pat {
                        panic!(
                            "Pattern syntax error: undelimited capture group name in '{}'",
                            self
                        );
                    } else {
                        cur_group.push(c);
                    }
                }
                _ => {
                    panic!(
                        "Pattern syntax error: invalid character '{}' in '{}'",
                        c,
                        self
                    );
                }
            }
        }

        if bit != 0 {
            panic!("Pattern syntax error: invalid pattern length in '{}'", self);
        }

        Rule::Terminal {
            pattern: pat,
            mask: mask,
            capture_group: groups
                .iter()
                .filter_map(
                    |x| if *x.1 != A::Token::zero() {
                        Some((x.0.clone(), x.1.clone()))
                    } else {
                        None
                    }
                )
                .collect(),
        }
    }
}

/// Internal to `new_disassembler!`
pub trait AddToRuleGen<A: Architecture> {
    /// Internal to `new_disassembler!`
    fn push(&self, &mut Vec<Vec<Rule<A>>>);
}

#[derive(Clone)]
/// Internal to `new_disassembler!`
pub struct OptionalRule<A: Architecture>(pub Rule<A>);

impl<A: Architecture> AddToRuleGen<A> for OptionalRule<A> {
    fn push(&self, rules: &mut Vec<Vec<Rule<A>>>) {
        let mut copy = rules.clone();
        for mut c in copy.iter_mut() {
            c.push(self.0.clone());
        }

        rules.append(&mut copy);
    }
}

impl<A: Architecture, T: Into<Rule<A>> + Clone> AddToRuleGen<A> for T {
    fn push(&self, rules: &mut Vec<Vec<Rule<A>>>) {
        for mut c in rules.iter_mut() {
            let s: Self = self.clone();
            c.push(s.into());
        }
    }
}

/// Internal to `new_disassembler!`
pub struct RuleGen<A: Architecture> {
    /// Internal to `new_disassembler!`
    pub rules: Vec<Vec<Rule<A>>>,
}

impl<A: Architecture> RuleGen<A> {
    /// Internal to `new_disassembler!`
    pub fn new() -> RuleGen<A> {
        RuleGen { rules: vec![vec![]] }
    }

    /// Internal to `new_disassembler!`
    pub fn push<T: AddToRuleGen<A>>(&mut self, t: &T) {
        t.push(&mut self.rules);
    }
}

#[macro_export]
macro_rules! opt {
    ($e:expr) => { { ::disassembler::OptionalRule($e.clone().into()) } };
}

#[macro_export]
macro_rules! new_disassembler {
    ($ty:ty => $( [ $( $t:expr ),+ ] = $f:expr),+) => {
        {
            let mut dis = $crate::disassembler::Disassembler::<$ty>::new();
            $({
                let mut gen = $crate::disassembler::RuleGen::new();
                $(
                    gen.push(&$t);
                )+
                fn a(a: &mut State<$ty>) -> bool { ($f)(a) };
                let fuc: $crate::disassembler::Action<$ty> = a;

                for r in gen.rules {
                    dis.add(&r,::std::sync::Arc::new(fuc));
                }
            })+

            ::std::sync::Arc::<$crate::disassembler::Disassembler<$ty>>::new(dis)
        }
    };
    ($ty:ty => $( [ $( $t:expr ),+ ] = $f:expr),+, _ = $def:expr) => {
        {
           let mut dis = $crate::disassembler::Disassembler::<$ty>::new();
            $({
                let mut gen = $crate::disassembler::RuleGen::new();
                $(
                    gen.push(&$t);
                )+
                fn a(a: &mut State<$ty>) -> bool { ($f)(a) };
                let fuc: $crate::disassembler::Action<$ty> = a;

                for r in gen.rules {
                    dis.add(&r,::std::sync::Arc::new(fuc));
                }
            })+


            fn __def(st: &mut State<$ty>) -> bool { ($def)(st) };
            dis.set_default(__def);

            ::std::sync::Arc::<$crate::disassembler::Disassembler<$ty>>::new(dis)
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use {Bound, Guard, OpaqueLayer, Region, Result, Rvalue};
    use std::sync::Arc;

    #[derive(Clone,Debug)]
    enum TestArchShort {}
    impl Architecture for TestArchShort {
        type Token = u8;
        type Configuration = ();

        fn prepare(_: &Region, _: &Self::Configuration) -> Result<Vec<(&'static str, u64, &'static str)>> {
            unimplemented!()
        }

        fn decode(_: &Region, _: u64, _: &Self::Configuration) -> Result<Match<Self>> {
            unimplemented!()
        }
    }

    #[derive(Clone,Debug)]
    enum TestArchWide {}
    impl Architecture for TestArchWide {
        type Token = u16;
        type Configuration = ();

        fn prepare(_: &Region, _: &Self::Configuration) -> Result<Vec<(&'static str, u64, &'static str)>> {
            unimplemented!()
        }

        fn decode(_: &Region, _: u64, _: &Self::Configuration) -> Result<Match<Self>> {
            unimplemented!()
        }
    }

    #[test]
    fn combine_expr() {
        let sub = new_disassembler!(TestArchShort =>
            [ 1 ] = &|_| { true },
            [ 2, 2 ] = &|_| { true }
        );

        let main = new_disassembler!(TestArchShort =>
            [ 3, sub ] = &|_| { true }
        );

        main.to_dot();
        sub.to_dot();
        let src = OpaqueLayer::wrap(vec![3, 1, 3, 2, 2]);

        {
            let maybe_res = main.next_match(&mut src.iter(), 0, ());

            assert!(maybe_res.is_some());
            let res = maybe_res.unwrap();

            assert_eq!(res.address, 0);
            assert_eq!(res.tokens.len(), 2);
            assert_eq!(res.tokens[0], 3);
            assert_eq!(res.tokens[1], 1);
            assert_eq!(res.groups.len(), 0);
            assert_eq!(res.mnemonics.len(), 0);
            assert_eq!(res.jumps.len(), 0);
        }

        {
            let maybe_res = main.next_match(&mut src.iter().seek(2), 2, ());

            assert!(maybe_res.is_some());
            let res = maybe_res.unwrap();

            assert_eq!(res.address, 2);
            assert_eq!(res.tokens.len(), 3);
            assert_eq!(res.tokens[0], 3);
            assert_eq!(res.tokens[1], 2);
            assert_eq!(res.tokens[2], 2);
            assert_eq!(res.groups.len(), 0);
            assert_eq!(res.mnemonics.len(), 0);
            assert_eq!(res.jumps.len(), 0);
        }
    }

    #[test]
    fn decode_macro() {
        let lock_prfx = new_disassembler!(TestArchShort =>
            [ 0x06 ] = &|_| { true }
        );

        new_disassembler!(TestArchShort =>
            [ 22 , 21, lock_prfx ] = &|_| { true },
            [ "....11 d@00"         ] = &|_| true,
            [ "....11 d@00", ".. d@0011. 0" ] = &|_| true
        );
    }

    fn fixture() -> (Arc<Disassembler<TestArchShort>>, Arc<Disassembler<TestArchShort>>, Arc<Disassembler<TestArchShort>>, OpaqueLayer) {
        let sub = new_disassembler!(TestArchShort =>
            [ 2 ] = |st: &mut State<TestArchShort>| {
                let next = st.address;
                st.mnemonic(2,"BA","",vec!(),&|_| { Ok(vec![]) }).unwrap();
                st.jump(Rvalue::new_u64(next + 2),Guard::always()).unwrap();
                true
            });
        let sub2 = new_disassembler!(TestArchShort =>
            [ 8 ] = &|_| false);

        let main = new_disassembler!(TestArchShort =>
            [ 1, sub ] = &|_| true,
            [ 1 ] = |st: &mut State<TestArchShort>| {
                let next = st.address;
                st.mnemonic(1,"A","",vec!(),&|_| { Ok(vec![]) }).unwrap();
                st.jump(Rvalue::new_u64(next + 1),Guard::always()).unwrap();
                true
            },
            [ "0 k@..... 11" ] = |st: &mut State<TestArchShort>| {
                let next = st.address;
                st.mnemonic(1,"C","",vec!(),&|_| { Ok(vec![]) }).unwrap();
                st.jump(Rvalue::new_u64(next + 1),Guard::always()).unwrap();
                true
            },
            _ = |st: &mut State<TestArchShort>| {
                let next = st.address;
                st.mnemonic(1,"UNK","",vec!(),&|_| { Ok(vec![]) }).unwrap();
                st.jump(Rvalue::new_u64(next + 1),Guard::always()).unwrap();
                true
            }
		);

        (sub, sub2, main, OpaqueLayer::wrap(vec![1, 1, 2, 1, 0b10111, 8, 1, 8]))
    }

    #[test]
    fn single_decoder() {
        let (_, _, main, def) = fixture();
        let maybe_res = main.next_match(&mut def.iter(), 0, ());

        assert!(maybe_res.is_some());
        let res = maybe_res.unwrap();

        assert_eq!(res.address, 0);
        assert_eq!(res.tokens.len(), 1);
        assert_eq!(res.tokens[0], 1);
        assert_eq!(res.groups.len(), 0);
        assert_eq!(res.mnemonics.len(), 1);
        assert_eq!(res.mnemonics[0].opcode, "A".to_string());
        assert_eq!(res.mnemonics[0].area, Bound::new(0, 1));
        assert_eq!(res.mnemonics[0].instructions.len(), 0);
        assert_eq!(res.jumps.len(), 1);

        if let &(0, Rvalue::Constant { value: 1, size: 64 }, ref g) = &res.jumps[0] {
            assert_eq!(g, &Guard::always());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn sub_decoder() {
        let (_, _, main, def) = fixture();
        let maybe_res = main.next_match(&mut def.iter().cut(&(1..def.len())), 1, ());

        assert!(maybe_res.is_some());
        let res = maybe_res.unwrap();

        assert_eq!(res.address, 1);
        assert_eq!(res.tokens.len(), 2);
        assert_eq!(res.tokens[0], 1);
        assert_eq!(res.tokens[1], 2);
        assert_eq!(res.groups.len(), 0);
        assert_eq!(res.mnemonics.len(), 1);
        assert_eq!(res.mnemonics[0].opcode, "BA".to_string());
        assert_eq!(res.mnemonics[0].area, Bound::new(1, 3));
        assert_eq!(res.mnemonics[0].instructions.len(), 0);
        assert_eq!(res.jumps.len(), 1);

        if let &(1, Rvalue::Constant { value: 3, size: 64 }, ref g) = &res.jumps[0] {
            assert_eq!(g, &Guard::always());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn semantic_false() {
        let (_, sub2, _, def) = fixture();
        let maybe_res = sub2.next_match(&mut def.iter().cut(&(7..def.len())), 7, ());

        assert!(maybe_res.is_none());
    }

    #[test]
    fn default_pattern() {
        let (_, _, main, def) = fixture();
        let maybe_res = main.next_match(&mut def.iter().cut(&(7..def.len())), 7, ());

        assert!(maybe_res.is_some());
        let res = maybe_res.unwrap();

        assert_eq!(res.address, 7);
        assert_eq!(res.tokens.len(), 1);
        assert_eq!(res.tokens[0], 8);
        assert_eq!(res.groups.len(), 0);
        assert_eq!(res.mnemonics.len(), 1);
        assert_eq!(res.mnemonics[0].opcode, "UNK".to_string());
        assert_eq!(res.mnemonics[0].area, Bound::new(7, 8));
        assert_eq!(res.mnemonics[0].instructions.len(), 0);
        assert_eq!(res.jumps.len(), 1);

        if let &(7, Rvalue::Constant { value: 8, size: 64 }, ref g) = &res.jumps[0] {
            assert_eq!(g, &Guard::always());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn slice() {
        let (_, _, main, def) = fixture();
        let maybe_res = main.next_match(&mut def.iter().cut(&(1..2)), 1, ());

        assert!(maybe_res.is_some());
        let res = maybe_res.unwrap();

        assert_eq!(res.address, 1);
        assert_eq!(res.tokens.len(), 1);
        assert_eq!(res.tokens[0], 1);
        assert_eq!(res.groups.len(), 0);
        assert_eq!(res.mnemonics.len(), 1);
        assert_eq!(res.mnemonics[0].opcode, "A".to_string());
        assert_eq!(res.mnemonics[0].area, Bound::new(1, 2));
        assert_eq!(res.mnemonics[0].instructions.len(), 0);
        assert_eq!(res.jumps.len(), 1);

        if let &(1, Rvalue::Constant { value: 2, size: 64 }, ref g) = &res.jumps[0] {
            assert_eq!(g, &Guard::always());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn empty() {
        let (_, _, main, def) = fixture();
        let maybe_res = main.next_match(&mut def.iter().cut(&(0..0)), 0, ());

        assert!(maybe_res.is_none());
    }

    #[test]
    fn capture_group() {
        let (_, _, main, def) = fixture();
        let maybe_res = main.next_match(&mut def.iter().cut(&(4..def.len())), 4, ());

        assert!(maybe_res.is_some());
        let res = maybe_res.unwrap();

        assert_eq!(res.address, 4);
        assert_eq!(res.tokens.len(), 1);
        assert_eq!(res.tokens[0], 0b10111);
        assert_eq!(res.groups.len(), 1);
        assert_eq!(res.groups, vec![("k".to_string(), 0b101)]);
        assert_eq!(res.mnemonics.len(), 1);
        assert_eq!(res.mnemonics[0].opcode, "C".to_string());
        assert_eq!(res.mnemonics[0].area, Bound::new(4, 5));
        assert_eq!(res.mnemonics[0].instructions.len(), 0);
        assert_eq!(res.jumps.len(), 1);

        if let &(4, Rvalue::Constant { value: 5, size: 64 }, ref g) = &res.jumps[0] {
            assert_eq!(g, &Guard::always());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn empty_capture_group() {
        let def = OpaqueLayer::wrap(vec![127]);
        let dec = new_disassembler!(TestArchShort =>
            ["01 a@.. 1 b@ c@..."] = |st: &mut State<TestArchShort>| {
                st.mnemonic(1, "1","",vec!(),&|_| { Ok(vec![]) }).unwrap();
                true
            }
        );
        let maybe_res = dec.next_match(&mut def.iter(), 0, ());

        assert!(maybe_res.is_some());
        let res = maybe_res.unwrap();

        assert_eq!(res.address, 0);
        assert_eq!(res.tokens.len(), 1);
        assert_eq!(res.tokens[0], 127);
        assert!(res.groups == vec![("a".to_string(), 3), ("c".to_string(), 7)] || res.groups == vec![("c".to_string(), 7), ("a".to_string(), 3)]);
        assert_eq!(res.mnemonics.len(), 1);
        assert_eq!(res.mnemonics[0].opcode, "1".to_string());
        assert_eq!(res.mnemonics[0].area, Bound::new(0, 1));
        assert_eq!(res.mnemonics[0].instructions.len(), 0);
        assert_eq!(res.jumps.len(), 0);
    }

    #[test]
    #[should_panic]
    fn too_long_capture_group() {
        new_disassembler!(TestArchShort => [ "k@........." ] = &|_| { true });
    }

    #[test]
    #[should_panic]
    fn too_long_token_pattern() {
        new_disassembler!(TestArchShort => [ "111111111" ] = &|_| { true });
    }

    #[test]
    #[should_panic]
    fn too_short_token_pattern() {
        new_disassembler!(TestArchShort => [ "1111111" ] = &|_| { true });
    }

    #[test]
    #[should_panic]
    fn invalid_char_in_token_pattern() {
        new_disassembler!(TestArchShort => [ "101/1010" ] = &|_| { true });
    }

    #[test]
    #[should_panic]
    fn invalid_token_pattern() {
        new_disassembler!(TestArchShort => [ "a111111" ] = &|_| { true });
    }

    #[test]
    fn wide_token() {
        let def = OpaqueLayer::wrap(vec![0x11, 0x22, 0x33, 0x44, 0x55, 0x44]);
        let dec = new_disassembler!(TestArchWide =>
            [0x2211] = |s: &mut State<TestArchWide>|
            {
                let a = s.address;
                s.mnemonic(2,"A","",vec!(),&|_| { Ok(vec![]) }).unwrap();
                s.jump(Rvalue::new_u64(a + 2),Guard::always()).unwrap();
                true
            },

            [0x4433] = |s: &mut State<TestArchWide>|
            {
                let a = s.address;
                s.mnemonic(2,"B","",vec!(),&|_| { Ok(vec![]) }).unwrap();
                s.jump(Rvalue::new_u64(a + 2),Guard::always()).unwrap();
                s.jump(Rvalue::new_u64(a + 4),Guard::always()).unwrap();
                true
            },

            [0x4455] = |s: &mut State<TestArchWide>|
            {
                s.mnemonic(2, "C","",vec!(),&|_| { Ok(vec![]) }).unwrap();
                true
            }
        );

        let maybe_res = dec.next_match(&mut def.iter(), 0, ());

        assert!(maybe_res.is_some());
        let res = maybe_res.unwrap();

        assert_eq!(res.address, 0);
        assert_eq!(res.tokens.len(), 1);
        assert_eq!(res.tokens[0], 0x2211);
        assert_eq!(res.mnemonics.len(), 1);
        assert_eq!(res.mnemonics[0].opcode, "A".to_string());
        assert_eq!(res.mnemonics[0].area, Bound::new(0, 2));
        assert_eq!(res.mnemonics[0].instructions.len(), 0);
        assert_eq!(res.jumps.len(), 1);
    }

    #[test]
    fn optional() {
        let def = OpaqueLayer::wrap(vec![127, 126, 125, 127, 125]);
        let dec = new_disassembler!(TestArchShort =>
            [127, opt!(126), 125] = |st: &mut State<TestArchShort>|
            {
                let l = st.tokens.len();
                st.mnemonic(l, "1", "", vec!(),&|_| { Ok(vec![]) }).unwrap();
                true
            }
        );

        dec.to_dot();

        {
            let maybe_res = dec.next_match(&mut def.iter(), 0, ());

            assert!(maybe_res.is_some());
            let res = maybe_res.unwrap();

            assert_eq!(res.address, 0);
            assert_eq!(res.tokens.len(), 3);
            assert_eq!(res.tokens, vec![127, 126, 125]);
            assert_eq!(res.mnemonics.len(), 1);
            assert_eq!(res.mnemonics[0].opcode, "1".to_string());
            assert_eq!(res.mnemonics[0].area, Bound::new(0, 3));
            assert_eq!(res.mnemonics[0].instructions.len(), 0);
            assert_eq!(res.jumps.len(), 0);
        }

        {
            let maybe_res = dec.next_match(&mut def.iter().cut(&(3..5)), 3, ());

            assert!(maybe_res.is_some());
            let res = maybe_res.unwrap();

            assert_eq!(res.address, 3);
            assert_eq!(res.tokens.len(), 2);
            assert_eq!(res.tokens, vec![127, 125]);
            assert_eq!(res.mnemonics.len(), 1);
            assert_eq!(res.mnemonics[0].opcode, "1".to_string());
            assert_eq!(res.mnemonics[0].area, Bound::new(3, 5));
            assert_eq!(res.mnemonics[0].instructions.len(), 0);
            assert_eq!(res.jumps.len(), 0);
        }
    }

    #[test]
    fn optional_group() {
        let def = OpaqueLayer::wrap(vec![127, 126]);
        let dec = new_disassembler!(TestArchShort =>
            [opt!("011 a@. 1111"), "0111111 b@.", "011 c@. 1110"] = |st: &mut State<TestArchShort>|
            {
                assert_eq!(st.get_group("b"),1);
                assert_eq!(st.get_group("c"),1);

                let l = st.tokens.len();
                st.mnemonic(l, "1", "", vec!(),&|_| { Ok(vec![]) }).unwrap();
                true
            }
        );

        {
            let maybe_res = dec.next_match(&mut def.iter(), 0, ());

            assert!(maybe_res.is_some());
            let res = maybe_res.unwrap();

            assert_eq!(res.address, 0);
            assert_eq!(res.tokens.len(), 2);
            assert_eq!(res.tokens, vec![127, 126]);
            assert_eq!(res.mnemonics.len(), 1);
            assert_eq!(res.mnemonics[0].opcode, "1".to_string());
            assert_eq!(res.mnemonics[0].area, Bound::new(0, 2));
            assert_eq!(res.mnemonics[0].instructions.len(), 0);
            assert_eq!(res.jumps.len(), 0);
        }
    }

    #[test]
    fn fixed_capture_group_contents() {
        let def = OpaqueLayer::wrap(vec![127, 255]);
        let dec = new_disassembler!(TestArchShort =>
            [ "01111111", "a@11111111" ] = |st: &mut State<TestArchShort>|
            {
                let l = st.tokens.len();
                st.mnemonic(l, "1", "", vec!(),&|_| { Ok(vec![]) }).unwrap();
                true
            }
        );

        let maybe_res = dec.next_match(&mut def.iter(), 0, ());

        assert!(maybe_res.is_some());
        let res = maybe_res.unwrap();

        assert_eq!(res.address, 0);
        assert_eq!(res.tokens.len(), 2);
        assert_eq!(res.tokens, vec![127, 255]);
        assert_eq!(res.groups, vec![("a".to_string(), 255)]);
        assert_eq!(res.mnemonics.len(), 1);
        assert_eq!(res.mnemonics[0].opcode, "1".to_string());
        assert_eq!(res.mnemonics[0].area, Bound::new(0, 2));
        assert_eq!(res.mnemonics[0].instructions.len(), 0);
        assert_eq!(res.jumps.len(), 0);
    }
}

use std::io::{Write,Cursor,Read};
use std::ops::{Range};
use std::collections::HashMap;
use leb128;
use uuid::Uuid;
use neo::il::{Statement,Endianess,CallTarget,Operation};
use neo::value::{Constant,Variable,Value};
use neo::{Str,Result};

#[derive(Debug,Clone)]
struct Strings {
    keys: HashMap<Str,usize>,
    strings: Vec<Str>,
}

impl Strings {
    pub fn with_capacity(cap: usize) -> Strings {
        Strings{
            keys: HashMap::with_capacity(cap),
            strings: Vec::with_capacity(cap),
        }
    }

    pub fn insert(&mut self, s: Str) -> usize {
        if let Some(i) = self.keys.get(&s).cloned() {
            i
        } else {
            let i = self.strings.len();
            self.keys.insert(s.clone(),i);
            self.strings.push(s);
            i
        }
    }

    pub fn get(&self, i: usize) -> Str {
        self.strings[i].clone()
    }

    pub fn len(&self) -> usize {
        self.strings.len()
    }
}

impl Default for Strings {
    fn default() -> Strings {
        Strings{
            keys: HashMap::default(),
            strings: Vec::default(),
        }
    }
}

#[derive(Clone,Debug)]
pub struct Bitcode {
    data: Vec<u8>,
    strings: Strings,
}
// const: <len, pow2><leb128 value>
// var: <name, leb128 str idx>, <subscript, leb128 + 1>, <len, value>

//  1\2  c   v   u
//   c|000 001 010
//   v|011 100 101
//   u|110 111 xxx
//
// add  00000--- <a> <b> <res>
// sub  00001--- <a> <b> <res>
// mul  00010--- <a> <b> <res>
// divu 00011--- <a> <b> <res>
// divs 00100--- <a> <b> <res>
// shl  00101--- <a> <b> <res>
// shru 00110--- <a> <b> <res>
// shrs 00111--- <a> <b> <res>
// mod  01000--- <a> <b> <res>
// and  01001--- <a> <b> <res>
// or   01010--- <a> <b> <res>
// xor  01011--- <a> <b> <res>
// eq   01100--- <a> <b> <res>
// leu  01101--- <a> <b> <res>
// les  01110--- <a> <b> <res>
// ltu  01111--- <a> <b> <res>
// lts  10000--- <a> <b> <res>
//
// c: 0, v: 1
// little: 0, big: 1
// zext   1000100- <size, leb128> <a>
// sext   1000101- <size, leb128> <a>
// mov    1000110- <a>
// movu   10001110
// init   10001111 <name, leb128> <size, leb128>
// sel    100100-- <size, leb128> <start> <a>
// load   100101e- <region, leb128> <size, leb128> <a>
// phi2   10011000 <a, var> <b, var> 3 0x0
// phi3   10011001 <a, var> <b, var> <c, var>
// call   10011010 <stub, leb128>
// call   10011011 <uuid, leb128>
// icall  1001110- <a>
// ucall  10011110
// phi0   10011111 3*3 0x0
// store  1010e--- <region, leb128> <size, leb128> <addr> <val>
// ret    10110000
//        10110001
// loadu  1011001e <region, leb128> <size, leb128>
// phi1   10110100 <a> 2*3 0x0

macro_rules! encoding_rule {
    ( $val:tt [ c , c ] => $a:expr, $b:expr, $res:expr, $data:expr, $strtbl:expr ) => {{
        let val = $val;
        let a = $a;
        let b = $b;
        let res = $res;
        let data = $data;
        let strtbl = $strtbl;

        data.write(&[val])?;
        Self::encode_constant(a,data)?;
        Self::encode_constant(b,data)?;
        Self::encode_variable(res,data,strtbl)?;
    }};
    ( $val:tt [ c , v ] => $a:expr, $b:expr, $res:expr, $data:expr, $strtbl:expr ) => {{
        let val = $val;
        let a = $a;
        let b = $b;
        let res = $res;
        let data = $data;
        let strtbl = $strtbl;

        data.write(&[val])?;
        Self::encode_constant(a,data)?;
        Self::encode_variable(b,data,strtbl)?;
        Self::encode_variable(res,data,strtbl)?;
    }};
    ( $val:tt [ c , u ] => $a:expr, $res:expr, $data:expr, $strtbl:expr ) => {{
        let val = $val;
        let a = $a;
        let res = $res;
        let data = $data;
        let strtbl = $strtbl;

        data.write(&[val])?;
        Self::encode_constant(a,data)?;
        Self::encode_variable(res,data,strtbl)?;
    }};
    ( $val:tt [ v , c ] => $a:expr, $b:expr, $res:expr, $data:expr, $strtbl:expr ) => {{
        let val = $val;
        let a = $a;
        let b = $b;
        let res = $res;
        let data = $data;
        let strtbl = $strtbl;

        data.write(&[val])?;
        Self::encode_variable(a,data,strtbl)?;
        Self::encode_constant(b,data)?;
        Self::encode_variable(res,data,strtbl)?;
    }};
    ( $val:tt [ v , v ] => $a:expr, $b:expr, $res:expr, $data:expr, $strtbl:expr ) => {{
        let val = $val;
        let a = $a;
        let b = $b;
        let res = $res;
        let data = $data;
        let strtbl = $strtbl;

        data.write(&[val])?;
        Self::encode_variable(a,data,strtbl)?;
        Self::encode_variable(b,data,strtbl)?;
        Self::encode_variable(res,data,strtbl)?;
    }};
    ( $val:tt [ v , u ] => $a:expr, $res:expr, $data:expr, $strtbl:expr ) => {{
        let val = $val;
        let a = $a;
        let res = $res;
        let data = $data;
        let strtbl = $strtbl;

        data.write(&[val])?;
        Self::encode_variable(a,data,strtbl)?;
        Self::encode_variable(res,data,strtbl)?;
    }};
    ( $val:tt [ u , c ] => $b:expr, $res:expr, $data:expr, $strtbl:expr ) => {{
        let val = $val;
        let b = $b;
        let res = $res;
        let data = $data;
        let strtbl = $strtbl;

        data.write(&[val])?;
        Self::encode_constant(b,data)?;
        Self::encode_variable(res,data,strtbl)?;
    }};
    ( $val:tt [ u , v ] => $b:expr, $res:expr, $data:expr, $strtbl:expr ) => {{
        let val = $val;
        let b = $b;
        let res = $res;
        let data = $data;
        let strtbl = $strtbl;

        data.write(&[val])?;
        Self::encode_variable(b,data,strtbl)?;
        Self::encode_variable(res,data,strtbl)?;
    }};
}

impl Default for Bitcode {
    fn default() -> Bitcode {
        Bitcode{
            strings: Strings::default(),
            data: Vec::new(),
        }
    }
}

impl Bitcode {
    pub fn append<I: IntoIterator<Item=Statement<Value>> + Sized>(&mut self, i: I) -> Result<Range<usize>> {
        let mut buf = Cursor::new(Vec::new());
        let start = self.data.len();

        for stmt in i {
            Self::encode_statement(stmt,&mut buf,&mut self.strings)?;
        }

        self.data.extend(buf.into_inner().into_iter());
        Ok(start..self.data.len())
    }

    pub fn new(v: Vec<Statement<Value>>) -> Result<Bitcode> {
        let mut strtbl = Strings::default();
        let mut buf = Cursor::new(Vec::new());

        for stmt in v {
            Self::encode_statement(stmt,&mut buf,&mut strtbl)?;
        }

        Ok(Bitcode{ data: buf.into_inner(), strings: strtbl })
    }

    pub fn with_capacity(bytes: usize, strs: usize) -> Bitcode {
        Bitcode{
            strings: Strings::with_capacity(strs),
            data: Vec::with_capacity(bytes),
        }
    }

    pub fn rewrite<F: FnMut(&mut Statement<Value>) -> Result<()> + Sized>(&mut self, range: Range<usize>, mut func: F) -> Result<Range<usize>> {
        debug!("rewrite bitcode in {:?}",range);
        let mut read_pos = range.start;
        let mut range_pos = range.start;
        let mut write_pos = range.start;
        let mut tmp = Vec::with_capacity(10);

        while range_pos < range.end {
            let mut stmt = {
                debug!("read from {:?}",read_pos);
                let mut read = Cursor::new(&self.data[read_pos..]);
                let stmt = self.decode_statement(&mut read)?;

                read_pos += read.position() as usize;
                range_pos += read.position() as usize;
                debug!("read to {:?}",read_pos);
                stmt
            };

            debug!("map {:?}",stmt);
            func(&mut stmt)?;
            debug!("  to {:?}",stmt);

            {
                let mut write = Cursor::new(tmp);

                Self::encode_statement(stmt, &mut write, &mut self.strings)?;
                tmp = write.into_inner();

                if read_pos - write_pos < tmp.len() {
                    let diff = tmp.len() - (read_pos - write_pos);
                    debug!("make space for {} more bytes",diff);

                    self.data.reserve(diff);
                    // XXX
                    for _ in 0..diff { self.data.insert(write_pos,42); }
                    read_pos += diff;
                }
            }

            //debug!("data: {:?}",self.data);
            debug!("write {:?} at {}",tmp,write_pos);

            {
                let mut write = Cursor::new(&mut self.data[write_pos..]);
                write_pos += write.write(&tmp)?;
                tmp.clear();
            }

            //debug!("data: {:?}",self.data);
        }

        if read_pos > write_pos {
            debug!("remove {:?}",write_pos..read_pos);
            self.data.drain(write_pos..read_pos);
        }

        Ok(range.start..write_pos)
    }

    pub fn remove(&mut self, range: Range<usize>) {
        self.data.drain(range);
    }

    pub fn insert(&mut self, pos: usize, stmts: Vec<Statement<Value>>) -> Result<Range<usize>> {
        let data = Vec::with_capacity(stmts.len() * 10);
        let mut cur = Cursor::new(data);

        for stmt in stmts {
            Self::encode_statement(stmt, &mut cur, &mut self.strings)?;
        }

        let buf = cur.into_inner();
        let len = buf.len();

        for b in buf.into_iter().rev() {
            self.data.insert(pos,b);
        }
            //        self.data.splice(pos..pos,buf.into_iter());
        Ok(pos..(pos + len))
    }

    fn encode_statement<W: Write>(stmt: Statement<Value>, data: &mut W, strtbl: &mut Strings) -> Result<()> {
        use neo::il::Operation::*;
        use neo::value::Value::*;

        match stmt {
            // Add: 0b00000---
            Statement::Expression{ op: Add(Constant(a),Constant(b)), result } => encoding_rule!( 0b00000_000 [c,c] => a, b, result, data, strtbl ),
            Statement::Expression{ op: Add(Constant(a),Variable(b)), result } => encoding_rule!( 0b00000_001 [c,v] => a, b, result, data, strtbl ),
            Statement::Expression{ op: Add(Constant(a),Undefined), result } => encoding_rule!( 0b00000_010 [c,u] => a, result, data, strtbl ),
            Statement::Expression{ op: Add(Variable(a),Constant(b)), result } => encoding_rule!( 0b00000_011 [v,c] => a, b, result, data, strtbl ),
            Statement::Expression{ op: Add(Variable(a),Variable(b)), result } => encoding_rule!( 0b00000_100 [v,v] => a, b, result, data, strtbl ),
            Statement::Expression{ op: Add(Variable(a),Undefined), result } => encoding_rule!( 0b00000_101 [v,u] => a, result, data, strtbl ),
            Statement::Expression{ op: Add(Undefined,Constant(b)), result } => encoding_rule!( 0b00000_110 [u,c] => b, result, data, strtbl ),
            Statement::Expression{ op: Add(Undefined,Variable(b)), result } => encoding_rule!( 0b00000_111 [u,v] => b, result, data, strtbl ),
            Statement::Expression{ op: Add(Undefined,Undefined), result } => {
                Self::encode_statement(Statement::Expression{ op: Move(Undefined), result: result },data,strtbl)?;
            }

            // Subtract: 0b00001---
            Statement::Expression{ op: Subtract(Constant(a),Constant(b)), result } => encoding_rule!( 0b00001_000 [c,c] => a, b, result, data, strtbl ),
            Statement::Expression{ op: Subtract(Constant(a),Variable(b)), result } => encoding_rule!( 0b00001_001 [c,v] => a, b, result, data, strtbl ),
            Statement::Expression{ op: Subtract(Constant(a),Undefined), result } => encoding_rule!( 0b00001_010 [c,u] => a, result, data, strtbl ),
            Statement::Expression{ op: Subtract(Variable(a),Constant(b)), result } => encoding_rule!( 0b00001_011 [v,c] => a, b, result, data, strtbl ),
            Statement::Expression{ op: Subtract(Variable(a),Variable(b)), result } => encoding_rule!( 0b00001_100 [v,v] => a, b, result, data, strtbl ),
            Statement::Expression{ op: Subtract(Variable(a),Undefined), result } => encoding_rule!( 0b00001_101 [v,u] => a, result, data, strtbl ),
            Statement::Expression{ op: Subtract(Undefined,Constant(b)), result } => encoding_rule!( 0b00001_110 [u,c] => b, result, data, strtbl ),
            Statement::Expression{ op: Subtract(Undefined,Variable(b)), result } => encoding_rule!( 0b00001_111 [u,v] => b, result, data, strtbl ),
            Statement::Expression{ op: Subtract(Undefined,Undefined), result } => {
                Self::encode_statement(Statement::Expression{ op: Move(Undefined), result: result },data,strtbl)?;
            }

            // Multiply: 0b00010---
            Statement::Expression{ op: Multiply(Constant(a),Constant(b)), result } => encoding_rule!( 0b00010_000 [c,c] => a, b, result, data, strtbl ),
            Statement::Expression{ op: Multiply(Constant(a),Variable(b)), result } => encoding_rule!( 0b00010_001 [c,v] => a, b, result, data, strtbl ),
            Statement::Expression{ op: Multiply(Constant(a),Undefined), result } => encoding_rule!( 0b00010_010 [c,u] => a, result, data, strtbl ),
            Statement::Expression{ op: Multiply(Variable(a),Constant(b)), result } => encoding_rule!( 0b00010_011 [v,c] => a, b, result, data, strtbl ),
            Statement::Expression{ op: Multiply(Variable(a),Variable(b)), result } => encoding_rule!( 0b00010_100 [v,v] => a, b, result, data, strtbl ),
            Statement::Expression{ op: Multiply(Variable(a),Undefined), result } => encoding_rule!( 0b00010_101 [v,u] => a, result, data, strtbl ),
            Statement::Expression{ op: Multiply(Undefined,Constant(b)), result } => encoding_rule!( 0b00010_110 [u,c] => b, result, data, strtbl ),
            Statement::Expression{ op: Multiply(Undefined,Variable(b)), result } => encoding_rule!( 0b00010_111 [u,v] => b, result, data, strtbl ),
            Statement::Expression{ op: Multiply(Undefined,Undefined), result } => {
                Self::encode_statement(Statement::Expression{ op: Move(Undefined), result: result },data,strtbl)?;
            }

            // DivideUnsigned: 0b00011---
            Statement::Expression{ op: DivideUnsigned(Constant(a),Constant(b)), result } => encoding_rule!( 0b00011_000 [c,c] => a, b, result, data, strtbl ),
            Statement::Expression{ op: DivideUnsigned(Constant(a),Variable(b)), result } => encoding_rule!( 0b00011_001 [c,v] => a, b, result, data, strtbl ),
            Statement::Expression{ op: DivideUnsigned(Constant(a),Undefined), result } => encoding_rule!( 0b00011_010 [c,u] => a, result, data, strtbl ),
            Statement::Expression{ op: DivideUnsigned(Variable(a),Constant(b)), result } => encoding_rule!( 0b00011_011 [v,c] => a, b, result, data, strtbl ),
            Statement::Expression{ op: DivideUnsigned(Variable(a),Variable(b)), result } => encoding_rule!( 0b00011_100 [v,v] => a, b, result, data, strtbl ),
            Statement::Expression{ op: DivideUnsigned(Variable(a),Undefined), result } => encoding_rule!( 0b00011_101 [v,u] => a, result, data, strtbl ),
            Statement::Expression{ op: DivideUnsigned(Undefined,Constant(b)), result } => encoding_rule!( 0b00011_110 [u,c] => b, result, data, strtbl ),
            Statement::Expression{ op: DivideUnsigned(Undefined,Variable(b)), result } => encoding_rule!( 0b00011_111 [u,v] => b, result, data, strtbl ),
            Statement::Expression{ op: DivideUnsigned(Undefined,Undefined), result } => {
                Self::encode_statement(Statement::Expression{ op: Move(Undefined), result: result },data,strtbl)?;
            }

            // DivideSigned: 0b00100---
            Statement::Expression{ op: DivideSigned(Constant(a),Constant(b)), result } => encoding_rule!( 0b00100_000 [c,c] => a, b, result, data, strtbl ),
            Statement::Expression{ op: DivideSigned(Constant(a),Variable(b)), result } => encoding_rule!( 0b00100_001 [c,v] => a, b, result, data, strtbl ),
            Statement::Expression{ op: DivideSigned(Constant(a),Undefined), result } => encoding_rule!( 0b00100_010 [c,u] => a, result, data, strtbl ),
            Statement::Expression{ op: DivideSigned(Variable(a),Constant(b)), result } => encoding_rule!( 0b00100_011 [v,c] => a, b, result, data, strtbl ),
            Statement::Expression{ op: DivideSigned(Variable(a),Variable(b)), result } => encoding_rule!( 0b00100_100 [v,v] => a, b, result, data, strtbl ),
            Statement::Expression{ op: DivideSigned(Variable(a),Undefined), result } => encoding_rule!( 0b00100_101 [v,u] => a, result, data, strtbl ),
            Statement::Expression{ op: DivideSigned(Undefined,Constant(b)), result } => encoding_rule!( 0b00100_110 [u,c] => b, result, data, strtbl ),
            Statement::Expression{ op: DivideSigned(Undefined,Variable(b)), result } => encoding_rule!( 0b00100_111 [u,v] => b, result, data, strtbl ),
            Statement::Expression{ op: DivideSigned(Undefined,Undefined), result } => {
                Self::encode_statement(Statement::Expression{ op: Move(Undefined), result: result },data,strtbl)?;
            }

            // ShiftLeft: 0b00101---
            Statement::Expression{ op: ShiftLeft(Constant(a),Constant(b)), result } => encoding_rule!( 0b00101_000 [c,c] => a, b, result, data, strtbl ),
            Statement::Expression{ op: ShiftLeft(Constant(a),Variable(b)), result } => encoding_rule!( 0b00101_001 [c,v] => a, b, result, data, strtbl ),
            Statement::Expression{ op: ShiftLeft(Constant(a),Undefined), result } => encoding_rule!( 0b00101_010 [c,u] => a, result, data, strtbl ),
            Statement::Expression{ op: ShiftLeft(Variable(a),Constant(b)), result } => encoding_rule!( 0b00101_011 [v,c] => a, b, result, data, strtbl ),
            Statement::Expression{ op: ShiftLeft(Variable(a),Variable(b)), result } => encoding_rule!( 0b00101_100 [v,v] => a, b, result, data, strtbl ),
            Statement::Expression{ op: ShiftLeft(Variable(a),Undefined), result } => encoding_rule!( 0b00101_101 [v,u] => a, result, data, strtbl ),
            Statement::Expression{ op: ShiftLeft(Undefined,Constant(b)), result } => encoding_rule!( 0b00101_110 [u,c] => b, result, data, strtbl ),
            Statement::Expression{ op: ShiftLeft(Undefined,Variable(b)), result } => encoding_rule!( 0b00101_111 [u,v] => b, result, data, strtbl ),
            Statement::Expression{ op: ShiftLeft(Undefined,Undefined), result } => {
                Self::encode_statement(Statement::Expression{ op: Move(Undefined), result: result },data,strtbl)?;
            }

            // ShiftRightUnsigned: 0b00110---
            Statement::Expression{ op: ShiftRightUnsigned(Constant(a),Constant(b)), result } => encoding_rule!( 0b00110_000 [c,c] => a, b, result, data, strtbl ),
            Statement::Expression{ op: ShiftRightUnsigned(Constant(a),Variable(b)), result } => encoding_rule!( 0b00110_001 [c,v] => a, b, result, data, strtbl ),
            Statement::Expression{ op: ShiftRightUnsigned(Constant(a),Undefined), result } => encoding_rule!( 0b00110_010 [c,u] => a, result, data, strtbl ),
            Statement::Expression{ op: ShiftRightUnsigned(Variable(a),Constant(b)), result } => encoding_rule!( 0b00110_011 [v,c] => a, b, result, data, strtbl ),
            Statement::Expression{ op: ShiftRightUnsigned(Variable(a),Variable(b)), result } => encoding_rule!( 0b00110_100 [v,v] => a, b, result, data, strtbl ),
            Statement::Expression{ op: ShiftRightUnsigned(Variable(a),Undefined), result } => encoding_rule!( 0b00110_101 [v,u] => a, result, data, strtbl ),
            Statement::Expression{ op: ShiftRightUnsigned(Undefined,Constant(b)), result } => encoding_rule!( 0b00110_110 [u,c] => b, result, data, strtbl ),
            Statement::Expression{ op: ShiftRightUnsigned(Undefined,Variable(b)), result } => encoding_rule!( 0b00110_111 [u,v] => b, result, data, strtbl ),
            Statement::Expression{ op: ShiftRightUnsigned(Undefined,Undefined), result } => {
                Self::encode_statement(Statement::Expression{ op: Move(Undefined), result: result },data,strtbl)?;
            }

            // ShiftRightSigned: 0b00111---
            Statement::Expression{ op: ShiftRightSigned(Constant(a),Constant(b)), result } => encoding_rule!( 0b00111_000 [c,c] => a, b, result, data, strtbl ),
            Statement::Expression{ op: ShiftRightSigned(Constant(a),Variable(b)), result } => encoding_rule!( 0b00111_001 [c,v] => a, b, result, data, strtbl ),
            Statement::Expression{ op: ShiftRightSigned(Constant(a),Undefined), result } => encoding_rule!( 0b00111_010 [c,u] => a, result, data, strtbl ),
            Statement::Expression{ op: ShiftRightSigned(Variable(a),Constant(b)), result } => encoding_rule!( 0b00111_011 [v,c] => a, b, result, data, strtbl ),
            Statement::Expression{ op: ShiftRightSigned(Variable(a),Variable(b)), result } => encoding_rule!( 0b00111_100 [v,v] => a, b, result, data, strtbl ),
            Statement::Expression{ op: ShiftRightSigned(Variable(a),Undefined), result } => encoding_rule!( 0b00111_101 [v,u] => a, result, data, strtbl ),
            Statement::Expression{ op: ShiftRightSigned(Undefined,Constant(b)), result } => encoding_rule!( 0b00111_110 [u,c] => b, result, data, strtbl ),
            Statement::Expression{ op: ShiftRightSigned(Undefined,Variable(b)), result } => encoding_rule!( 0b00111_111 [u,v] => b, result, data, strtbl ),
            Statement::Expression{ op: ShiftRightSigned(Undefined,Undefined), result } => {
                Self::encode_statement(Statement::Expression{ op: Move(Undefined), result: result },data,strtbl)?;
            }

            // Modulo: 0b01000---
            Statement::Expression{ op: Modulo(Constant(a),Constant(b)), result } => encoding_rule!( 0b01000_000 [c,c] => a, b, result, data, strtbl ),
            Statement::Expression{ op: Modulo(Constant(a),Variable(b)), result } => encoding_rule!( 0b01000_001 [c,v] => a, b, result, data, strtbl ),
            Statement::Expression{ op: Modulo(Constant(a),Undefined), result } => encoding_rule!( 0b01000_010 [c,u] => a, result, data, strtbl ),
            Statement::Expression{ op: Modulo(Variable(a),Constant(b)), result } => encoding_rule!( 0b01000_011 [v,c] => a, b, result, data, strtbl ),
            Statement::Expression{ op: Modulo(Variable(a),Variable(b)), result } => encoding_rule!( 0b01000_100 [v,v] => a, b, result, data, strtbl ),
            Statement::Expression{ op: Modulo(Variable(a),Undefined), result } => encoding_rule!( 0b01000_101 [v,u] => a, result, data, strtbl ),
            Statement::Expression{ op: Modulo(Undefined,Constant(b)), result } => encoding_rule!( 0b01000_110 [u,c] => b, result, data, strtbl ),
            Statement::Expression{ op: Modulo(Undefined,Variable(b)), result } => encoding_rule!( 0b01000_111 [u,v] => b, result, data, strtbl ),
            Statement::Expression{ op: Modulo(Undefined,Undefined), result } => {
                Self::encode_statement(Statement::Expression{ op: Move(Undefined), result: result },data,strtbl)?;
            }

            // And: 0b01001---
            Statement::Expression{ op: And(Constant(a),Constant(b)), result } => encoding_rule!( 0b01001_000 [c,c] => a, b, result, data, strtbl ),
            Statement::Expression{ op: And(Constant(a),Variable(b)), result } => encoding_rule!( 0b01001_001 [c,v] => a, b, result, data, strtbl ),
            Statement::Expression{ op: And(Constant(a),Undefined), result } => encoding_rule!( 0b01001_010 [c,u] => a, result, data, strtbl ),
            Statement::Expression{ op: And(Variable(a),Constant(b)), result } => encoding_rule!( 0b01001_011 [v,c] => a, b, result, data, strtbl ),
            Statement::Expression{ op: And(Variable(a),Variable(b)), result } => encoding_rule!( 0b01001_100 [v,v] => a, b, result, data, strtbl ),
            Statement::Expression{ op: And(Variable(a),Undefined), result } => encoding_rule!( 0b01001_101 [v,u] => a, result, data, strtbl ),
            Statement::Expression{ op: And(Undefined,Constant(b)), result } => encoding_rule!( 0b01001_110 [u,c] => b, result, data, strtbl ),
            Statement::Expression{ op: And(Undefined,Variable(b)), result } => encoding_rule!( 0b01001_111 [u,v] => b, result, data, strtbl ),
            Statement::Expression{ op: And(Undefined,Undefined), result } => {
                Self::encode_statement(Statement::Expression{ op: Move(Undefined), result: result },data,strtbl)?;
            }

            // InclusiveOr: 0b01010---
            Statement::Expression{ op: InclusiveOr(Constant(a),Constant(b)), result } => encoding_rule!( 0b01010_000 [c,c] => a, b, result, data, strtbl ),
            Statement::Expression{ op: InclusiveOr(Constant(a),Variable(b)), result } => encoding_rule!( 0b01010_001 [c,v] => a, b, result, data, strtbl ),
            Statement::Expression{ op: InclusiveOr(Constant(a),Undefined), result } => encoding_rule!( 0b01010_010 [c,u] => a, result, data, strtbl ),
            Statement::Expression{ op: InclusiveOr(Variable(a),Constant(b)), result } => encoding_rule!( 0b01010_011 [v,c] => a, b, result, data, strtbl ),
            Statement::Expression{ op: InclusiveOr(Variable(a),Variable(b)), result } => encoding_rule!( 0b01010_100 [v,v] => a, b, result, data, strtbl ),
            Statement::Expression{ op: InclusiveOr(Variable(a),Undefined), result } => encoding_rule!( 0b01010_101 [v,u] => a, result, data, strtbl ),
            Statement::Expression{ op: InclusiveOr(Undefined,Constant(b)), result } => encoding_rule!( 0b01010_110 [u,c] => b, result, data, strtbl ),
            Statement::Expression{ op: InclusiveOr(Undefined,Variable(b)), result } => encoding_rule!( 0b01010_111 [u,v] => b, result, data, strtbl ),
            Statement::Expression{ op: InclusiveOr(Undefined,Undefined), result } => {
                Self::encode_statement(Statement::Expression{ op: Move(Undefined), result: result },data,strtbl)?;
            }

            // ExclusiveOr: 0b01011---
            Statement::Expression{ op: ExclusiveOr(Constant(a),Constant(b)), result } => encoding_rule!( 0b01011_000 [c,c] => a, b, result, data, strtbl ),
            Statement::Expression{ op: ExclusiveOr(Constant(a),Variable(b)), result } => encoding_rule!( 0b01011_001 [c,v] => a, b, result, data, strtbl ),
            Statement::Expression{ op: ExclusiveOr(Constant(a),Undefined), result } => encoding_rule!( 0b01011_010 [c,u] => a, result, data, strtbl ),
            Statement::Expression{ op: ExclusiveOr(Variable(a),Constant(b)), result } => encoding_rule!( 0b01011_011 [v,c] => a, b, result, data, strtbl ),
            Statement::Expression{ op: ExclusiveOr(Variable(a),Variable(b)), result } => encoding_rule!( 0b01011_100 [v,v] => a, b, result, data, strtbl ),
            Statement::Expression{ op: ExclusiveOr(Variable(a),Undefined), result } => encoding_rule!( 0b01011_101 [v,u] => a, result, data, strtbl ),
            Statement::Expression{ op: ExclusiveOr(Undefined,Constant(b)), result } => encoding_rule!( 0b01011_110 [u,c] => b, result, data, strtbl ),
            Statement::Expression{ op: ExclusiveOr(Undefined,Variable(b)), result } => encoding_rule!( 0b01011_111 [u,v] => b, result, data, strtbl ),
            Statement::Expression{ op: ExclusiveOr(Undefined,Undefined), result } => {
                Self::encode_statement(Statement::Expression{ op: Move(Undefined), result: result },data,strtbl)?;
            }

            // Equal: 0b01100---
            Statement::Expression{ op: Equal(Constant(a),Constant(b)), result } => encoding_rule!( 0b01100_000 [c,c] => a, b, result, data, strtbl ),
            Statement::Expression{ op: Equal(Constant(a),Variable(b)), result } => encoding_rule!( 0b01100_001 [c,v] => a, b, result, data, strtbl ),
            Statement::Expression{ op: Equal(Constant(a),Undefined), result } => encoding_rule!( 0b01100_010 [c,u] => a, result, data, strtbl ),
            Statement::Expression{ op: Equal(Variable(a),Constant(b)), result } => encoding_rule!( 0b01100_011 [v,c] => a, b, result, data, strtbl ),
            Statement::Expression{ op: Equal(Variable(a),Variable(b)), result } => encoding_rule!( 0b01100_100 [v,v] => a, b, result, data, strtbl ),
            Statement::Expression{ op: Equal(Variable(a),Undefined), result } => encoding_rule!( 0b01100_101 [v,u] => a, result, data, strtbl ),
            Statement::Expression{ op: Equal(Undefined,Constant(b)), result } => encoding_rule!( 0b01100_110 [u,c] => b, result, data, strtbl ),
            Statement::Expression{ op: Equal(Undefined,Variable(b)), result } => encoding_rule!( 0b01100_111 [u,v] => b, result, data, strtbl ),
            Statement::Expression{ op: Equal(Undefined,Undefined), result } => {
                Self::encode_statement(Statement::Expression{ op: Move(Undefined), result: result },data,strtbl)?;
            }

            // LessOrEqualUnsigned: 0b01101---
            Statement::Expression{ op: LessOrEqualUnsigned(Constant(a),Constant(b)), result } => encoding_rule!( 0b01101_000 [c,c] => a, b, result, data, strtbl ),
            Statement::Expression{ op: LessOrEqualUnsigned(Constant(a),Variable(b)), result } => encoding_rule!( 0b01101_001 [c,v] => a, b, result, data, strtbl ),
            Statement::Expression{ op: LessOrEqualUnsigned(Constant(a),Undefined), result } => encoding_rule!( 0b01101_010 [c,u] => a, result, data, strtbl ),
            Statement::Expression{ op: LessOrEqualUnsigned(Variable(a),Constant(b)), result } => encoding_rule!( 0b01101_011 [v,c] => a, b, result, data, strtbl ),
            Statement::Expression{ op: LessOrEqualUnsigned(Variable(a),Variable(b)), result } => encoding_rule!( 0b01101_100 [v,v] => a, b, result, data, strtbl ),
            Statement::Expression{ op: LessOrEqualUnsigned(Variable(a),Undefined), result } => encoding_rule!( 0b01101_101 [v,u] => a, result, data, strtbl ),
            Statement::Expression{ op: LessOrEqualUnsigned(Undefined,Constant(b)), result } => encoding_rule!( 0b01101_110 [u,c] => b, result, data, strtbl ),
            Statement::Expression{ op: LessOrEqualUnsigned(Undefined,Variable(b)), result } => encoding_rule!( 0b01101_111 [u,v] => b, result, data, strtbl ),
            Statement::Expression{ op: LessOrEqualUnsigned(Undefined,Undefined), result } => {
                Self::encode_statement(Statement::Expression{ op: Move(Undefined), result: result },data,strtbl)?;
            }

            // LessOrEqualSigned: 0b01110---
            Statement::Expression{ op: LessOrEqualSigned(Constant(a),Constant(b)), result } => encoding_rule!( 0b01110_000 [c,c] => a, b, result, data, strtbl ),
            Statement::Expression{ op: LessOrEqualSigned(Constant(a),Variable(b)), result } => encoding_rule!( 0b01110_001 [c,v] => a, b, result, data, strtbl ),
            Statement::Expression{ op: LessOrEqualSigned(Constant(a),Undefined), result } => encoding_rule!( 0b01110_010 [c,u] => a, result, data, strtbl ),
            Statement::Expression{ op: LessOrEqualSigned(Variable(a),Constant(b)), result } => encoding_rule!( 0b01110_011 [v,c] => a, b, result, data, strtbl ),
            Statement::Expression{ op: LessOrEqualSigned(Variable(a),Variable(b)), result } => encoding_rule!( 0b01110_100 [v,v] => a, b, result, data, strtbl ),
            Statement::Expression{ op: LessOrEqualSigned(Variable(a),Undefined), result } => encoding_rule!( 0b01110_101 [v,u] => a, result, data, strtbl ),
            Statement::Expression{ op: LessOrEqualSigned(Undefined,Constant(b)), result } => encoding_rule!( 0b01110_110 [u,c] => b, result, data, strtbl ),
            Statement::Expression{ op: LessOrEqualSigned(Undefined,Variable(b)), result } => encoding_rule!( 0b01110_111 [u,v] => b, result, data, strtbl ),
            Statement::Expression{ op: LessOrEqualSigned(Undefined,Undefined), result } => {
                Self::encode_statement(Statement::Expression{ op: Move(Undefined), result: result },data,strtbl)?;
            }

            // LessUnsigned: 0b01111---
            Statement::Expression{ op: LessUnsigned(Constant(a),Constant(b)), result } => encoding_rule!( 0b01111_000 [c,c] => a, b, result, data, strtbl ),
            Statement::Expression{ op: LessUnsigned(Constant(a),Variable(b)), result } => encoding_rule!( 0b01111_001 [c,v] => a, b, result, data, strtbl ),
            Statement::Expression{ op: LessUnsigned(Constant(a),Undefined), result } => encoding_rule!( 0b01111_010 [c,u] => a, result, data, strtbl ),
            Statement::Expression{ op: LessUnsigned(Variable(a),Constant(b)), result } => encoding_rule!( 0b01111_011 [v,c] => a, b, result, data, strtbl ),
            Statement::Expression{ op: LessUnsigned(Variable(a),Variable(b)), result } => encoding_rule!( 0b01111_100 [v,v] => a, b, result, data, strtbl ),
            Statement::Expression{ op: LessUnsigned(Variable(a),Undefined), result } => encoding_rule!( 0b01111_101 [v,u] => a, result, data, strtbl ),
            Statement::Expression{ op: LessUnsigned(Undefined,Constant(b)), result } => encoding_rule!( 0b01111_110 [u,c] => b, result, data, strtbl ),
            Statement::Expression{ op: LessUnsigned(Undefined,Variable(b)), result } => encoding_rule!( 0b01111_111 [u,v] => b, result, data, strtbl ),
            Statement::Expression{ op: LessUnsigned(Undefined,Undefined), result } => {
                Self::encode_statement(Statement::Expression{ op: Move(Undefined), result: result },data,strtbl)?;
            }

            // LessSigned: 0b10000---
            Statement::Expression{ op: LessSigned(Constant(a),Constant(b)), result } => encoding_rule!( 0b10000_000 [c,c] => a, b, result, data, strtbl ),
            Statement::Expression{ op: LessSigned(Constant(a),Variable(b)), result } => encoding_rule!( 0b10000_001 [c,v] => a, b, result, data, strtbl ),
            Statement::Expression{ op: LessSigned(Constant(a),Undefined), result } => encoding_rule!( 0b10000_010 [c,u] => a, result, data, strtbl ),
            Statement::Expression{ op: LessSigned(Variable(a),Constant(b)), result } => encoding_rule!( 0b10000_011 [v,c] => a, b, result, data, strtbl ),
            Statement::Expression{ op: LessSigned(Variable(a),Variable(b)), result } => encoding_rule!( 0b10000_100 [v,v] => a, b, result, data, strtbl ),
            Statement::Expression{ op: LessSigned(Variable(a),Undefined), result } => encoding_rule!( 0b10000_101 [v,u] => a, result, data, strtbl ),
            Statement::Expression{ op: LessSigned(Undefined,Constant(b)), result } => encoding_rule!( 0b10000_110 [u,c] => b, result, data, strtbl ),
            Statement::Expression{ op: LessSigned(Undefined,Variable(b)), result } => encoding_rule!( 0b10000_111 [u,v] => b, result, data, strtbl ),
            Statement::Expression{ op: LessSigned(Undefined,Undefined), result } => {
                Self::encode_statement(Statement::Expression{ op: Move(Undefined), result: result },data,strtbl)?;
            }

            // ZeroExtend: 0b1000100- <size, leb128> <a>
            Statement::Expression{ op: ZeroExtend(sz,Constant(a)), result } => {
                data.write(&[0b10001000])?;
                leb128::write::unsigned(data,sz as u64)?;
                Self::encode_constant(a,data)?;
                Self::encode_variable(result,data,strtbl)?;
            }
            Statement::Expression{ op: ZeroExtend(sz,Variable(a)), result } => {
                data.write(&[0b10001001])?;
                leb128::write::unsigned(data,sz as u64)?;
                Self::encode_variable(a,data,strtbl)?;
                Self::encode_variable(result,data,strtbl)?;
            }
            Statement::Expression{ op: ZeroExtend(_,Undefined), result } => {
                Self::encode_statement(Statement::Expression{ op: Move(Undefined), result: result },data,strtbl)?;
            }

            // SignExtend: 0b1000101- <size, leb128> <a>
            Statement::Expression{ op: SignExtend(sz,Constant(a)), result } => {
                data.write(&[0b10001010])?;
                leb128::write::unsigned(data,sz as u64)?;
                Self::encode_constant(a,data)?;
                Self::encode_variable(result,data,strtbl)?;
            }
            Statement::Expression{ op: SignExtend(sz,Variable(a)), result } => {
                data.write(&[0b10001011])?;
                leb128::write::unsigned(data,sz as u64)?;
                Self::encode_variable(a,data,strtbl)?;
                Self::encode_variable(result,data,strtbl)?;
            }
            Statement::Expression{ op: SignExtend(_,Undefined), result } => {
                Self::encode_statement(Statement::Expression{ op: Move(Undefined), result: result },data,strtbl)?;
            }

            // Move: 0b1000110- <a>
            Statement::Expression{ op: Move(Constant(a)), result } => {
                data.write(&[0b10001100])?;
                Self::encode_constant(a,data)?;
                Self::encode_variable(result,data,strtbl)?;
            }
            Statement::Expression{ op: Move(Variable(a)), result } => {
                data.write(&[0b10001101])?;
                Self::encode_variable(a,data,strtbl)?;
                Self::encode_variable(result,data,strtbl)?;
            }

            // Move Undefined: 0b10001110
            Statement::Expression{ op: Move(Undefined), result } => {
                data.write(&[0b10001110])?;
                Self::encode_variable(result,data,strtbl)?;
            }

            // Initialize: 0b10001111 <name, leb128> <size, leb128>
            Statement::Expression{ op: Initialize(name,sz), result } => {
                data.write(&[0b10001111])?;
                leb128::write::unsigned(data,Self::encode_str(name,strtbl))?;
                leb128::write::unsigned(data,sz as u64)?;
                Self::encode_variable(result,data,strtbl)?;
            }

            // Select: 0b100100-- <size, leb128> <start> <a>
            Statement::Expression{ op: Select(sz,Constant(start),Constant(src)), result } => {
                data.write(&[0b10010000])?;
                leb128::write::unsigned(data,sz as u64)?;
                Self::encode_constant(start,data)?;
                Self::encode_constant(src,data)?;
                Self::encode_variable(result,data,strtbl)?;
            }
            Statement::Expression{ op: Select(sz,Constant(start),Variable(src)), result } => {
                data.write(&[0b10010001])?;
                leb128::write::unsigned(data,sz as u64)?;
                Self::encode_constant(start,data)?;
                Self::encode_variable(src,data,strtbl)?;
                Self::encode_variable(result,data,strtbl)?;
            }
            Statement::Expression{ op: Select(sz,Variable(start),Constant(src)), result } => {
                data.write(&[0b10010010])?;
                leb128::write::unsigned(data,sz as u64)?;
                Self::encode_variable(start,data,strtbl)?;
                Self::encode_constant(src,data)?;
                Self::encode_variable(result,data,strtbl)?;
            }
            Statement::Expression{ op: Select(sz,Variable(start),Variable(src)), result } => {
                data.write(&[0b10010011])?;
                leb128::write::unsigned(data,sz as u64)?;
                Self::encode_variable(start,data,strtbl)?;
                Self::encode_variable(src,data,strtbl)?;
                Self::encode_variable(result,data,strtbl)?;
            }
            Statement::Expression{ op: Select(_,_,Undefined), result } => {
                Self::encode_statement(Statement::Expression{ op: Move(Undefined), result: result },data,strtbl)?;
            }
            Statement::Expression{ op: Select(_,Undefined,_), result } => {
                Self::encode_statement(Statement::Expression{ op: Move(Undefined), result: result },data,strtbl)?;
            }

            // Load: 0b100101e- <region, leb128> <size, leb128> <a>
            Statement::Expression{ op: Load(region,Endianess::Little,bytes,Constant(addr)), result } => {
                data.write(&[0b10010100])?;
                leb128::write::unsigned(data,Self::encode_str(region,strtbl))?;
                leb128::write::unsigned(data,bytes as u64)?;
                Self::encode_constant(addr,data)?;
                Self::encode_variable(result,data,strtbl)?;
            }
            Statement::Expression{ op: Load(region,Endianess::Big,bytes,Constant(addr)), result } => {
                data.write(&[0b10010110])?;
                leb128::write::unsigned(data,Self::encode_str(region,strtbl))?;
                leb128::write::unsigned(data,bytes as u64)?;
                Self::encode_constant(addr,data)?;
                Self::encode_variable(result,data,strtbl)?;
            }
            Statement::Expression{ op: Load(region,Endianess::Little,bytes,Variable(addr)), result } => {
                data.write(&[0b10010101])?;
                leb128::write::unsigned(data,Self::encode_str(region,strtbl))?;
                leb128::write::unsigned(data,bytes as u64)?;
                Self::encode_variable(addr,data,strtbl)?;
                Self::encode_variable(result,data,strtbl)?;
            }
            Statement::Expression{ op: Load(region,Endianess::Big,bytes,Variable(addr)), result } => {
                data.write(&[0b10010111])?;
                leb128::write::unsigned(data,Self::encode_str(region,strtbl))?;
                leb128::write::unsigned(data,bytes as u64)?;
                Self::encode_variable(addr,data,strtbl)?;
                Self::encode_variable(result,data,strtbl)?;
            }

            // Phi2: 0b10011000 <a> <b>
            Statement::Expression{ op: Phi(Variable(a),Variable(b),Undefined), result } |
            Statement::Expression{ op: Phi(Variable(a),Undefined,Variable(b)), result } |
            Statement::Expression{ op: Phi(Undefined,Variable(a),Variable(b)), result } => {
                data.write(&[0b10011000])?;
                Self::encode_variable(a,data,strtbl)?;
                Self::encode_variable(b,data,strtbl)?;
                Self::encode_variable(result,data,strtbl)?;
                data.write(&[0,0,0])?;
            }

            // Phi3: 0b10011001 <a> <b> <c>
            Statement::Expression{ op: Phi(Variable(a),Variable(b),Variable(c)), result } => {
                data.write(&[0b10011001])?;
                Self::encode_variable(a,data,strtbl)?;
                Self::encode_variable(b,data,strtbl)?;
                Self::encode_variable(c,data,strtbl)?;
                Self::encode_variable(result,data,strtbl)?;
            }

            // Call: 0b10011010 <stub, leb128>
            Statement::Call{ function: CallTarget::External(name) } => {
                data.write(&[0b10011010])?;
                leb128::write::unsigned(data,Self::encode_str(name,strtbl))?;
            }

            // Call: 0b10011011 <uuid, leb128>
            Statement::Call{ function: CallTarget::Function(uuid) } => {
                data.write(&[0b10011011])?;
                leb128::write::unsigned(data,Self::encode_str(uuid.to_string().into(),strtbl))?;
            }

            // IndirectCall: 0b1001110- <a>
            Statement::IndirectCall{ target: Constant(tgt) } => {
                data.write(&[0b10011100])?;
                Self::encode_constant(tgt,data)?;
            }
            Statement::IndirectCall{ target: Variable(tgt) } => {
                data.write(&[0b10011101])?;
                Self::encode_variable(tgt,data,strtbl)?;
            }

            // IndirectCall Undefined: 0b10011110
            Statement::IndirectCall{ target: Undefined } => {
                data.write(&[0b10011110])?;
            }

            // Phi0: 0b10011111
            Statement::Expression{ op: Phi(Undefined,Undefined,Undefined), result } => {
                data.write(&[0b10011111])?;
                Self::encode_variable(result,data,strtbl)?;
                data.write(&[0,0,0,0,0,0,0,0,0])?;
            }

            // Store: 0b1010e--- <region, leb128> <size, leb128> <addr> <val>
            Statement::Store{ region, bytes, endianess, address: Constant(addr), value: Constant(value) } => {
                data.write(&[0b10100000 | if endianess == Endianess::Little { 0 } else { 0b1000 }])?;
                leb128::write::unsigned(data,Self::encode_str(region,strtbl))?;
                leb128::write::unsigned(data,bytes as u64)?;
                Self::encode_constant(addr,data)?;
                Self::encode_constant(value,data)?;
            }
            Statement::Store{ region, bytes, endianess, address: Constant(addr), value: Variable(value) } => {
                data.write(&[0b10100001 | if endianess == Endianess::Little { 0 } else { 0b1000 }])?;
                leb128::write::unsigned(data,Self::encode_str(region,strtbl))?;
                leb128::write::unsigned(data,bytes as u64)?;
                Self::encode_constant(addr,data)?;
                Self::encode_variable(value,data,strtbl)?;
            }
            Statement::Store{ region, bytes, endianess, address: Constant(addr), value: Undefined } => {
                data.write(&[0b10100010 | if endianess == Endianess::Little { 0 } else { 0b1000 }])?;
                leb128::write::unsigned(data,Self::encode_str(region,strtbl))?;
                leb128::write::unsigned(data,bytes as u64)?;
                Self::encode_constant(addr,data)?;
            }
            Statement::Store{ region, bytes, endianess, address: Variable(addr), value: Constant(value) } => {
                data.write(&[0b10100011 | if endianess == Endianess::Little { 0 } else { 0b1000 }])?;
                leb128::write::unsigned(data,Self::encode_str(region,strtbl))?;
                leb128::write::unsigned(data,bytes as u64)?;
                Self::encode_variable(addr,data,strtbl)?;
                Self::encode_constant(value,data)?;
            }
            Statement::Store{ region, bytes, endianess, address: Variable(addr), value: Variable(value) } => {
                data.write(&[0b10100100 | if endianess == Endianess::Little { 0 } else { 0b1000 }])?;
                leb128::write::unsigned(data,Self::encode_str(region,strtbl))?;
                leb128::write::unsigned(data,bytes as u64)?;
                Self::encode_variable(addr,data,strtbl)?;
                Self::encode_variable(value,data,strtbl)?;
            }
            Statement::Store{ region, bytes, endianess, address: Variable(addr), value: Undefined } => {
                data.write(&[0b10100101 | if endianess == Endianess::Little { 0 } else { 0b1000 }])?;
                leb128::write::unsigned(data,Self::encode_str(region,strtbl))?;
                leb128::write::unsigned(data,bytes as u64)?;
                Self::encode_variable(addr,data,strtbl)?;
            }
            Statement::Store{ region, bytes, endianess, address: Undefined, value: Constant(value) } => {
                data.write(&[0b10100110 | if endianess == Endianess::Little { 0 } else { 0b1000 }])?;
                leb128::write::unsigned(data,Self::encode_str(region,strtbl))?;
                leb128::write::unsigned(data,bytes as u64)?;
                Self::encode_constant(value,data)?;
            }
            Statement::Store{ region, bytes, endianess, address: Undefined, value: Variable(value) } => {
                data.write(&[0b10100111 | if endianess == Endianess::Little { 0 } else { 0b1000 }])?;
                leb128::write::unsigned(data,Self::encode_str(region,strtbl))?;
                leb128::write::unsigned(data,bytes as u64)?;
                Self::encode_variable(value,data,strtbl)?;
            }
            Statement::Store{ address: Undefined, value: Undefined,.. } => { /* NOP */ }

            // Return: 0b10110000
            Statement::Return => {
                data.write(&[0b10110000])?;
            }

            // Load Undefined: 0b1011 001e <region, leb128> <size, leb128>
            Statement::Expression{ op: Load(region,endianess,bytes,Undefined), result } => {
                data.write(&[0b1011_0010 | if endianess == Endianess::Little { 0 } else { 0b1 }])?;
                leb128::write::unsigned(data,Self::encode_str(region,strtbl))?;
                leb128::write::unsigned(data,bytes as u64)?;
                Self::encode_variable(result,data,strtbl)?;
            }

            // Phi1: 0b10110100 <a>
            Statement::Expression{ op: Phi(Variable(a),Undefined,Undefined), result } => {
                data.write(&[0b10110100])?;
                Self::encode_variable(a,data,strtbl)?;
                Self::encode_variable(result,data,strtbl)?;
                data.write(&[0,0,0,0,0,0])?;
            }

            Statement::Expression{ op: Phi(_,_,_),.. } => {
                return Err(format!("Internal error: invalid Phi expression {:?}",stmt).into());
            }
        }

        Ok(())
    }

    // const: <len, pow2><leb128 value>
    fn encode_constant<W: Write>(c: Constant, data: &mut W) -> Result<()> {
        let Constant{ value, bits } = c;
        leb128::write::unsigned(data,bits as u64)?;
        leb128::write::unsigned(data,value)?;
        Ok(())
    }

    // var: <name, leb128 str idx>, <subscript, leb128 + 1>, <len, pow2>
    fn encode_variable<W: Write>(c: Variable, data: &mut W, strtbl: &mut Strings) -> Result<()> {
        let Variable{ name, subscript, bits } = c;
        leb128::write::unsigned(data,Self::encode_str(name,strtbl))?;
        leb128::write::unsigned(data,if let Some(subscript) = subscript { subscript as u64 + 1 } else { 0 })?;
        leb128::write::unsigned(data,bits as u64)?;
        Ok(())
    }

    fn encode_str(s: Str, strtbl: &mut Strings) -> u64 {
        strtbl.insert(s) as u64
    }

    fn decode_statement<R: Read>(&self, data: &mut R) -> Result<Statement<Value>> {
        let mut opcode = [0u8; 1];
        data.read_exact(&mut opcode)?;

        match opcode[0] {
            0b00000_000...0b00001_000 => {
                let (a,b) = self.decode_arguments(opcode[0] & 0b111,data)?;
                let res = self.decode_variable(data)?;
                let stmt = Statement::Expression{ op: Operation::Add(a,b), result: res };

                Ok(stmt)
            }
            0b00001_000...0b00010_000 => {
                let (a,b) = self.decode_arguments(opcode[0] & 0b111,data)?;
                let res = self.decode_variable(data)?;
                let stmt = Statement::Expression{ op: Operation::Subtract(a,b), result: res };

                Ok(stmt)
            }
            0b00010_000...0b00011_000 => {
                let (a,b) = self.decode_arguments(opcode[0] & 0b111,data)?;
                let res = self.decode_variable(data)?;
                let stmt = Statement::Expression{ op: Operation::Multiply(a,b), result: res };

                Ok(stmt)
            }
            0b00011_000...0b00100_000 => {
                let (a,b) = self.decode_arguments(opcode[0] & 0b111,data)?;
                let res = self.decode_variable(data)?;
                let stmt = Statement::Expression{ op: Operation::DivideUnsigned(a,b), result: res };

                Ok(stmt)
            }
            0b00100_000...0b00101_000 => {
                let (a,b) = self.decode_arguments(opcode[0] & 0b111,data)?;
                let res = self.decode_variable(data)?;
                let stmt = Statement::Expression{ op: Operation::DivideSigned(a,b), result: res };

                Ok(stmt)
            }
            0b00101_000...0b00110_000 => {
                let (a,b) = self.decode_arguments(opcode[0] & 0b111,data)?;
                let res = self.decode_variable(data)?;
                let stmt = Statement::Expression{ op: Operation::ShiftLeft(a,b), result: res };

                Ok(stmt)
            }
            0b00110_000...0b00111_000 => {
                let (a,b) = self.decode_arguments(opcode[0] & 0b111,data)?;
                let res = self.decode_variable(data)?;
                let stmt = Statement::Expression{ op: Operation::ShiftRightUnsigned(a,b), result: res };

                Ok(stmt)
            }
            0b00111_000...0b01000_000 => {
                let (a,b) = self.decode_arguments(opcode[0] & 0b111,data)?;
                let res = self.decode_variable(data)?;
                let stmt = Statement::Expression{ op: Operation::ShiftRightSigned(a,b), result: res };

                Ok(stmt)
            }
            0b01000_000...0b01001_000 => {
                let (a,b) = self.decode_arguments(opcode[0] & 0b111,data)?;
                let res = self.decode_variable(data)?;
                let stmt = Statement::Expression{ op: Operation::Modulo(a,b), result: res };

                Ok(stmt)
            }
            0b01001_000...0b01010_000 => {
                let (a,b) = self.decode_arguments(opcode[0] & 0b111,data)?;
                let res = self.decode_variable(data)?;
                let stmt = Statement::Expression{ op: Operation::And(a,b), result: res };

                Ok(stmt)
            }
            0b01010_000...0b01011_000 => {
                let (a,b) = self.decode_arguments(opcode[0] & 0b111,data)?;
                let res = self.decode_variable(data)?;
                let stmt = Statement::Expression{ op: Operation::InclusiveOr(a,b), result: res };

                Ok(stmt)
            }
            0b01011_000...0b01100_000 => {
                let (a,b) = self.decode_arguments(opcode[0] & 0b111,data)?;
                let res = self.decode_variable(data)?;
                let stmt = Statement::Expression{ op: Operation::ExclusiveOr(a,b), result: res };

                Ok(stmt)
            }
            0b01100_000...0b01101_000 => {
                let (a,b) = self.decode_arguments(opcode[0] & 0b111,data)?;
                let res = self.decode_variable(data)?;
                let stmt = Statement::Expression{ op: Operation::Equal(a,b), result: res };

                Ok(stmt)
            }
            0b01101_000...0b01110_000 => {
                let (a,b) = self.decode_arguments(opcode[0] & 0b111,data)?;
                let res = self.decode_variable(data)?;
                let stmt = Statement::Expression{ op: Operation::LessOrEqualUnsigned(a,b), result: res };

                Ok(stmt)
            }
            0b01110_000...0b01111_000 => {
                let (a,b) = self.decode_arguments(opcode[0] & 0b111,data)?;
                let res = self.decode_variable(data)?;
                let stmt = Statement::Expression{ op: Operation::LessOrEqualSigned(a,b), result: res };

                Ok(stmt)
            }
            0b01111_000...0b10000_000 => {
                let (a,b) = self.decode_arguments(opcode[0] & 0b111,data)?;
                let res = self.decode_variable(data)?;
                let stmt = Statement::Expression{ op: Operation::LessUnsigned(a,b), result: res };

                Ok(stmt)
            }
            0b10000_000...0b10000_111 => {
                let (a,b) = self.decode_arguments(opcode[0] & 0b111,data)?;
                let res = self.decode_variable(data)?;
                let stmt = Statement::Expression{ op: Operation::LessSigned(a,b), result: res };

                Ok(stmt)
            }

            // zext  1000 100- <size, leb128> <a>
            0b1000_1000 | 0b1000_1001 => {
                let sz = leb128::read::unsigned(data)? as usize;
                let a = if opcode[0] & 1 == 0 {
                    Value::Constant(self.decode_constant(data)?)
                } else {
                    Value::Variable(self.decode_variable(data)?)
                };
                let res = self.decode_variable(data)?;
                let stmt = Statement::Expression{ op: Operation::ZeroExtend(sz,a), result: res };

                Ok(stmt)
            }

            // sext  1000 101- <size, leb128> <a>
            0b1000_1010 | 0b1000_1011 => {
                let sz = leb128::read::unsigned(data)? as usize;
                let a = if opcode[0] & 1 == 0 {
                    Value::Constant(self.decode_constant(data)?)
                } else {
                    Value::Variable(self.decode_variable(data)?)
                };
                let res = self.decode_variable(data)?;
                let stmt = Statement::Expression{ op: Operation::SignExtend(sz,a), result: res };

                Ok(stmt)
            }

            // mov   1000 110- <a>
            0b1000_1100 | 0b1000_1101 => {
                let a = if opcode[0] & 1 == 0 {
                    Value::Constant(self.decode_constant(data)?)
                } else {
                    Value::Variable(self.decode_variable(data)?)
                };
                let res = self.decode_variable(data)?;
                let stmt = Statement::Expression{ op: Operation::Move(a), result: res };

                Ok(stmt)
            }

            // movu  1000 1110
            0b1000_1110 => {
                let res = self.decode_variable(data)?;
                let stmt = Statement::Expression{ op: Operation::Move(Value::Undefined), result: res };

                Ok(stmt)
            }

            // init  1000 1111 <name, leb128> <size, leb128>
            0b1000_1111 => {
                let name = leb128::read::unsigned(data)? as usize;
                let sz = leb128::read::unsigned(data)? as usize;
                let res = self.decode_variable(data)?;
                let stmt = Statement::Expression{
                    op: Operation::Initialize(self.strings.get(name),sz),
                    result: res
                };

                Ok(stmt)
            }

            // sel   1001 00-- <size, leb128> <start> <a>
            0b1001_0000...0b1001_0011 => {
                let sz = leb128::read::unsigned(data)? as usize;
                let start = if opcode[0] & 0b10 == 0 {
                    Value::Constant(self.decode_constant(data)?)
                } else {
                    Value::Variable(self.decode_variable(data)?)
                };
                let val = if opcode[0] & 0b1 == 0 {
                    Value::Constant(self.decode_constant(data)?)
                } else {
                    Value::Variable(self.decode_variable(data)?)
                };
                let res = self.decode_variable(data)?;
                let stmt = Statement::Expression{ op: Operation::Select(sz,start,val), result: res };

                Ok(stmt)
            }

            // load  1001 01e- <region, leb128> <size, leb128> <a>
            0b1001_0100...0b1001_0111 => {
                let reg = leb128::read::unsigned(data)? as usize;
                let sz = leb128::read::unsigned(data)? as usize;
                let val = if opcode[0] & 0b1 == 0 {
                    Value::Constant(self.decode_constant(data)?)
                } else {
                    Value::Variable(self.decode_variable(data)?)
                };
                let endianess = if opcode[0] & 0b10 == 0 {
                    Endianess::Little
                } else {
                    Endianess::Big
                };
                let res = self.decode_variable(data)?;
                let stmt = Statement::Expression{
                    op: Operation::Load(self.strings.get(reg),endianess,sz,val),
                    result: res
                };

                Ok(stmt)
            }

            // phi2  1001 1000 <a, var> <b, var> 0x000000
            0b1001_1000 => {
                let a = Value::Variable(self.decode_variable(data)?);
                let b = Value::Variable(self.decode_variable(data)?);
                let res = self.decode_variable(data)?;
                let stmt = Statement::Expression{
                    op: Operation::Phi(a,b,Value::Undefined),
                    result: res
                };

                data.read(&mut [0;3])?;
                Ok(stmt)
            }

            // phi3  1001 1001 <a, var> <b, var> <c, var>
            0b1001_1001 => {
                let a = Value::Variable(self.decode_variable(data)?);
                let b = Value::Variable(self.decode_variable(data)?);
                let c = Value::Variable(self.decode_variable(data)?);
                let res = self.decode_variable(data)?;
                let stmt = Statement::Expression{
                    op: Operation::Phi(a,b,c),
                    result: res
                };

                Ok(stmt)
            }

            // call  1001 1010 <stub, leb128>
            // call  1001 1011 <uuid, leb128>
            0b1001_1010 | 0b1001_1011 => {
                let s = leb128::read::unsigned(data)? as usize;
                let stmt = Statement::Call{
                    function: if opcode[0] & 1 == 0 {
                        CallTarget::External(self.strings.get(s))
                    } else {
                        let s = &self.strings.get(s);
                        let uu = Uuid::parse_str(s)
                            .map_err(|_| format!("Internal error: invalid uuid '{}'",s))?;
                        CallTarget::Function(uu)
                    }
                };

                Ok(stmt)
            }

            // icall 1001 110- <a>
            0b1001_1100...0b1001_1101 => {
                let val = if opcode[0] & 0b1 == 0 {
                    Value::Constant(self.decode_constant(data)?)
                } else {
                    Value::Variable(self.decode_variable(data)?)
                };
                let stmt = Statement::IndirectCall{ target: val };

                Ok(stmt)
            }

            0b1001_1110 => {
                let stmt = Statement::IndirectCall{ target: Value::Undefined };

                Ok(stmt)
            }

            // phi0  1001 1111
            0b1001_1111 => {
                let res = self.decode_variable(data)?;
                let stmt = Statement::Expression{
                    op: Operation::Phi(Value::undef(),Value::undef(),Value::undef()),
                    result: res
                };

                data.read(&mut [0;9])?;
                Ok(stmt)
            }

            // store 1010 e--- <region, leb128> <size, leb128> <addr> <val>
            0b1010_0000...0b1010_1111 => {
                let reg = leb128::read::unsigned(data)? as usize;
                let sz = leb128::read::unsigned(data)? as usize;
                let (addr,val) = self.decode_arguments(opcode[0] & 0b111,data)?;
                let endianess = if opcode[0] & 0b1000 == 0 {
                    Endianess::Little
                } else {
                    Endianess::Big
                };
                let stmt = Statement::Store{
                    region: self.strings.get(reg),
                    bytes: sz,
                    endianess: endianess,
                    address: addr,
                    value: val,
                };

                Ok(stmt)
            }

            // ret   1011 0000
            0b1011_0000 => {
                let stmt = Statement::Return;

                Ok(stmt)
            }

            // loadu: 0b1011001e <region, leb128> <size, leb128>
            0b1011_0010 | 0b1011_0011 => {
                let reg = leb128::read::unsigned(data)? as usize;
                let sz = leb128::read::unsigned(data)? as usize;
                let endianess = if opcode[0] & 1 == 0 {
                    Endianess::Little
                } else {
                    Endianess::Big
                };
                let res = self.decode_variable(data)?;
                let stmt = Statement::Expression{
                    op: Operation::Load(self.strings.get(reg),endianess,sz,Value::Undefined),
                    result: res
                };

                Ok(stmt)
            }

            // phi1: 0b10110100 <a>
            0b10110100 => {
                let a = Value::Variable(self.decode_variable(data)?);
                let res = self.decode_variable(data)?;
                let stmt = Statement::Expression{
                    op: Operation::Phi(a,Value::undef(),Value::undef()),
                    result: res
                };

                data.read(&mut [0;6])?;
                Ok(stmt)
            }

            _ => Err(format!("Internal error: invalid bitcode {:b}",opcode[0]).into()),
        }
    }

    //  1\2  c   v   u
    //   c|000 001 010
    //   v|011 100 101
    //   u|110 111 xxx
    fn decode_arguments<R: Read>(&self, code: u8, data: &mut R) -> Result<(Value,Value)> {
        match code {
            0b000 => {
                let a = self.decode_constant(data)?;
                let b = self.decode_constant(data)?;
                Ok((Value::Constant(a),Value::Constant(b)))
            }
            0b001 => {
                let a = self.decode_constant(data)?;
                let b = self.decode_variable(data)?;
                Ok((Value::Constant(a),Value::Variable(b)))
            }
            0b010 => {
                let a = self.decode_constant(data)?;
                Ok((Value::Constant(a),Value::Undefined))
            }
            0b011 => {
                let a = self.decode_variable(data)?;
                let b = self.decode_constant(data)?;
                Ok((Value::Variable(a),Value::Constant(b)))
            }
            0b100 => {
                let a = self.decode_variable(data)?;
                let b = self.decode_variable(data)?;
                Ok((Value::Variable(a),Value::Variable(b)))
            }
            0b101 => {
                let a = self.decode_variable(data)?;
                Ok((Value::Variable(a),Value::Undefined))
            }
            0b110 => {
                let a = self.decode_constant(data)?;
                Ok((Value::Undefined,Value::Constant(a)))
            }
            0b111 => {
                let a = self.decode_variable(data)?;
                Ok((Value::Undefined,Value::Variable(a)))
            }
            _ => Err(format!("internal error: impossible argument code {:b}",code).into())
        }
    }

    // const: <len, pow2><leb128 value>
    fn decode_constant<R: Read>(&self, data: &mut R) -> Result<Constant> {
        let bits = leb128::read::unsigned(data)?;
        let value = leb128::read::unsigned(data)?;

        Ok(Constant{ bits: bits as usize, value: value })
    }

    // var: <name, leb128 str idx>, <subscript, leb128 + 1>, <len, pow2>
    fn decode_variable<R: Read>(&self, data: &mut R) -> Result<Variable> {
        let name = leb128::read::unsigned(data)?;
        let subscript = leb128::read::unsigned(data)?;
        let bits = leb128::read::unsigned(data)?;
        let var = Variable{
            name: self.strings.get(name as usize),
            subscript: if subscript == 0 { None } else { Some(subscript as usize - 1)  },
            bits: bits as usize,
        };


        Ok(var)
    }

    pub fn iter<'a>(&'a self) -> BitcodeIter<'a> {
        BitcodeIter{
            cursor: Cursor::new(&self.data),
            bitcode: self,
        }
    }

    pub fn iter_range<'a>(&'a self, rgn: Range<usize>) -> BitcodeIter<'a> {
        BitcodeIter{
            cursor: Cursor::new(&self.data[rgn]),
            bitcode: self,
        }
    }

    pub fn num_bytes(&self) -> usize {
        self.data.len()
    }

    pub fn num_strings(&self) -> usize {
        self.strings.len()
    }
}

pub struct BitcodeIter<'a> {
    cursor: Cursor<&'a [u8]>,
    bitcode: &'a Bitcode,
}

impl<'a> Iterator for BitcodeIter<'a> {
    type Item = Statement<Value>;

    fn next(&mut self) -> Option<Self::Item> {
        self.bitcode.decode_statement(&mut self.cursor).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    quickcheck! {
        fn round_trip(v: Vec<Statement<Value>>) -> bool {
            debug!("in: {:?}",v);
            match Bitcode::new(v.clone()) {
                Ok(bt) => {
                    debug!("{:?}",bt);
                    let w = bt.iter().collect::<Vec<_>>();
                    debug!("decoded: {:?}",w);
                    v == w
                }
                Err(s) => {
                    debug!("err: {}",s);
                    false
                }
            }
        }
    }

    #[test]
    fn rewrite_equal_size() {
        use neo::Operation::Add;
        let s1 = Statement::Expression{
            op: Add(Value::val(42,32).unwrap(),Value::var("i",32,None).unwrap()),
            result: Variable::new("i",32,None).unwrap()
        };
        let s2 = Statement::Expression{
            op: Add(Value::val(42,32).unwrap(),Value::var("i",32,None).unwrap()),
            result: Variable::new("i",32,None).unwrap()
        };
        let s3 = Statement::Expression{
            op: Add(Value::val(42,32).unwrap(),Value::var("i",32,None).unwrap()),
            result: Variable::new("i",32,None).unwrap()
        };
        let mut bitcode = Bitcode::default();

        let _ = bitcode.append(vec![s1]).unwrap();
        let rgn = bitcode.append(vec![s2]).unwrap();
        let _ = bitcode.append(vec![s3]).unwrap();

        let new_rgn = bitcode.rewrite(rgn.clone(), |stmt| {
            match stmt {
                &mut Statement::Expression{ op: Add(Value::Constant(ref mut a),Value::Variable(ref mut b)), ref mut result } => {
                    *a = Constant::new(43,32).unwrap();
                    *b = Variable::new("i",32,Some(1)).unwrap();
                    *result = Variable::new("i",32,Some(2)).unwrap();
                }
                _ => { unreachable!() }
            }

            Ok(())
        }).unwrap();

        assert_eq!(rgn, new_rgn);
        for (idx,stmt) in bitcode.iter().enumerate() {
            match stmt {
                Statement::Expression{ op: Add(Value::Constant(a),Value::Variable(b)), result } => {
                    if idx == 0 || idx == 2 {
                        assert_eq!(a, Constant::new(42,32).unwrap());
                        assert_eq!(b, Variable::new("i",32,None).unwrap());
                        assert_eq!(result, Variable::new("i",32,None).unwrap());
                    } else if idx == 1 {
                        assert_eq!(a, Constant::new(43,32).unwrap());
                        assert_eq!(b, Variable::new("i",32,Some(1)).unwrap());
                        assert_eq!(result, Variable::new("i",32,Some(2)).unwrap());
                    } else {
                        unreachable!()
                    }
                }
                _ => { unreachable!() }
            }
        }
    }

    #[test]
    fn rewrite_smaller_size() {
        use neo::Operation::Add;
        let s1 = Statement::Expression{
            op: Add(Value::val(42,32).unwrap(),Value::var("i",32,None).unwrap()),
            result: Variable::new("i",32,None).unwrap()
        };
        let s2 = Statement::Expression{
            op: Add(Value::val(42,32).unwrap(),Value::var("i",32,None).unwrap()),
            result: Variable::new("i",32,None).unwrap()
        };
        let s3 = Statement::Expression{
            op: Add(Value::val(42,32).unwrap(),Value::var("i",32,None).unwrap()),
            result: Variable::new("i",32,None).unwrap()
        };
        let mut bitcode = Bitcode::default();

        let _ = bitcode.append(vec![s1]).unwrap();
        let rgn = bitcode.append(vec![s2]).unwrap();
        let _ = bitcode.append(vec![s3]).unwrap();

        let new_rgn = bitcode.rewrite(rgn.clone(), |stmt| {
            *stmt = Statement::Return;

            Ok(())
        }).unwrap();

        assert_eq!(rgn.start, new_rgn.start);
        assert!(rgn.end > new_rgn.end);
        for (idx,stmt) in bitcode.iter().enumerate() {
            match stmt {
                Statement::Expression{ op: Add(Value::Constant(a),Value::Variable(b)), result } => {
                    assert_eq!(a, Constant::new(42,32).unwrap());
                    assert_eq!(b, Variable::new("i",32,None).unwrap());
                    assert_eq!(result, Variable::new("i",32,None).unwrap());
                    assert!(idx == 0 || idx == 2);
                }
                Statement::Return => {
                    assert_eq!(idx, 1);
                }
                _ => { unreachable!() }
            }
        }
    }

    #[test]
    fn rewrite_larger_size() {
        use neo::Operation::Add;
        let s1 = Statement::Expression{
            op: Add(Value::val(42,32).unwrap(),Value::var("i",32,None).unwrap()),
            result: Variable::new("i",32,None).unwrap()
        };
        let s2 = Statement::Return;
        let s3 = Statement::Expression{
            op: Add(Value::val(42,32).unwrap(),Value::var("i",32,None).unwrap()),
            result: Variable::new("i",32,None).unwrap()
        };
        let mut bitcode = Bitcode::default();

        let _ = bitcode.append(vec![s1]).unwrap();
        let rgn = bitcode.append(vec![s2]).unwrap();
        let _ = bitcode.append(vec![s3]).unwrap();

        let new_rgn = bitcode.rewrite(rgn.clone(), |stmt| {
            *stmt = Statement::Expression{
                op: Add(Value::val(42,32).unwrap(),Value::var("i",32,None).unwrap()),
                result: Variable::new("i",32,None).unwrap()
            };

            Ok(())
        }).unwrap();

        assert_eq!(rgn.start, new_rgn.start);
        assert!(rgn.end < new_rgn.end);
        for (idx,stmt) in bitcode.iter().enumerate() {
            match stmt {
                Statement::Expression{ op: Add(Value::Constant(a),Value::Variable(b)), result } => {
                    assert_eq!(a, Constant::new(42,32).unwrap());
                    assert_eq!(b, Variable::new("i",32,None).unwrap());
                    assert_eq!(result, Variable::new("i",32,None).unwrap());
                }
                _ => { unreachable!() }
            }
        }
    }

    #[test]
    fn insert_mid() {
        use neo::Operation::Add;
        let s1 = Statement::Expression{
            op: Add(Value::val(42,32).unwrap(),Value::var("i",32,None).unwrap()),
            result: Variable::new("i",32,None).unwrap()
        };
        let s2 = Statement::Expression{
            op: Add(Value::val(42,32).unwrap(),Value::var("i",32,None).unwrap()),
            result: Variable::new("i",32,None).unwrap()
        };
        let mut bitcode = Bitcode::default();

        bitcode.append(vec![s1]);
        let rgn = bitcode.append(vec![s2]).unwrap();

        let new_rgn = bitcode.insert(rgn.start,vec![Statement::Return]).unwrap();

        assert_eq!(rgn.start, new_rgn.start);
        for (idx,stmt) in bitcode.iter().enumerate() {
            match stmt {
                Statement::Expression{ op: Add(Value::Constant(a),Value::Variable(b)), result } => {
                    assert_eq!(a, Constant::new(42,32).unwrap());
                    assert_eq!(b, Variable::new("i",32,None).unwrap());
                    assert_eq!(result, Variable::new("i",32,None).unwrap());
                    assert!(idx == 0 || idx == 2);
                }
                Statement::Return => {
                    assert_eq!(idx, 1);
                }
                _ => { unreachable!() }
            }
        }
    }

    #[test]
    fn insert_start() {
        use neo::Operation::Add;
        let s1 = Statement::Expression{
            op: Add(Value::val(42,32).unwrap(),Value::var("i",32,None).unwrap()),
            result: Variable::new("i",32,None).unwrap()
        };
        let mut bitcode = Bitcode::default();
        let rgn = bitcode.append(vec![s1]).unwrap();
        let new_rgn = bitcode.insert(rgn.start,vec![Statement::Return]).unwrap();

        assert_eq!(rgn.start, new_rgn.start);
        for (idx,stmt) in bitcode.iter().enumerate() {
            match stmt {
                Statement::Expression{ op: Add(Value::Constant(a),Value::Variable(b)), result } => {
                    assert_eq!(a, Constant::new(42,32).unwrap());
                    assert_eq!(b, Variable::new("i",32,None).unwrap());
                    assert_eq!(result, Variable::new("i",32,None).unwrap());
                    assert_eq!(idx, 1);
                }
                Statement::Return => {
                    assert_eq!(idx, 0);
                }
                _ => { unreachable!() }
            }
        }
    }

    #[test]
    fn insert_end() {
        use neo::Operation::Add;
        let s1 = Statement::Expression{
            op: Add(Value::val(42,32).unwrap(),Value::var("i",32,None).unwrap()),
            result: Variable::new("i",32,None).unwrap()
        };
        let mut bitcode = Bitcode::default();
        let rgn = bitcode.append(vec![s1]).unwrap();
        let new_rgn = bitcode.insert(rgn.end,vec![Statement::Return]).unwrap();

        assert_eq!(rgn.end, new_rgn.start);
        for (idx,stmt) in bitcode.iter().enumerate() {
            match stmt {
                Statement::Expression{ op: Add(Value::Constant(a),Value::Variable(b)), result } => {
                    assert_eq!(a, Constant::new(42,32).unwrap());
                    assert_eq!(b, Variable::new("i",32,None).unwrap());
                    assert_eq!(result, Variable::new("i",32,None).unwrap());
                    assert_eq!(idx, 0);
                }
                Statement::Return => {
                    assert_eq!(idx, 1);
                }
                _ => { unreachable!() }
            }
        }
    }
}

use disassembler::*;
use program::Program;
use layer::LayerIter;
use value::{Lvalue,Rvalue,Endianess};
use codegen::CodeGen;
use guard::Guard;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};

pub struct AvrState {
    flash_size: u16,
}

fn reg(st: &State<u16>, cap: &str) -> Lvalue {
    unimplemented!()
}

fn ioreg(st: &State<u16>, cap: &str) -> Lvalue {
    unimplemented!()
}

fn sram(off: &Rvalue) -> Lvalue {
    Lvalue::Memory{
        offset: Box::new(off.clone()),
        name: "sram".to_string(),
        endianess: Endianess::Big,
        bytes: 1
    }
}

fn flash(off: &Rvalue) -> Lvalue {
    Lvalue::Memory{
        offset: Box::new(off.clone()),
        name: "flash".to_string(),
        endianess: Endianess::Big,
        bytes: 2
    }
}

fn get_sp(cg: &mut CodeGen) -> Lvalue {
    unimplemented!()
}

fn set_sp(v: &Rvalue, cg: &mut CodeGen) {
    unimplemented!();
}

fn resolv(r: u16) -> Lvalue {
    unimplemented!()
}

static GLOBAL_AVR_TEMPVAR_COUNT: AtomicUsize = ATOMIC_USIZE_INIT;

fn new_temp(bits: usize) -> Lvalue {
    Lvalue::Variable{
        name: format!("__temp{}",GLOBAL_AVR_TEMPVAR_COUNT.fetch_add(1, Ordering::SeqCst)),
        width: bits as u16,
        subscript: None
    }
}

lazy_static! {
    static ref R1: Lvalue = Lvalue::Variable{ name: "r1".to_string(), width: 8, subscript: None };
    static ref R2: Lvalue = Lvalue::Variable{ name: "r2".to_string(), width: 8, subscript: None };
    static ref R3: Lvalue = Lvalue::Variable{ name: "r3".to_string(), width: 8, subscript: None };
    static ref R4: Lvalue = Lvalue::Variable{ name: "r4".to_string(), width: 8, subscript: None };
    static ref R5: Lvalue = Lvalue::Variable{ name: "r5".to_string(), width: 8, subscript: None };
    static ref R6: Lvalue = Lvalue::Variable{ name: "r6".to_string(), width: 8, subscript: None };
    static ref R7: Lvalue = Lvalue::Variable{ name: "r7".to_string(), width: 8, subscript: None };
    static ref R8: Lvalue = Lvalue::Variable{ name: "r8".to_string(), width: 8, subscript: None };
    static ref R9: Lvalue = Lvalue::Variable{ name: "r9".to_string(), width: 8, subscript: None };
    static ref R10: Lvalue = Lvalue::Variable{ name: "r10".to_string(), width: 8, subscript: None };
    static ref R11: Lvalue = Lvalue::Variable{ name: "r11".to_string(), width: 8, subscript: None };
    static ref R12: Lvalue = Lvalue::Variable{ name: "r12".to_string(), width: 8, subscript: None };
    static ref R13: Lvalue = Lvalue::Variable{ name: "r13".to_string(), width: 8, subscript: None };
    static ref R14: Lvalue = Lvalue::Variable{ name: "r14".to_string(), width: 8, subscript: None };
    static ref R15: Lvalue = Lvalue::Variable{ name: "r15".to_string(), width: 8, subscript: None };
    static ref R16: Lvalue = Lvalue::Variable{ name: "r16".to_string(), width: 8, subscript: None };
    static ref R17: Lvalue = Lvalue::Variable{ name: "r17".to_string(), width: 8, subscript: None };
    static ref R18: Lvalue = Lvalue::Variable{ name: "r18".to_string(), width: 8, subscript: None };
    static ref R19: Lvalue = Lvalue::Variable{ name: "r19".to_string(), width: 8, subscript: None };
    static ref R20: Lvalue = Lvalue::Variable{ name: "r20".to_string(), width: 8, subscript: None };
    static ref R21: Lvalue = Lvalue::Variable{ name: "r21".to_string(), width: 8, subscript: None };
    static ref R22: Lvalue = Lvalue::Variable{ name: "r22".to_string(), width: 8, subscript: None };
    static ref R23: Lvalue = Lvalue::Variable{ name: "r23".to_string(), width: 8, subscript: None };
    static ref R24: Lvalue = Lvalue::Variable{ name: "r24".to_string(), width: 8, subscript: None };
    static ref R25: Lvalue = Lvalue::Variable{ name: "r25".to_string(), width: 8, subscript: None };
    static ref R26: Lvalue = Lvalue::Variable{ name: "r26".to_string(), width: 8, subscript: None };
    static ref R27: Lvalue = Lvalue::Variable{ name: "r27".to_string(), width: 8, subscript: None };
    static ref R28: Lvalue = Lvalue::Variable{ name: "r28".to_string(), width: 8, subscript: None };
    static ref R29: Lvalue = Lvalue::Variable{ name: "r29".to_string(), width: 8, subscript: None };
    static ref R30: Lvalue = Lvalue::Variable{ name: "r30".to_string(), width: 8, subscript: None };
    static ref R31: Lvalue = Lvalue::Variable{ name: "r31".to_string(), width: 8, subscript: None };
}

pub fn disassemble(st: AvrState, data: LayerIter) -> Program {
    let main = new_disassembler!(u16 =>
        // MOV
        [ "001011 r@. d@..... r@...." ] = |st: &mut State<u16>| {
            let rd = reg(st,"d");
            let rr = reg(st,"r");
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"mov","{8}, {8}",vec!(rd.to_rv(),rr.to_rv()),|cg: &mut CodeGen| {
                cg.assign(rd,rr.to_rv());
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // MOVW
        [ "00000001 d@.... r@...." ] = |st: &mut State<u16>| {
            let rd1 = reg(st,"d"); let rd2 = resolv(st.get_group("d") * 2 + 1);
            let rr1 = reg(st,"r"); let rr2 = resolv(st.get_group("r") * 2 + 1);
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"mov","{8}, {8}",vec!(rd1.to_rv(),rr1.to_rv()),|cg: &mut CodeGen| {
                cg.assign(rd1,rr1.to_rv());
                cg.assign(rd2,rr2.to_rv());
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // IN
        [ "10110 A@.. d@..... A@...." ] = |st: &mut State<u16>| {
            let rd = reg(st,"d");
            let io = ioreg(st,"A");
            let name = if let Lvalue::Variable{ name: n,..} = io { n } else { "(noname)".to_string() };
            let off = Rvalue::Constant(st.get_group("d") as u64);
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"in",&format!("{{8}}, {{8::{}}}",name),vec!(rd.to_rv(),off.clone()),|cg: &mut CodeGen| {
                cg.assign(rd,sram(&off).to_rv());
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // OUT
        [ "10111 A@.. r@..... A@...." ] = |st: &mut State<u16>| {
            let rr = reg(st,"r");
            let io = ioreg(st,"A");
            let name = if let Lvalue::Variable{ name: n,..} = io { n } else { "(noname)".to_string() };
            let off = Rvalue::Constant(st.get_group("d") as u64);
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"in",&format!("{{8::{}}}, {{8}}",name),vec!(off.clone(),rr.to_rv()),|cg: &mut CodeGen| {
                cg.assign(sram(&off),rr.to_rv());
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // POP
        [ "1001000 d@..... 1111" ] = |st: &mut State<u16>| {
            let rd = reg(st,"d");
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"pop","{{8}}",vec!(rd.to_rv()),|cg: &mut CodeGen| {
                let sp = get_sp(cg);
                cg.sub_i(sp.clone(),sp.to_rv(),Rvalue::Constant(1));
                cg.assign(rd,sram(&sp.to_rv()).to_rv());
                set_sp(&sp.to_rv(),cg);
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // PUSH
        [ "1001001 d@..... 1111" ] = |st: &mut State<u16>| {
            let rd = reg(st,"d");
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"push","{{8}}",vec!(rd.to_rv()),|cg: &mut CodeGen| {
                let sp = get_sp(cg);
                cg.sub_i(sp.clone(),sp.to_rv(),Rvalue::Constant(1));
                cg.assign(sram(&sp.to_rv()),rd.to_rv());
                set_sp(&sp.to_rv(),cg);
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // SWAP
        [ "1001010 d@..... 0010" ] = |st: &mut State<u16>| {
            let rd = reg(st,"d");
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"swap","{{8}}",vec!(rd.to_rv()),|cg: &mut CodeGen| {
                let lower = new_temp(8);
                let higher = new_temp(8);

                cg.div_i(higher.clone(),rd.to_rv(),Rvalue::Constant(128));
                cg.mod_i(lower.clone(),rd.to_rv(),Rvalue::Constant(127));

                let shifted = new_temp(8);
                cg.mul_i(shifted.clone(),lower.to_rv(),Rvalue::Constant(128));

                cg.add_i(rd.clone(),shifted.to_rv(),higher.to_rv());
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // XCH
        [ "1001001 r@..... 0100" ] = |st: &mut State<u16>| {
            let rr = reg(st,"r");
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"xch","{{8}}",vec!(rr.to_rv()),|cg: &mut CodeGen| {
                let z = new_temp(16);
                cg.mul_i(z.clone(),R30.to_rv(),Rvalue::Constant(0x100));
                cg.add_i(z.clone(),R31.to_rv(),z.to_rv());

                let tmp = new_temp(8);
                cg.assign(tmp.clone(),sram(&z.to_rv()).to_rv());
                cg.assign(sram(&z.to_rv()),rr.to_rv());
                cg.assign(rr,tmp.to_rv());
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // SER
        [ "11101111 d@.... 1111" ] = |st: &mut State<u16>| {
            let rd = reg(st,"d");
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"ser","{{8}}",vec!(rd.to_rv()),|cg: &mut CodeGen| {
                cg.assign(rd,Rvalue::Constant(0xff));
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // LDI
	    [ "1110 K@.... d@.... K@...." ] = |st: &mut State<u16>| {
            let rd = reg(st,"d");
            let K = st.get_group("K");
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"ldi",&format!("{{8}}, {{::{}}}",K),vec!(rd.to_rv()),|cg: &mut CodeGen| {
                cg.assign(rd,Rvalue::Constant(K as u64));
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },
        // LAC
	    [ "1001001 r@..... 0110" ] = |st: &mut State<u16>| {
            let rr = reg(st,"r");
            let next = st.address + (st.tokens.len() as u64) * 2;

            st.mnemonic(2,"xch","{{8}}",vec!(rr.to_rv()),|cg: &mut CodeGen| {
                let z = new_temp(16);
                cg.mul_i(z.clone(),R30.to_rv(),Rvalue::Constant(0x100));
                cg.add_i(z.clone(),R31.to_rv(),z.to_rv());

                let comp = new_temp(8);
                cg.sub_i(comp,Rvalue::Constant(0xff),sram(&z.to_rv()).to_rv());

                cg.and_i(sram(&z.to_rv()),rr.to_rv(),comp.to_rv());
            });
            st.jump(Rvalue::Constant(next),Guard::new());
            true
        },

    );

    /*
	main[e("1001001 r@..... 0101")] = unary_reg("las",[](cg &c, const variable &r)
	{
		rvalue z = r30 * 0x100 + r31;
		rvalue tmp = sram(z);
		c.assign(sram(z),r | tmp);
		c.assign(r,tmp);
	});
	main[e("1001001 r@..... 0111")] = unary_reg("lat",[](cg &c, const variable &r)
	{
		rvalue z = r30 * 0x100 + r31;
		rvalue tmp = sram(z);
		c.assign(sram(z),r ^ tmp);
		c.assign(r,tmp);
	});
	main[e("1001000 d@..... 0000") >> e("k@................")] = sem_action([](sm &st)
	{
		constant k = constant(st.capture_groups["k"]);
		variable Rd = decode_reg(st.capture_groups["d"]);

		st.mnemonic(st.tokens.size() * 2,"lds","{8}, {8}",Rd,k,[&](cg &c)
		{
			c.assign(Rd,sram(k));
		});
		st.jump(st.address + st.tokens.size() * 2);
		return true;
	});

	main[e("10100 k@... d@.... k@....")] = sem_action([](sm &st)
	{
		unsigned int k_ = st.capture_groups["k"];
		variable Rd = decode_reg(st.capture_groups["d"] + 16);
		constant k = constant((~k_ & 16) | (k_ & 16) | (k_ & 64) | (k_ & 32) | (k_ & 15));

		st.mnemonic(st.tokens.size() * 2,"lds","{8}, {16}",Rd,k,[&](cg &c)
		{
			c.assign(Rd,sram(k));
		});
		st.jump(st.address + st.tokens.size() * 2);
		return true;
	});

	main[f(0x95c8)] = sem_action([](sm &st)
	{
		std::list<rvalue> nop;
		st.mnemonic(st.tokens.size() * 2,"lpm","",nop,[&](cg &c)
		{
			rvalue z = r30 * 0x100 + r31;
			c.assign(r1,flash(z));
		});
		st.jump(st.address + st.tokens.size() * 2);
		return true;
	});

	main[f(0x95e8)] = sem_action([](sm &st)
	{
		// TODO
		st.mnemonic(st.tokens.size() * 2,"spm");
		st.jump(st.address + st.tokens.size() * 2);
		return true;
	});

	main[f(0x95f8)] = sem_action([](sm &st)
	{
		// TODO
		st.mnemonic(st.tokens.size() * 2,"spm","",decode_preg(30,PostInc),std::function<void(cg &c)>());
		st.jump(st.address + st.tokens.size() * 2);
		return true;
	});

	main[e("1001001 d@..... 0000") >> e("k@................")] = sem_action([](sm &st)
	{
		constant k(st.capture_groups["k"]);
		variable Rr = decode_reg(st.capture_groups["r"]);

		st.mnemonic(st.tokens.size() * 2,"sts","{8}, {8}",{k,Rr},[&](cg &c)
		{
			c.assign(sram(k),Rr);
		});
		st.jump(st.address + st.tokens.size() * 2);
		return true;
	});

	main[e("10101 k@... d@.... k@....")] = sem_action([](sm &st)
	{
		unsigned int _k = st.capture_groups["k"];
		constant k = constant((~_k & 16) | (_k & 16) | (_k & 64) | (_k & 32) | (_k & 15));
		variable Rr = decode_reg(st.capture_groups["r"]);

		st.mnemonic(st.tokens.size() * 2,"sts","{16}, {8}",{k,Rr},[&](cg &c)
		{
			c.assign(sram(k),Rr);
		});
		st.jump(st.address + st.tokens.size() * 2);
		return true;
	});

	main[e("10011010 A@..... b@...")] = sem_action([](sm &st)
	{
		constant k = constant(st.capture_groups["A"]);
		constant b = constant(1 << (st.capture_groups["b"] - 1));

		st.mnemonic(st.tokens.size() * 2,"sbi","{8}, {8}",k,b,[&](cg &c)
		{
			c.assign(sram(k),sram(k) | b);
		});
		st.jump(st.address + st.tokens.size() * 2);
		return true;
	});

	main[e("10011000 A@..... b@...")] = sem_action([](sm &st)
	{
		constant k = constant(st.capture_groups["A"]);
		constant b = constant((~(1 << (st.capture_groups["b"] - 1))) & 0xff);

		st.mnemonic(st.tokens.size() * 2,"cbi","{8}, {8}",k,b,[&](cg &c)
		{
			c.assign(sram(k),sram(k) & b);
		});
		st.jump(st.address + st.tokens.size() * 2);
		return true;
	});

	// SREG operations
	//main[e("100101001 s@... 1000")] = simple("bclr");
	//main[e("100101000 s@... 1000")] = simple("bset");
	main[f(0x9408)] = simple("sec",[](cg &m) { m.assign(C,constant(1)); });
	main[f(0x9458)] = simple("seh",[](cg &m) { m.assign(H,constant(1)); });
	main[f(0x9478)] = simple("sei",[](cg &m) { m.assign(I,constant(1)); });
	main[f(0x9428)] = simple("sen",[](cg &m) { m.assign(N,constant(1)); });
	main[f(0x9448)] = simple("ses",[](cg &m) { m.assign(S,constant(1)); });
	main[f(0x9468)] = simple("set",[](cg &m) { m.assign(T,constant(1)); });
	main[f(0x9438)] = simple("sev",[](cg &m) { m.assign(V,constant(1)); });
	main[f(0x9418)] = simple("sez",[](cg &m) { m.assign(Z,constant(1)); });
	main[f(0x9488)] = simple("clc",[](cg &m) { m.assign(C,constant(0)); });
	main[f(0x94d8)] = simple("clh",[](cg &m) { m.assign(H,constant(0)); });
	main[f(0x94f8)] = simple("cli",[](cg &m) { m.assign(I,constant(0)); });
	main[f(0x94a8)] = simple("cln",[](cg &m) { m.assign(N,constant(0)); });
	main[f(0x94c8)] = simple("cls",[](cg &m) { m.assign(S,constant(0)); });
	main[f(0x94e8)] = simple("clt",[](cg &m) { m.assign(T,constant(0)); });
	main[f(0x94b8)] = simple("clv",[](cg &m) { m.assign(V,constant(0)); });
	main[f(0x9498)] = simple("clz",[](cg &m) { m.assign(Z,constant(0)); });
	main[e("000101 r@. d@..... r@....")] = binary_reg("cp",[](cg &m, const variable &Rd, const variable &Rr)
	{
		rvalue R = (Rd - Rr) % 0x100;

		m.less_i(H,Rd % 0x10, Rr % 0x10);
		m.less_i(C,Rd, Rr);
		m.equal_i(Z,R,constant(0));
		m.less_i(N,constant(0x7f),R);
		m.not_b(V,C);
		m.or_b(S,m.and_b(m.not_b(N),V),m.and_b(N,m.not_b(V)));
	});
	main[e("000001 r@. d@..... r@....")] = binary_reg("cpc",[](cg &m, const variable &Rd, const variable &Rr)
	{
		rvalue Cr = m.lift_b(C);
		rvalue R = (Rd - Rr - Cr) % 0x100;

		m.less_i(H,Rd % 0x10, Rr % 0x10);
		m.less_i(C,Rd,Rr);
		m.and_b(Z,Z,m.equal_i(R,constant(0)));
		m.less_i(N,constant(0x7f), R);
		m.not_b(V,C);
		m.or_b(S,m.and_b(m.not_b(N),V),m.and_b(N,m.not_b(V)));
	});
	main[e("0011 K@.... d@.... K@....")] = binary_regconst("cpi",[&](cg &m, const variable &Rd, const constant &K)
	{
		rvalue R = (Rd - K) % 0x100;

		m.less_i(H,Rd % 0x10,K % 0x10);
		m.less_i(C,Rd,K);
		m.equal_i(Z,R,constant(0));
		m.less_i(N,constant(0x7f),R);
		m.not_b(V,C);
		m.or_b(S,m.and_b(m.not_b(N),V),m.and_b(N,m.not_b(V)));
	});

	// main[e("001000 d@..........")] = tst (alias for and)

	// bit-level logic
	// main[e("0110 K@.... d@.... K@....")] = sbr (alias for ori)
	// main[e("000011 d@..........")] = lsl (alias for add X,X);
	main[e("1001010 d@..... 0110")] = unary_reg("lsr");

	// byte-level arithmetic and logic
	main[e("000111 r@. d@..... r@....")] = binary_reg("adc",[](cg &m, const variable &Rd, const variable &Rr)
	{
		rvalue Cr = m.lift_b(C);
		rvalue R = Rd + Rr + Cr;

		m.less_i(H,constant(16),(Rd % 0x10) + (Rr % 0x10));
		m.or_b(V,
			m.and_b(m.less_i(Rr,constant(0x80),m.and_b(m.less_i(Rd,constant(0x80)),m.less_i(constant(0x7f),R))),
			m.and_b(m.less_i(constant(0x7f),Rr),m.and_b(m.less_i(constant(0x7f),Rd),m.less_i(R,constant(0x80))))));
		m.less_i(N,R,constant(0x7f));
		m.equal_i(Z,constant(0),R);
		m.less_i(C,constant(0x100),R);
		m.or_b(S,m.and_b(m.not_b(N),V),m.and_b(N,m.not_b(V)));
		m.assign(Rd,R % 0x100);
	});
	main[e("000011 r@. d@..... r@....")] = binary_reg("add",[](cg &m, const variable &Rd, const variable &Rr)
	{
		rvalue R = Rd + Rr;

		m.less_i(H,constant(16),(Rd % 0x10) + (Rr % 0x10));
		m.or_b(V,
			m.and_b(m.less_i(Rr,constant(0x80),m.and_b(m.less_i(Rd,constant(0x80)),m.less_i(constant(0x7f),R))),
			m.and_b(m.less_i(constant(0x7f),Rr),m.and_b(m.less_i(constant(0x7f),Rd),m.less_i(R,constant(0x80))))));
		m.less_i(N,R,constant(0x7f));
		m.equal_i(Z,constant(0),R);
		m.less_i(C,constant(0x100),R);
		m.or_b(S,m.and_b(m.not_b(N),V),m.and_b(N,m.not_b(V)));
		m.assign(Rd,R % 0x100);
	});
	main[e("001000 r@. d@..... r@....")] = binary_reg("and",[](cg &m, const variable &Rd, const variable &Rr)
	{
		m.and_i(Rd,Rd & Rr);

		m.assign(V,constant(0));										// V: 0
		m.less_i(N,Rd,constant(0x7f));
		m.or_b(S,m.and_b(m.not_b(N),V),m.and_b(N,m.not_b(V)));
		m.equal_i(Z,constant(0),Rd);
	});
	main[e("0111 K@.... d@.... K@....")] = binary_regconst("andi",[&](cg &m, const variable &Rd, const constant &K)
	{
		m.and_i(Rd,Rd & K);

		m.assign(V,constant(0));										// V: 0
		m.less_i(N,Rd,constant(0x7f));
		m.or_b(S,m.and_b(m.not_b(N),V),m.and_b(N,m.not_b(V)));
		m.equal_i(Z,constant(0),Rd);
	});

	main[e("001001 r@. d@..... r@....")] = sem_action([](sm &st)
	{
		variable Rd = decode_reg(st.capture_groups["d"]);
		variable Rr = decode_reg(st.capture_groups["r"]);

		if(Rd == Rr)
		{
			st.mnemonic(st.tokens.size() * 2,"clr","",Rd,[&](cg &m)
			{
				m.assign(Rd,constant(0));
				m.assign(V,constant(0));
				m.assign(N,constant(0));
				m.assign(S,constant(0));
				m.assign(Z,constant(0));
			});
			st.jump(st.address + st.tokens.size() * 2);
		}
		else
		{
			st.mnemonic(st.tokens.size() * 2,"eor","",Rd,Rr,[&](cg &m)
			{
				m.xor_i(Rd,Rd,Rr);
				m.assign(V,constant(0));										// V: 0
				m.less_i(N,Rd,constant(0x7f));
				m.or_b(S,m.and_b(m.not_b(N),V),m.and_b(N,m.not_b(V)));
				m.equal_i(Z,constant(0),Rd);
			});
			st.jump(st.address + st.tokens.size() * 2);
		}
		return true;
	});
	main[e("1001010 d@..... 0001")] = unary_reg("neg",[](cg &m, const variable &Rd)
	{
		//TODO: m.assign(Rd,Rd ^ 0xff);
	});

	main[e("001010 r@. d@..... r@....")] = binary_reg("or",[](cg &m, const variable &Rd, const variable &Rr)
	{
		// TODO
	});
	main[e("0110 K@.... d@.... K@....")] = binary_regconst("ori",[&](cg &m, const variable &Rd, const constant &K)
	{
		//m.or_b(Rd,Rd,K);
	});

	main[e("000110 r@. d@..... r@....")] = binary_reg("sub",[&](cg &m, const variable &Rd, const variable &Rr)
	{
		rvalue R = (Rd - Rr) % 0x100;

		m.less_i(H,Rd % 0x10, Rr % 0x10);
		m.less_i(C,Rd, Rr);
		m.equal_i(Z,R,constant(0));
		m.less_i(N,constant(0x7f), R);
		m.not_b(V,C);
		m.or_b(S,m.and_b(m.not_b(N),V),m.and_b(N,m.not_b(V)));
		m.assign(Rd,R);
	});
	main[e("0101 K@.... d@.... K@....")] = binary_regconst("subi",[&](cg &m, const variable &Rd, const constant &K)
	{
		rvalue Cr = m.lift_b(C);
		rvalue R = Rd - K - Cr;

		m.less_i(H,Rd % 0x10, K % 0x10);
		m.less_i(C,Rd, K);
		m.and_b(Z,Z,m.equal_i(R,constant(0)));
		m.less_i(N,constant(0x7f), R);
		m.not_b(V,C);
		m.or_b(S,m.and_b(m.not_b(N),V),m.and_b(N,m.not_b(V)));
		m.assign(Rd,R);
	});

	main[e("1001010 d@..... 0101")] = unary_reg("asr");
	main[e("000111 d@..........")] = unary_reg("rol");
	main[e("1001010 d@..... 0111")] = unary_reg("ror");
	main[e("1001010 d@..... 1010")] = unary_reg("dec");
	main[e("1001010 d@..... 0011")] = unary_reg("inc");
	main[e("000010 r@. d@..... r@....")] = binary_reg("sbc",[](cg &m, const variable &Rd, const variable &Rr)
	{
		rvalue Cr = m.lift_b(C);
		rvalue R = Rd - Rr - Cr;

		m.less_i(H,Rd % 0x10, Rr % 0x10);
		m.less_i(C,Rd,Rr);
		m.and_b(Z,Z,m.equal_i(R,constant(0)));
		m.less_i(N,constant(0x7f),R);
		m.not_b(V,C);
		m.or_b(S,m.and_b(m.not_b(N),V),m.and_b(N,m.not_b(V)));

		m.assign(Rd,R % 0x100);
	});

	main[e("0100 K@.... d@.... K@....")] = binary_regconst("sbci",[&](cg &m, const variable &Rd, const constant &K)
	{
		rvalue Cr = m.lift_b(C);
		rvalue R = Rd - K - Cr;

		m.less_i(H,Rd % 0x10, K % 0x10);
		m.less_i(C,Rd,K);
		m.and_b(Z,Z,m.equal_i(R,constant(0)));
		m.less_i(N,constant(0x7f), R);
		m.not_b(V,C);
		m.or_b(S,m.and_b(m.not_b(N),V),m.and_b(N,m.not_b(V)));

		m.assign(Rd,R % 0x100);
	});

	main[e("1001010 d@..... 0000")] = unary_reg("com");

	// word-level arithmetic and logic
	main[e("10010110 K@.. d@.. K@....")] = sem_action([](sm &st)
	{
		constant K = constant((unsigned int)st.capture_groups["K"]);
		unsigned int d = (unsigned int)st.capture_groups["d"] * 2 + 24;
		variable Rd1 = decode_reg(d);
		variable Rd2 = decode_reg(d+1);

		st.mnemonic(st.tokens.size() * 2,"adiw","{8}:{8}, {16}",{Rd2,Rd1,K},[&](cg &c)
		{
			rvalue R = Rd2 * 0x100 + Rd1 + K;

			// V: !Rdh7•R15
			c.and_b(V,c.less_i(Rd2,constant(0x80)),c.not_b(c.less_i(R,constant(0x8000))));

			// N: R15
			c.less_i(N,R,constant(0x8000));

			// Z: !R15•!R14•!R13•!R12•!R11•!R10•!R9•!R8•!R7•R6•!R5•!R4•!R3•!R2•!R1•!R0
			c.equal_i(Z,constant(0),R);

			// C: !R15•Rdh7
			c.and_b(V,c.not_b(c.less_i(Rd2,constant(0x80))),c.less_i(R,constant(0x8000)));

			// S: N ⊕ V
			c.or_b(S,c.and_b(c.not_b(N),V),c.and_b(N,c.not_b(V)));

			c.assign(Rd2,R / 0x100);
			c.assign(Rd1,R % 0x100);
		});
		st.jump(st.address + st.tokens.size() * 2);
		return true;
	});
	main[e("10010111 K@.. d@.. K@....")] = sem_action([](sm &st)
	{
		unsigned int d = (unsigned int)st.capture_groups["d"] * 2 + 24;
		constant K = constant((unsigned int)st.capture_groups["K"]);
		variable Rd1 = decode_reg(d);
		variable Rd2 = decode_reg(d+1);

		st.mnemonic(st.tokens.size() * 2,"sbiw","{8}:{8}, {16}",{Rd1,Rd2,K});
		st.jump(st.address + st.tokens.size() * 2);
		return true;
	});
	main[e("0000 0011 0 d@... 1 r@...")] = binary_reg("fmul",[](cg &m, const variable &Rd, const variable &Rr)
	{
		// TODO
	});
	main[e("000000111 d@... 0 r@...")] = binary_reg("fmuls",[](cg &m, const variable &Rd, const variable &Rr)
	{
		// TODO
	});
	main[e("000000111 d@... 1 r@...")] = binary_reg("fmulsu",[](cg &m, const variable &Rd, const variable &Rr)
	{
		// TODO
	});
	main[e("100111 r@. d@..... r@....")] = binary_reg("mul",[](cg &m, const variable &Rd, const variable &Rr)
	{
		// TODO
	});
	main[e("00000010 d@.... r@....")] = binary_reg("muls",[](cg &m, const variable &Rd, const variable &Rr)
	{
		// TODO
	});
	main[e("000000110 d@... 0 r@...")] = binary_reg("muls",[](cg &m, const variable &Rd, const variable &Rr)
	{
		// TODO
	});

	// branches
	// main[e("111101 k@....... s@...")] = simple("brbc");
	// main[e("111100 k@....... s@...")] = [](sm &st)  { st.mnemonic(st.tokens.size() * 2,"brbs"; });
	main[e("111101 k@....... 000")] = branch("brcc",C,false);
	main[e("111100 k@....... 000")] = branch("brcs",C,true);
	main[e("111100 k@....... 001")] = branch("breq",Z,true);
	main[e("111101 k@....... 100")] = branch("brge",S,false);
	main[e("111101 k@....... 101")] = branch("brhc",H,false);
	main[e("111100 k@....... 101")] = branch("brhs",H,true);
	main[e("111101 k@....... 111")] = branch("brid",I,false);
	main[e("111100 k@....... 111")] = branch("brie",I,true);
	main[e("111100 k@....... 000")] = branch("brlo",C,true);
	main[e("111100 k@....... 100")] = branch("brlt",S,true);
	main[e("111100 k@....... 010")] = branch("brmi",N,true);
	main[e("111101 k@....... 001")] = branch("brne",Z,false);
	main[e("111101 k@....... 010")] = branch("brpl",N,false);
	main[e("111101 k@....... 000")] = branch("brsh",C,false);
	main[e("111101 k@....... 110")] = branch("brtc",T,false);
	main[e("111100 k@....... 110")] = branch("brts",T,true);
	main[e("111101 k@....... 011")] = branch("brvc",V,false);
	main[e("111100 k@....... 011")] = branch("brvs",V,true);
	main[e("1111 110r@..... 0 b@...")] = sem_action([](sm &st)
	{
		variable Rr = decode_reg(st.capture_groups["r"]);
		constant b = constant(st.capture_groups["b"]);

		st.mnemonic(st.tokens.size() * 2,"sbrc","",Rr,b,std::function<void(cg &c)>());
		st.jump(st.address + st.tokens.size() * 2);
		//st.skip_next = true;
		return true;
	});
	main[e("1111 111 r@..... 0 b@...")] = sem_action([](sm &st)
	{
		variable Rr = decode_reg(st.capture_groups["r"]);
		constant b = constant(st.capture_groups["b"]);

		st.mnemonic(st.tokens.size() * 2,"sbrs","",Rr,b,std::function<void(cg &c)>());
		st.jump(st.address + st.tokens.size() * 2);
		//st.skip_next = true;
		return true;
	});
	main[e("000100 r@. d@..... r@....")] = sem_action([](sm &st)
	{
		variable Rr = decode_reg(st.capture_groups["r"]);
		variable Rd = decode_reg(st.capture_groups["d"]);

		st.mnemonic(st.tokens.size() * 2,"cpse","",Rd,Rr,std::function<void(cg &c)>());
		st.jump(st.address + st.tokens.size() * 2);
		//st.skip_next = true;
		return true;
	});
	main[e("1001 1001 A@..... b@...")] = sem_action([](sm &st)
	{
		variable A = decode_ioreg(st.capture_groups["A"]);
		constant b = constant(st.capture_groups["b"]);

		st.mnemonic(st.tokens.size() * 2,"sbic","",A,b,std::function<void(cg &c)>());
		st.jump(st.address + st.tokens.size() * 2);
		//st.skip_next = true;
		return true;
	});
	main[e("1001 1011 A@..... b@...")] = sem_action([](sm &st)
	{
		variable A = decode_ioreg(st.capture_groups["A"]);
		constant b = constant(st.capture_groups["b"]);

		st.mnemonic(st.tokens.size() * 2,"sbis","",A,b,std::function<void(cg &c)>());
		st.jump(st.address + st.tokens.size() * 2);
		//st.skip_next = true;
		return true;
	});

	// jump branches
	main[e("1001010 k@..... 111 k@.") >> e("k@................")] = sem_action([](sm &st)
	{
		constant k = constant((st.capture_groups["k"] * 2) % (st.state.flash_sz));

		st.mnemonic(st.tokens.size() * 2,"call","",k,[&](cg &c)
		{
			c.call_i(k);
		});
		st.jump(st.address + st.tokens.size() * 2);
		return true;
	});
	main[e("1001010 k@..... 110 k@.") >> e("k@................")] = sem_action([](sm &st)
	{
		constant k = constant((st.capture_groups["k"] * 2) % (st.state.flash_sz));

		st.mnemonic(st.tokens.size() * 2,"jmp","",k,std::function<void(cg &c)>());
		st.jump(k);
		return true;
	});

	main[e("1101 k@............")] = sem_action([](sm &st)
	{
		int _k = st.capture_groups["k"];
		constant k = constant(((_k <= 2047 ? _k : _k - 4096) * 2 + 2 + st.address) % st.state.flash_sz);

		st.mnemonic(st.tokens.size() * 2,"rcall","",k,[&](cg &c)
		{
			c.call_i(k);
		});
		st.jump(st.address + 2);
		return true;
	});
	main[e("1100 k@............")] = sem_action([](sm &st)
	{
		int _k = st.capture_groups["k"];
		constant k = constant(((_k <= 2047 ? _k : _k - 4096) * 2 + 2 + st.address) % st.state.flash_sz);

		st.mnemonic(st.tokens.size() * 2,"rjmp","",k,std::function<void(cg &c)>());
		std::cerr << k << " " << _k << " " << (_k <= 2047 ? _k : _k - 4096) << " " << (_k <= 2047 ? _k : _k - 4096) * 2 + 2 + st.address << std::endl;
		st.jump(k);
		return true;
	});
	main[f(0x9508)] = sem_action([](sm &st) { st.mnemonic(st.tokens.size() * 2, "ret"); return true;  });
	main[f(0x9518)] = sem_action([](sm &st) { st.mnemonic(st.tokens.size() * 2, "reti"); return true; });
	main[f(0x9409)] = sem_action([](sm &st)
	{
		variable J(variable("J",16));
		std::list<rvalue> nop;

		st.mnemonic(st.tokens.size() * 2,"ijmp","",nop,[&](cg &c)
		{
			c.assign(J,((r31 * 0x100 + r30) * 2) % constant(st.state.flash_sz));
		});
		st.jump(J);
		return true;
	});

	// TODO: icall
	main[f(0x9509)] = sem_action([](sm &st) { st.mnemonic(st.tokens.size() * 2, "icall"); return true; });

	// store and load with x,y,z
	main[e("1001 001r@. r@.... 1100")] = binary_st(r26,r27,false,false);
	main[e("1001 001r@. r@.... 1101")] = binary_st(r26,r27,false,true);
	main[e("1001 001r@. r@.... 1110")] = binary_st(r26,r27,true,false);

	main[e("1000 001r@. r@.... 1000")] = binary_st(r28,r29,false,false);
	main[e("1001 001r@. r@.... 1001")] = binary_st(r28,r29,false,true);
	main[e("1001 001r@. r@.... 1010")] = binary_st(r28,r29,true,false);
	main[e("10q@.0 q@..1r@. r@.... 1q@...")] = binary_stq(r28,r29);

	main[e("1000 001r@. r@.... 0000")] = binary_st(r30,r31,false,false);
	main[e("1001 001r@. r@.... 0001")] = binary_st(r30,r31,false,true);
	main[e("1001 001r@. r@.... 0010")] = binary_st(r30,r31,true,false);
	main[e("10q@.0 q@..1r@. r@.... 0q@...")] = binary_stq(r30,r31);

	main[e("1001 000d@. d@.... 1100")] = binary_ld(r26,r27,false,false);
	main[e("1001 000d@. d@.... 1101")] = binary_ld(r26,r27,false,true);
	main[e("1001 000d@. d@.... 1110")] = binary_ld(r26,r27,true,false);

	main[e("1000 000d@. d@.... 1000")] = binary_ld(r28,r29,false,false);
	main[e("1001 000d@. d@.... 1001")] = binary_ld(r28,r29,false,true);
	main[e("1001 000d@. d@.... 1010")] = binary_ld(r28,r29,true,false);
	main[e("10 q@. 0 q@.. 0 d@..... 1 q@...")] = binary_ldq(r28,r29);

	main[e("1000 000d@. d@.... 0000")] = binary_ld(r30,r31,false,false);
	main[e("1001 000 d@..... 0001")] = binary_ld(r30,r31,false,true);
	main[e("1001 000d@. d@.... 0010")] = binary_ld(r30,r31,true,false);
	main[e("10q@.0 q@..0d@. d@.... 0q@...")] = binary_ldq(r30,r31);

	// misc
	main[f(0x9598)] = simple("break",[](cg &m) { /* TODO */ });
	main[e("10010100 K@.... 1011")] = sem_action([](sm &st)
	{
		st.mnemonic(st.tokens.size() * 2,"des","",constant(st.capture_groups["K"]),std::function<void(cg &c)>());
		st.jump(st.tokens.size() * 2 + st.address);
		return true;
	});

	main[f(0x0)] = simple("nop",[](cg &m) { /* TODO */ });
	main[f(0x9588)] = simple("sleep",[](cg &m) { /* TODO */ });
	main[f(0x95a8)] = simple("wdr",[](cg &m) { /* TODO */ });

	// catch all
	main = sem_action([](sm &st)
	{
		st.mnemonic(1, "unk");
		return true;
	});*/

    Program::disassemble(None,main,State::<u16>::new(0),data,0)
}

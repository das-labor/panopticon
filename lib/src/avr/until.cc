#include <functional>
#include <list>
#include <string>

#define AVR_PRIVATE
#include <avr/avr.hh>
#include <avr/until.hh>

using namespace po;
using namespace po::avr;

// registers
const variable r0 = "r0"_v8, r1 = "r1"_v8, r2 = "r2"_v8, r3 = "r3"_v8, r4 = "r4"_v8, r5 = "r5"_v8, r6 = "r6"_v8, 
							 r7 = "r7"_v8, r8 = "r8"_v8, r9 = "r9"_v8, r10 = "r10"_v8, r11 = "r11"_v8, r12 = "r12"_v8,
							 r13 = "r13"_v8, r14 = "r14"_v8, r15 = "r15"_v8, r16 = "r16"_v8, r17 = "r17"_v8, r18 = "r18"_v8,
							 r19 = "r19"_v8, r20 = "r20"_v8, r21 = "r21"_v8, r22 = "r22"_v8, r23 = "r23"_v8, r24 = "r24"_v8,
							 r25 = "r25"_v8, r26 = "r26"_v8, r27 = "r27"_v8, r28 = "r28"_v8, r29 = "r29"_v8, r30 = "r30"_v8,
							 r31 = "r31"_v1, I = "I"_v1, T = "T"_v1, H = "H"_v1, S = "S"_v1, V = "V"_v1, N = "N"_v1, Z = "Z"_v1, C = "C"_v1;

variable po::avr::decode_reg(unsigned int r)
{
	assert(r >= 0 && r <= 31);
	return variable("r" + std::to_string(r),8);
}

variable po::avr::decode_preg(unsigned int r, IndirectRegOp op, int d)
{
	std::string name;
	
	switch(r)
	{
		case 26: name = "X"; break;
		case 28: name = "Y"; break;
		case 30: name = "Z"; break;
		default: assert(false);
	}

	switch(op)
	{
		case PostInc: name += "+"; break;
		case PreDec: name = "-" + name; break;
		case Nop: break;
		case PostDisplace: if(r != 26) { name += "+" + std::to_string(d); break; }
		default: assert(false);
	}

	return variable(name,8);
}

variable po::avr::decode_ioreg(unsigned int r)
{
	std::string name;

	switch(r)
	{
		default: name = "io" + std::to_string(r); break;
		case 0x00: name = "ubrr1"; break;
		case 0x01: name = "ucsr1b"; break;
		case 0x02: name = "ucsr1a"; break;
		case 0x03: name = "udr1"; break;
		case 0x05: name = "pine"; break;
		case 0x06: name = "ddre"; break;
		case 0x07: name = "porte"; break;
		case 0x08: name = "acsr"; break;
		case 0x09: name = "ubrr0"; break;
		case 0x0A: name = "ucsr0b"; break;
		case 0x0B: name = "ucsr0a"; break;
		case 0x0C: name = "udr0"; break;
		case 0x0D: name = "spcr"; break;
		case 0x0E: name = "spsr"; break;
		case 0x0F: name = "spdr"; break;
		case 0x10: name = "pind"; break;
		case 0x11: name = "ddrd"; break;
		case 0x12: name = "portd"; break;
		case 0x13: name = "pinc"; break;
		case 0x14: name = "ddrc"; break;
		case 0x15: name = "portc"; break;
		case 0x16: name = "pinb"; break;
		case 0x17: name = "ddrb"; break;
		case 0x18: name = "portb"; break;
		case 0x19: name = "pina"; break;
		case 0x1A: name = "ddra"; break;
		case 0x1B: name = "porta"; break;
		case 0x1C: name = "eecr"; break;
		case 0x1D: name = "eedr"; break;
		case 0x1E: name = "eearl"; break;
		case 0x1F: name = "eearh"; break;
		case 0x20: name = "ubrrh"; break;
		case 0x21: name = "wdtcr"; break;
		case 0x22: name = "ocr2"; break;
		case 0x23: name = "tcnt2"; break;
		case 0x24: name = "icr1l"; break;
		case 0x25: name = "icr1h"; break;
		case 0x26: name = "assr"; break;
		case 0x27: name = "tccr2"; break;
		case 0x28: name = "ocr1bl"; break;
		case 0x29: name = "ocr1bh"; break;
		case 0x2A: name = "ocr1al"; break;
		case 0x2B: name = "ocr1ah"; break;
		case 0x2C: name = "tcnt1l"; break;
		case 0x2D: name = "tcnt1h"; break;
		case 0x2E: name = "tccr1b"; break;
		case 0x2F: name = "tccr1a"; break;
		case 0x30: name = "sfior"; break;
		case 0x31: name = "ocr0"; break;
		case 0x32: name = "tcnt0"; break;
		case 0x33: name = "tccr0"; break;
		case 0x34: name = "mcusr"; break;
		case 0x35: name = "mcucr"; break;
		case 0x36: name = "emcucr"; break;
		case 0x37: name = "spmcr"; break;
		case 0x38: name = "tifr"; break;
		case 0x39: name = "timsk"; break;
		case 0x3A: name = "gifr"; break;
		case 0x3B: name = "gimsk"; break;
		case 0x3D: name = "spl"; break;
		case 0x3E: name = "sph"; break;
		case 0x3F: name = "sreg"; break;
	}

	return variable(name,8);
}

memory po::avr::sram(rvalue o) 
{ 
	return memory(o,1,memory::BigEndian,"sram"); 
}

memory po::avr::flash(rvalue o) 
{ 
	return memory(o,1,memory::BigEndian,"flash"); 
}

sem_action po::avr::unary_reg(std::string x, std::function<void(cg &c, const variable &v)> func)
{
	return [x,func](sm &st)
	{
		variable op = st.capture_groups.count("d") ? decode_reg((unsigned int)st.capture_groups["d"]) : 
																								 decode_reg((unsigned int)st.capture_groups["r"]);
		if(func)
			st.mnemonic(st.tokens.size(),x,"{8}",op,std::bind(func,std::placeholders::_1,op));
		else
			st.mnemonic(st.tokens.size(),x,"{8}",op);
		st.jump(st.address + st.tokens.size());
	};
}

sem_action po::avr::binary_reg(std::string x, std::function<void(cg &,const variable&,const variable&)> func)
{
	return [x,func](sm &st)
	{
		variable Rd = decode_reg(st.capture_groups["d"]);
		variable Rr = decode_reg(st.capture_groups["r"]);

		st.mnemonic(st.tokens.size(),x,"{8}, {8}",Rd,Rr,bind(func,std::placeholders::_1,Rd,Rr));
		st.jump(st.address + st.tokens.size());
	};
}

sem_action po::avr::branch(std::string m, rvalue flag, bool set)
{
	return [m,flag,set](sm &st)
	{
		int64_t _k = st.capture_groups["k"];
		guard_ptr g(new guard(flag,relation::Eq,set ? 1_i8 : 0_i8));
		constant k = constant((int8_t)(_k <= 63 ? _k : _k - 128),8);

		st.mnemonic(st.tokens.size(),m,"{8:-}",k);
		st.jump(st.address + 1,g->negation());
		st.jump(st.address + k.content() + 1,g);
	};
}

sem_action po::avr::binary_regconst(std::string x, std::function<void(cg &,const variable&,const constant&)> func)
{
	return [x,func](sm &st)
	{
		variable Rd = decode_reg(st.capture_groups["d"] + 16);
		constant K = constant(st.capture_groups["K"],16);

		st.mnemonic(st.tokens.size(),x,"{8}, {8}",{Rd,K},bind(func,std::placeholders::_1,Rd,K));
		st.jump(st.address + st.tokens.size());
	};
}

sem_action po::avr::binary_st(variable Rd1, variable Rd2, bool pre_dec, bool post_inc)
{
	assert(!(pre_dec == true && post_inc == true));

	return [=](sm &st)
	{
		lvalue X = po::temporary(po::avr_tag());
		
		variable Rr = decode_reg(st.capture_groups["r"]);
		std::string fmt("");

		if(pre_dec)
			fmt += "-";
		
		fmt += "{8::";

		if(Rd1.name() == "r26")
			fmt += "X";
		else if(Rd1.name() == "r28")
			fmt += "Y";
		else if(Rd1.name() == "r30")
			fmt += "Z";
		else
			assert(false);


		if(post_inc)
			fmt += "+";

		fmt += "}, {8}";

		st.mnemonic(st.tokens.size(),"st",fmt,{X,Rr},[=](cg &c)
		{
			c.or_b(X,c.shiftl_u(Rd2,8_i8),Rd1);
			
			if(pre_dec) 
				c.sub_i(X,X,1_i8);
			
			c.assign(sram(X),Rr);
			
			if(post_inc) 
				c.add_i(X,X,1_i8);

			if(post_inc || pre_dec)
			{
				c.and_b(Rd1,X,0xff_i8);
				c.shiftr_u(Rd2,X,8_i8);
			}
		});
		st.jump(st.address + st.tokens.size());
	};
}

sem_action po::avr::binary_ld(variable Rr1, variable Rr2, bool pre_dec, bool post_inc)
{
	assert(!(pre_dec == true && post_inc == true));

	return [=](sm &st)
	{
		lvalue X = po::temporary(po::avr_tag());
		
		variable Rd = decode_reg(st.capture_groups["r"]);
		std::string fmt("");

		if(pre_dec)
			fmt += "-";
		
		fmt += "{8::";

		if(Rr1.name() == "r26")
			fmt += "X";
		else if(Rr1.name() == "r28")
			fmt += "Y";
		else if(Rr1.name() == "r30")
			fmt += "Z";
		else
			assert(false);


		if(post_inc)
			fmt += "+";

		fmt += "}, {8}";

		st.mnemonic(st.tokens.size(),"ld",fmt,{X,Rd},[=](cg &c)
		{
			c.or_b(X,c.shiftl_u(Rr2,8_i8),Rr1);
			
			if(pre_dec) 
				c.sub_i(X,X,1_i8);
			
			c.assign(Rd,sram(X));
			
			if(post_inc) 
				c.add_i(X,X,1_i8);

			if(post_inc || pre_dec)
			{
				c.and_b(Rr1,X,0xff_i8);
				c.shiftr_u(Rr2,X,8_i8);
			}
		});
		st.jump(st.address + st.tokens.size());
	};
}

sem_action po::avr::binary_stq(variable Rd1, variable Rd2)
{
	return [=](sm &st)
	{
		unsigned int q = st.capture_groups["q"];
		lvalue X = po::temporary(po::avr_tag());

		variable Rr = decode_reg(st.capture_groups["r"]);
		std::string fmt("{8::");

		if(Rd1.name() == "r28")
			fmt += "Y";
		else if(Rd1.name() == "r30")
			fmt += "Z";
		else
			assert(false);

		fmt += "+" + std::to_string(q);

		fmt += "}, {8}";

		st.mnemonic(st.tokens.size(),"st",fmt,{X,Rr},[=](cg &c)
		{
			c.or_b(X,c.shiftl_u(Rd2,8_i8),Rd1);
			c.add_i(X,X,constant(q,16));
			c.assign(sram(X),Rr);
		});
		st.jump(st.address + st.tokens.size());
	};
}

sem_action po::avr::binary_ldq(variable Rr1, variable Rr2)
{
		return [=](sm &st)
	{
		unsigned int q = st.capture_groups["q"];
		lvalue X = po::temporary(po::avr_tag());
		
		variable Rd = decode_reg(st.capture_groups["r"]);
		std::string fmt("{8::");

		if(Rr1.name() == "r28")
			fmt += "Y";
		else if(Rr1.name() == "r30")
			fmt += "Z";
		else
			assert(false);

		fmt += "+" + std::to_string(q);

		fmt += "}, {8}";

		st.mnemonic(st.tokens.size(),"ld",fmt,{X,Rd},[=](cg &c)
		{
			c.or_b(X,c.shiftl_u(Rr2,8_i8),Rr1);
			c.add_i(X,X,constant(q,16));
			c.assign(Rd,sram(X));
		});
		st.jump(st.address + st.tokens.size());
	};
}

sem_action po::avr::simple(std::string x, std::function<void(cg&)> fn)
{
	return [x,fn](sm &st)
	{
		std::list<rvalue> nop;
		st.mnemonic(st.tokens.size(),x,"",nop,fn);
		st.jump(st.address + st.tokens.size());
	};
}

// H: !a3•b3 + b3•c3 + c3•!a3
// Half carry for c = a - b or a = b + c
void po::avr::half_carry(const rvalue &a, const rvalue &b, const rvalue &c, cg &m)
{
	rvalue a_not = m.not_b(a);

	m.slice(H,m.or_b(m.or_b(
		m.and_b(a_not,b),
		m.and_b(b,c)),
		m.and_b(a_not,c)),
	3_i8,3_i8);
}

// V: a7•!b7•!c7 + !a7•b7•c7
// Two's complements overflow for c = a - b or a = b + c
void po::avr::two_complement_overflow(const rvalue &a, const rvalue &b, const rvalue &c, cg &m)
{
	m.slice(V,
		m.or_b(
			m.and_b(m.and_b(a,m.not_b(b)),c),
			m.and_b(m.and_b(m.not_b(a),b),c)),
		7_i8,7_i8);
}

// !a7•!a6•!a5•!a4•!a3•!a2•!a1•!a0
rvalue po::avr::zero(const rvalue &a, cg &m)
{
	rvalue not_a = m.not_b(a);
	return m.and_b(m.slice(not_a,0_i8,0_i8),
					m.and_b(m.slice(not_a,1_i8,1_i8),
						m.and_b(m.slice(not_a,2_i8,2_i8),
							m.and_b(m.slice(not_a,3_i8,3_i8),
								m.and_b(m.slice(not_a,4_i8,4_i8),
									m.and_b(m.slice(not_a,5_i8,5_i8),
										m.and_b(m.slice(not_a,6_i8,6_i8),
											m.slice(not_a,7_i8,7_i8))))))));
}

// Zero flag for result a
void po::avr::is_zero(const rvalue &a, cg &m)
{
	m.assign(Z,zero(a,m));
}

// C: !a7•b7 + b7•c7 + c7•!a7
// Carry for c = a - b or a = b + c
void po::avr::carry(const rvalue &a, const rvalue &b, const rvalue &c, cg &m)
{
	rvalue a_not = m.not_b(a);

	m.slice(C,m.or_b(m.or_b(
		m.and_b(a_not,b),
		m.and_b(b,c)),
		m.and_b(a_not,c)),
	7_i8,7_i8);
}

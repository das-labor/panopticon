#include <functional>
#include <list>
#include <string>

#define AVR_PRIVATE
#include <avr/avr.hh>
#include <avr/until.hh>

using namespace po;
using namespace po::avr;

variable po::avr::decode_reg(unsigned int r)
{
	assert(r >= 0 && r <= 31);
	return variable("r" + std::to_string(r));
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

	return variable(name);
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

	return variable(name);
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
		guard_ptr g(new guard(flag,relation::Eq,set ? 1_val : 0_val));
		constant k = (int8_t)(_k <= 63 ? _k : _k - 128);

		st.mnemonic(st.tokens.size(),m,"{8:-}",k);
		st.jump(st.address + 1,g->negation());
		st.jump(st.address + k.value() + 1,g);
	};
}

sem_action po::avr::binary_regconst(std::string x, std::function<void(cg &,const variable&,const constant&)> func)
{
	return [x,func](sm &st)
	{
		variable Rd = decode_reg(st.capture_groups["d"] + 16);
		constant K = st.capture_groups["K"];

		st.mnemonic(st.tokens.size(),x,"{8}, {8}",{Rd,K},bind(func,std::placeholders::_1,Rd,K));
		st.jump(st.address + st.tokens.size());
	};
}

sem_action po::avr::binary_st(variable Rd1, variable Rd2, bool pre_dec, bool post_inc)
{
	assert(!(pre_dec == true && post_inc == true));

	return [=](sm &st)
	{
		variable X = "ptr"_var;
		
		st.mnemonic(0,"internal-ptr","",std::list<rvalue>(),[=](cg &c)
		{
			c.or_b(X,c.shiftl_u(Rd2,8_val),Rd1);
		});

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
			if(pre_dec) 
				c.sub_i(X,X,1_val);
			
			c.assign(sram(X),Rr);
			
			if(post_inc) 
				c.add_i(X,X,1_val);

			if(post_inc || pre_dec)
			{
				c.and_b(Rd1,X,0xff_val);
				c.shiftr_u(Rd2,X,8_val);
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
		variable X = "ptr"_var;
		
		st.mnemonic(0,"internal-ptr","",std::list<rvalue>(),[=](cg &c)
		{
			c.or_b(X,c.shiftl_u(Rr2,8_val),Rr1);
		});

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
			if(pre_dec) 
				c.sub_i(X,X,1_val);
			
			c.assign(Rd,sram(X));
			
			if(post_inc) 
				c.add_i(X,X,1_val);

			if(post_inc || pre_dec)
			{
				c.and_b(Rr1,X,0xff_val);
				c.shiftr_u(Rr2,X,8_val);
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
		variable X = "ptr"_var;
		
		st.mnemonic(0,"internal-ptr","",std::list<rvalue>(),[=](cg &c)
		{
			c.or_b(X,c.shiftl_u(Rd2,8_val),Rd1);
		});

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
			c.add_i(X,X,constant(q));
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
		variable X = "ptr"_var;
		
		st.mnemonic(0,"internal-ptr","",std::list<rvalue>(),[=](cg &c)
		{
			c.or_b(X,c.shiftl_u(Rr2,8_val),Rr1);
		});

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
			c.add_i(X,X,constant(q));
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

	m.slice("H"_var,m.or_b(m.or_b(
		m.and_b(a_not,b),
		m.and_b(b,c)),
		m.and_b(a_not,c)),
	3_val,3_val);
}

// V: a7•!b7•!c7 + !a7•b7•c7
// Two's complements overflow for c = a - b or a = b + c
void po::avr::two_complement_overflow(const rvalue &a, const rvalue &b, const rvalue &c, cg &m)
{
	m.slice("V"_var,
		m.or_b(
			m.and_b(m.and_b(a,m.not_b(b)),c),
			m.and_b(m.and_b(m.not_b(a),b),c)),
		7_val,7_val);
}

// !a7•!a6•!a5•!a4•!a3•!a2•!a1•!a0
rvalue po::avr::zero(const rvalue &a, cg &m)
{
	rvalue not_a = m.not_b(a);
	return m.and_b(m.slice(not_a,0_val,0_val),
					m.and_b(m.slice(not_a,1_val,1_val),
						m.and_b(m.slice(not_a,2_val,2_val),
							m.and_b(m.slice(not_a,3_val,3_val),
								m.and_b(m.slice(not_a,4_val,4_val),
									m.and_b(m.slice(not_a,5_val,5_val),
										m.and_b(m.slice(not_a,6_val,6_val),
											m.slice(not_a,7_val,7_val))))))));
}

// Zero flag for result a
void po::avr::is_zero(const rvalue &a, cg &m)
{
	m.assign("Z"_var,zero(a,m));
}

// C: !a7•b7 + b7•c7 + c7•!a7
// Carry for c = a - b or a = b + c
void po::avr::carry(const rvalue &a, const rvalue &b, const rvalue &c, cg &m)
{
	rvalue a_not = m.not_b(a);

	m.slice("C"_var,m.or_b(m.or_b(
		m.and_b(a_not,b),
		m.and_b(b,c)),
		m.and_b(a_not,c)),
	7_val,7_val);
}

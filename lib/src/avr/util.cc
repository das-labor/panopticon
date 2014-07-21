#include <functional>
#include <list>
#include <string>

#define AVR_PRIVATE
#include <panopticon/avr/avr.hh>
#include <panopticon/avr/util.hh>

using namespace po;
using namespace po::avr;
using namespace po::dsl;

// registers
const variable r0 = variable("r0",8), r1 = variable("r1",8), r2 = variable("r2",8), r3 = variable("r3",8), r4 = variable("r4",8), r5 = variable("r5",8), r6 = variable("r6",8),
							 r7 = variable("r7",8), r8 = variable("r8",8), r9 = variable("r9",8), r10 = variable("r10",8), r11 = variable("r11",8), r12 = variable("r12",8),
							 r13 = variable("r13",8), r14 = variable("r14",8), r15 = variable("r15",8), r16 = variable("r16",8), r17 = variable("r17",8), r18 = variable("r18",8),
							 r19 = variable("r19",8), r20 = variable("r20",8), r21 = variable("r21",8), r22 = variable("r22",8), r23 = variable("r23",8), r24 = variable("r24",8),
							 r25 = variable("r25",8), r26 = variable("r26",8), r27 = variable("r27",8), r28 = variable("r28",8), r29 = variable("r29",8), r30 = variable("r30",8),
							 r31 = variable("r31",1), I = variable("I",1), T = variable("T",1), H = variable("H",1), S = variable("S",1), V = variable("V",1), N = variable("N",1), Z = variable("Z",1), C = variable("C",1);

variable po::avr::decode_reg(unsigned int r)
{
	ensure(r <= 31);
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
		default: ensure(false);
	}

	switch(op)
	{
		case PostInc: name += "+"; break;
		case PreDec: name = "-" + name; break;
		case Nop: break;
		case PostDisplace: if(r != 26) { name += "+" + std::to_string(d); break; }
		default: ensure(false);
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
	return memory(o,1,BigEndian,"sram");
}

memory po::avr::sram(unsigned int o)
{
	return sram(constant(o));
}

memory po::avr::flash(rvalue o)
{
	return memory(o,1,BigEndian,"flash");
}

memory po::avr::flash(unsigned int o)
{
	return flash(constant(o));
}

sem_action po::avr::unary_reg(std::string x, std::function<void(cg &c, const variable &v)> func)
{
	return [x,func](sm &st)
	{
		variable op = st.capture_groups.count("d") ? decode_reg((unsigned int)st.capture_groups["d"]) :
																								 decode_reg((unsigned int)st.capture_groups["r"]);
		if(func)
			st.mnemonic(st.tokens.size() * 2,x,"{8}",op,std::bind(func,std::placeholders::_1,op));
		else
			st.mnemonic(st.tokens.size() * 2,x,"{8}",op);
		st.jump(st.address + st.tokens.size() * 2);
	};
}

sem_action po::avr::binary_reg(std::string x, std::function<void(cg &,const variable&,const variable&)> func)
{
	return [x,func](sm &st)
	{
		variable Rd = decode_reg(st.capture_groups["d"]);
		variable Rr = decode_reg(st.capture_groups["r"]);

		st.mnemonic(st.tokens.size() * 2,x,"{8}, {8}",Rd,Rr,bind(func,std::placeholders::_1,Rd,Rr));
		st.jump(st.address + st.tokens.size() * 2);
	};
}

sem_action po::avr::branch(std::string m, rvalue flag, bool set)
{
	return [m,flag,set](sm &st)
	{
		int64_t _k = st.capture_groups["k"] * 2;
		guard g(flag,relation::Eq,set ? constant(1) : constant(0));
		constant k((int8_t)(_k <= 63 ? _k : _k - 128));

		st.mnemonic(st.tokens.size() * 2,m,"{8:-}",k);
		st.jump(st.address + 2,g.negation());
		st.jump(st.address + k.content() + 2,g);
	};
}

sem_action po::avr::binary_regconst(std::string x, std::function<void(cg &,const variable&,const constant&)> func)
{
	return [x,func](sm &st)
	{
		variable Rd = decode_reg(st.capture_groups["d"] + 16);
		constant K(st.capture_groups["K"]);

		st.mnemonic(st.tokens.size() * 2,x,"{8}, {8}",{Rd,K},bind(func,std::placeholders::_1,Rd,K));
		st.jump(st.address + st.tokens.size() * 2);
	};
}

sem_action po::avr::binary_st(variable Rd1, variable Rd2, bool pre_dec, bool post_inc)
{
	ensure(!(pre_dec == true && post_inc == true));

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
			ensure(false);


		if(post_inc)
			fmt += "+";

		fmt += "}, {8}";

		st.mnemonic(st.tokens.size() * 2,"st",fmt,{X,Rr},[=](cg &c)
		{
			c.add_i(X,Rd2 * 0x100,Rd1);

			if(pre_dec)
				c.mod_i(X,X - 1,constant(0x10000));

			c.assign(sram(X),Rr);

			if(post_inc)
				c.mod_i(X,X + 1,constant(0x10000));
		});
		st.jump(st.address + st.tokens.size() * 2);
	};
}

sem_action po::avr::binary_ld(variable Rr1, variable Rr2, bool pre_dec, bool post_inc)
{
	ensure(!(pre_dec == true && post_inc == true));

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
			ensure(false);


		if(post_inc)
			fmt += "+";

		fmt += "}, {8}";

		st.mnemonic(st.tokens.size() * 2,"ld",fmt,{X,Rd},[=](cg &c)
		{
			c.add_i(X,Rr2 * 0x100 + Rr1);

			if(pre_dec)
				c.mod_i(X,X - 1,constant(0x10000));

			c.assign(Rd,sram(X));

			if(post_inc)
				c.mod_i(X,X + 1,constant(0x10000));
		});
		st.jump(st.address + st.tokens.size() * 2);
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
			ensure(false);

		fmt += "+" + std::to_string(q);

		fmt += "}, {8}";

		st.mnemonic(st.tokens.size() * 2,"st",fmt,{X,Rr},[=](cg &c)
		{
			c.add_i(X,Rd2 * 0x100 + Rd1 + constant(q));
			c.assign(sram(X),Rr);
		});
		st.jump(st.address + st.tokens.size() * 2);
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
			ensure(false);

		fmt += "+" + std::to_string(q);

		fmt += "}, {8}";

		st.mnemonic(st.tokens.size() * 2,"ld",fmt,{X,Rd},[=](cg &c)
		{
			c.add_i(X,Rr2 * 0x100 + Rr1 + constant(q));
			c.assign(Rd,sram(X));
		});
		st.jump(st.address + st.tokens.size() * 2);
	};
}

sem_action po::avr::simple(std::string x, std::function<void(cg&)> fn)
{
	return [x,fn](sm &st)
	{
		std::list<rvalue> nop;
		st.mnemonic(st.tokens.size() * 2,x,"",nop,fn);
		st.jump(st.address + st.tokens.size() * 2);
	};
}

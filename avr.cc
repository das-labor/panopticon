#include <iostream>
#include <iomanip>
#include <numeric>
#include <functional>
#include <algorithm>

#include "decoder.hh"
#include "avr.hh"

using namespace std;

typedef uint16_t token;
typedef vector<uint16_t>::iterator tokiter;

#define unary_reg(x) \
	[](sem_state<token,tokiter> &st)\
	{\
		if(st.capture_groups.count("d"))\
		{\
			st.add_mnemonic(area(st.address,st.address+st.tokens.size()),x,value_ptr(new reg((unsigned int)st.capture_groups["d"])));\
			st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());\
		}\
		else\
		{\
			st.add_mnemonic(area(st.address,st.address+st.tokens.size()),x,value_ptr(new reg((unsigned int)st.capture_groups["r"])));\
			st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());\
		}\
	};

#define binary_reg(x) \
	[](sem_state<token,tokiter> &st)\
	{\
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),x,value_ptr(new reg(st.capture_groups["d"])),value_ptr(new reg(st.capture_groups["r"])));\
		st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());\
	};

#define branch(x) \
	[](sem_state<token,tokiter> &st) \
	{	\
		int k = st.capture_groups["k"];\
		k = k <= 63 ? k : k - 128;\
		\
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),x,k);\
		st.unconditional(st.mnemonics.begin()->second,st.address + 1);\
		st.unconditional(st.mnemonics.begin()->second,st.address + k + 1);\
	};

#define binary_regconst(x)\
	[](sem_state<token,tokiter> &st)\
	{\
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),x,value_ptr(new reg(st.capture_groups["d"] + 16)),st.capture_groups["K"]);\
		st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());\
	};

#define binary_st(r)\
	[](sem_state<token,tokiter> &st)\
	{\
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"st",value_ptr(r),value_ptr(new reg(st.capture_groups["r"])));\
		st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());\
	};

#define binary_stq(r)\
	[](sem_state<token,tokiter> &st)\
	{\
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"st",value_ptr(new reg(r,reg::PostDisplace,st.capture_groups["q"])),value_ptr(new reg(st.capture_groups["r"])));\
		st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());\
	};

#define binary_ld(r)\
	[](sem_state<token,tokiter> &st)\
	{\
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"ld",value_ptr(new reg(st.capture_groups["r"])),value_ptr(r));\
		st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());\
	};

#define binary_ldq(r)\
	[](sem_state<token,tokiter> &st)\
	{\
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"ld",value_ptr(new reg(st.capture_groups["r"])),value_ptr(new reg(r,reg::PostDisplace,st.capture_groups["q"])));\
		st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());\
	};

#define simple(x)\
	[](sem_state<token,tokiter> &st)\
	{\
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),x);\
		st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());\
	};

flow_ptr avr_decode(vector<token> &bytes, addr_t entry)
{
	decoder<token,vector<token>::iterator> main;

	// memory operations
	main | "001011 r@. d@..... r@...." 	= binary_reg("mov");
	main | "00000001 d@.... r@...." 		= [](sem_state<token,tokiter> &st)
	{ 
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"movw",
										value_ptr(new reg(st.capture_groups["d"],st.capture_groups["d"] + 1)),
										value_ptr(new reg(st.capture_groups["r"],st.capture_groups["r"] + 1)));
		st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());
	};
	main | "10110 A@.. d@..... A@...." 	= [](sem_state<token,tokiter> &st)
	{
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"in",
										value_ptr(new reg(st.capture_groups["d"])),
										value_ptr(new ioreg(st.capture_groups["A"])));
		st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());
	};
	main | "10111 A@.. r@..... A@...." 	= [](sem_state<token,tokiter> &st) 	
	{
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"out",
										value_ptr(new ioreg(st.capture_groups["A"])),
										value_ptr(new reg(st.capture_groups["r"])));
		st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());
	};
	main | "1001000 d@..... 1111"				= unary_reg("pop");
	main | "1001001 d@..... 1111" 			= unary_reg("push");
	main | "1001010 d@..... 0010" 			= unary_reg("swap");
	main | "1001001 r@..... 0100" 			= unary_reg("xch");
	main | "1110 K@.... d@.... K@...."	= binary_regconst("ldi");
	main | "11101111 d@.... 1111" 			= unary_reg("ser");
	main | "1001001 r@..... 0110" 			= unary_reg("lac");
	main | "1001001 r@..... 0101" 			= unary_reg("las");
	main | "1001001 r@..... 0111" 			= unary_reg("lat");
	main | "1001000 d@..... 0000" | "k@................" = [](sem_state<token,tokiter> &st)
	{
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"lds",value_ptr(new reg(st.capture_groups["d"])),st.capture_groups["k"]);
		st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());
	};

	main | "10100 k@... d@.... k@...." 	= [](sem_state<token,tokiter> &st)
	{
		unsigned int k = st.capture_groups["k"];

		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"lds",value_ptr(new reg(st.capture_groups["d"] + 16)),(~k & 16) | (k & 16) | (k & 64) | (k & 32) | (k & 15));
		st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());
	};

	main | 0x95c8 											= 	[](sem_state<token,tokiter> &st)
	{
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"lpm",value_ptr(new reg(0)),value_ptr(new reg(reg::Z)));
		st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());
	};

	main | 0x95e8 											= [](sem_state<token,tokiter> &st)
	{
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"spm");
		st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());
	};

	main | 0x95f8 											= [](sem_state<token,tokiter> &st)
	{
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"spm",value_ptr(new reg(reg::Z,reg::PostInc)));
		st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());
	};

	main | "1001001 d@..... 0000" | "k@................" = [](sem_state<token,tokiter> &st)
	{
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"sts",st.capture_groups["k"],value_ptr(new reg(st.capture_groups["r"])));
		st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());
	};

	main | "10101 k@... d@.... k@...." 	= [](sem_state<token,tokiter> &st)
	{
		unsigned int k = st.capture_groups["k"];

		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"sts",(~k & 16) | (k & 16) | (k & 64) | (k & 32) | (k & 15),value_ptr(new reg(st.capture_groups["r"])));
		st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());
	};

	main | "10011010 A@..... b@..." 			= [](sem_state<token,tokiter> &st)
	{
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"sbi",value_ptr(new ioreg(st.capture_groups["A"])),st.capture_groups["b"]);
		st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());
	};

	main | "10011000 A@..... b@..." 			= [](sem_state<token,tokiter> &st)
	{
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"cbi",value_ptr(new ioreg(st.capture_groups["A"])),st.capture_groups["b"]);
		st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());
	};

	// SREG operations
	//main | "100101001 s@... 1000" = simple("bclr");
	//main | "100101000 s@... 1000" = simple("bset");
	main | 0x9408 = simple("sec");
	main | 0x9458 = simple("seh");
	main | 0x9478 = simple("sei");
	main | 0x9428 = simple("sen");
	main | 0x9448 = simple("ses");
	main | 0x9468 = simple("set");
	main | 0x9438 = simple("sev");
	main | 0x9418 = simple("sez");
	main | 0x9488 = simple("clc");
	main | 0x94d8 = simple("clh");
	main | 0x94f8 = simple("cli");
	main | 0x94a8 = simple("cln");
	main | 0x94c8 = simple("cls");
	main | 0x94e8 = simple("clt");
	main | 0x94b8 = simple("clv");
	main | 0x9498 = simple("clz");
	main | "000101 r@. d@..... r@...." 	= binary_reg("cp");
	main | "000001 r@. d@..... r@...." 	= binary_reg("cpc");
	main | "0011 K@.... d@.... K@...." 	= binary_regconst("cpi");
	main | "001000 d@.........." 				= unary_reg("tst");
	
	// bit-level logic
	main | "0110 K@.... d@.... K@...." 	= binary_regconst("sbr");
	main | "000011 d@.........."				= unary_reg("lsl");
	main | "1001010 d@..... 0110"				= unary_reg("lsr");

	// byte-level arithmetic and logic
	main | "000111 r@. d@..... r@...."	= binary_reg("adc");
	main | "000011 r@. d@..... r@...." 	= binary_reg("add");
	main | "001000 r@. d@..... r@...." 	= binary_reg("and");
	main | "0111 K@.... d@.... K@...." 	= binary_regconst("andi");
	main | "001001 r@. d@..... r@...." 	= [](sem_state<token,tokiter> &st)
	{
		int d = st.capture_groups["d"];
		int r = st.capture_groups["r"];

		if(d == r)
		{
			st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"clr",value_ptr(new reg(st.capture_groups["d"])));
			st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());
		}
		else
		{
			st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"eor",value_ptr(new reg(st.capture_groups["d"])),value_ptr(new reg(st.capture_groups["d"])));
			st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());
		}
	};
	main | "1001010 d@..... 0001"				= unary_reg("neg");
	main | "001010 r@. d@..... r@...." 	= binary_reg("or");
	main | "0110 K@.... d@.... K@...." 	= binary_regconst("ori");
	main | "000110 r@. d@..... r@...." 	= binary_reg("sub");
	main | "0101 K@.... d@.... K@...." 	= binary_regconst("subi");
	main | "1001010 d@..... 0101" 			= unary_reg("asr");
	main | "000111 d@.........." 				= unary_reg("rol");
	main | "1001010 d@..... 0111" 			= unary_reg("ror");
	main | "1001010 d@..... 1010" 			= unary_reg("dec");
	main | "1001010 d@..... 0011" 			= unary_reg("inc");
	main | "000010 r@. d@..... r@...." 	= binary_reg("sbc");
	main | "0100 K@.... d@.... K@...." 	= binary_regconst("sbci");
	main | "1001010 d@..... 0000" 			= unary_reg("com");

	// word-level arithmetic and logic
	main | "10010110 K@.. d@.. K@...." = [](sem_state<token,tokiter> &st)
	{
		unsigned int d = (unsigned int)st.capture_groups["d"] * 2 + 24;
		
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"adiw",value_ptr(new reg(d,d+1)),st.capture_groups["K"]);
		st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());
	};
	main | "10010111 K@.. d@.. K@...." = [](sem_state<token,tokiter> &st) 
	{
		unsigned int d = (unsigned int)st.capture_groups["d"] * 2 + 24;
		
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"sbiw",value_ptr(new reg(d,d+1)),st.capture_groups["K"]);
		st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());
	};
	main | "0000 0011 0 d@... 1 r@..."	= binary_reg("fmul");
	main | "000000111 d@... 0 r@..."		= binary_reg("fmuls");
	main | "000000111 d@... 1 r@..." 		= binary_reg("fmulsu");
	main | "100111 r@. d@..... r@...." 	= binary_reg("mul");
	main | "00000010 d@.... r@...." 		= binary_reg("muls");
	main | "000000110 d@... 0 r@..." 		= binary_reg("muls");
	
	
	// conditional branches
	// main | "111101 k@....... s@..." = simple("brbc");
	// main | "111100 k@....... s@..." = [](sem_state<token,tokiter> &st)  { st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"brbs"; });
	main | "111101 k@....... 000" 			= branch("brcc");
	main | "111100 k@....... 000" 			= branch("brcs");
	main | "111100 k@....... 001" 			= branch("breq");
	main | "111101 k@....... 100" 			= branch("brge");
	main | "111101 k@....... 101" 			= branch("brhc");
	main | "111100 k@....... 101" 			= branch("brhs");
	main | "111101 k@....... 111" 			= branch("brid");
	main | "111100 k@....... 111" 			= branch("brie");
	main | "111100 k@....... 000" 			= branch("brlo");
	main | "111100 k@....... 100" 			= branch("brlt");
	main | "111100 k@....... 010" 			= branch("brmi");
	main | "111101 k@....... 001"		 		= branch("brne");
	main | "111101 k@....... 010" 			= branch("brpl");
	main | "111101 k@....... 000" 			= branch("brsh");
	main | "111101 k@....... 110" 			= branch("brtc");
	main | "111100 k@....... 110" 			= branch("brts");
	main | "111101 k@....... 011" 			= branch("brvc");
	main | "111100 k@....... 011" 			= branch("brvs");
	main | "1111 110r@..... 0 b@..." 		= [](sem_state<token,tokiter> &st)
	{
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"sbrc",value_ptr(new reg(st.capture_groups["r"])),st.capture_groups["b"]);
		st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());
		//st.skip_next = true;
	};
	main | "1111 111 r@..... 0 b@..." 		= [](sem_state<token,tokiter> &st)
	{
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"sbrs",value_ptr(new reg(st.capture_groups["r"])),st.capture_groups["b"]);
		st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());
		//st.skip_next = true;
	};
	main | "000100 r@. d@..... r@...."	= [](sem_state<token,tokiter> &st)
	{
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"cpse",value_ptr(new reg(st.capture_groups["d"])),value_ptr(new reg(st.capture_groups["r"])));
		st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());
		//st.skip_next = true;
	};
	main | "1001 1001 A@..... b@..." 		= [](sem_state<token,tokiter> &st)
	{
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"sbic",value_ptr(new ioreg(st.capture_groups["A"])),st.capture_groups["b"]);
		st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());
		//st.skip_next = true;
	};
	main | "1001 1011 A@..... b@..." 		= [](sem_state<token,tokiter> &st)
	{
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"sbis",value_ptr(new ioreg(st.capture_groups["A"])),st.capture_groups["b"]);
		st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());
		//st.skip_next = true;
	};

	// unconditional branches
	main | "1001010 k@..... 111 k@." | "k@................"	= [](sem_state<token,tokiter> &st) 
	{
		int k = st.capture_groups["k"];
		
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"call",k)
			->call("t",k);
		st.unconditional(st.mnemonics.begin()->second,st.address + st.tokens.size());
	//	st.unconditional(st.mnemonics.begin()->second,k);
		//st.is_call = true;
	};
	main | "1001010 k@..... 110 k@." | "k@................"	= [](sem_state<token,tokiter> &st) 
	{ 
		int k = st.capture_groups["k"];
		
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"jmp",k);
		st.unconditional(st.mnemonics.begin()->second,k);
	};

	main | "1101 k@............" 														= [](sem_state<token,tokiter> &st) 
	{
		int k = st.capture_groups["k"];

		k = (k <= 2047 ? k : k - 4096);
		
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"rcall",k)
			->call("t",k + 1 + st.address);
		st.unconditional(st.mnemonics.begin()->second,st.address + 1);
		//st.unconditional(st.mnemonics.begin()->second,k + 1 + st.address);
		//st.is_call = true;
	};
	main | "1100 k@............" 														= [](sem_state<token,tokiter> &st) 
	{
		int k = st.capture_groups["k"];

		k = (k <= 2047 ? k : k - 4096);
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"rjmp",k);
		st.unconditional(st.mnemonics.begin()->second,k + 1 + st.address);
	};
	main | 0x9508 = [](sem_state<token,tokiter> &st) { st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"ret"); };
	main | 0x9518 = [](sem_state<token,tokiter> &st) { st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"reti"); };
	main | 0x9409 = [](sem_state<token,tokiter> &st) { st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"ijmp"); };
	main | 0x9509 = [](sem_state<token,tokiter> &st) { st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"icall"); };
	// icall
	
	// store and load with x,y,z
	main | "1001 001r@. r@.... 1100" = binary_st(new reg(reg::X));
	main | "1001 001r@. r@.... 1101" = binary_st(new reg(reg::X,reg::PostInc));
	main | "1001 001r@. r@.... 1110" = binary_st(new reg(reg::X,reg::PreDec));
	main | "1000 001r@. r@.... 1000" = binary_st(new reg(reg::Y));
	main | "1001 001r@. r@.... 1001" = binary_st(new reg(reg::Y,reg::PostInc));
	main | "1001 001r@. r@.... 1010" = binary_st(new reg(reg::Y,reg::PreDec));
	main | "10q@.0 q@..1r@. r@.... 1q@..." = binary_stq(reg::Y);
	main | "1000 001r@. r@.... 0000" = binary_st(new reg(reg::Z));
	main | "1001 001r@. r@.... 0001" = binary_st(new reg(reg::Z,reg::PostInc));
	main | "1001 001r@. r@.... 0010" = binary_st(new reg(reg::Z,reg::PreDec));
	main | "10q@.0 q@..1r@. r@.... 0q@..." = binary_stq(reg::Z);
	main | "1001 000d@. d@.... 1100" = binary_ld(new reg(reg::X));
	main | "1001 000d@. d@.... 1101" = binary_ld(new reg(reg::X,reg::PostInc));
	main | "1001 000d@. d@.... 1110" = binary_ld(new reg(reg::X,reg::PreDec));
	main | "1000 000d@. d@.... 1000" = binary_ld(new reg(reg::Y));
	main | "1001 000d@. d@.... 1001" = binary_ld(new reg(reg::Y,reg::PostInc));
	main | "1001 000d@. d@.... 1010" = binary_ld(new reg(reg::Y,reg::PreDec));
	main | "10 q@. 0 q@.. 0 d@..... 1 q@..." = binary_ldq(reg::Y);
	main | "1000 000d@. d@.... 0000" = binary_ld(new reg(reg::Z));
	main | "1001 000 d@..... 0001" = binary_ld(new reg(reg::Z,reg::PostInc));
	main | "1001 000d@. d@.... 0010" = binary_ld(new reg(reg::Z,reg::PreDec));
	main | "10q@.0 q@..0d@. d@.... 0q@..." = binary_ldq(reg::Z);

	// misc
	main | 0x9598 = simple("break");
	main | "10010100 K@.... 1011" = [](sem_state<token,tokiter> &st) 
	{
		st.add_mnemonic(area(st.address,st.address+st.tokens.size()),"des",st.capture_groups["K"]);
		st.unconditional(st.mnemonics.begin()->second,st.tokens.size() + st.address);
	};

	main | (token)0x0 = simple("nop");
	main | 0x9588 = simple("sleep");
	main | 0x95a8 = simple("wdr");

	// catch all
	main = simple("unk");

	return disassemble<token,tokiter>(main,bytes);
}

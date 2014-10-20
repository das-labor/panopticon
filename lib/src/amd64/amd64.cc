#include <panopticon/disassembler.hh>
#include <panopticon/amd64/amd64.hh>

using namespace po;
using namespace po::amd64;
using namespace po::dsl;

namespace po
{
	namespace amd64
	{
		unsigned int next_unused = 0;
		std::vector<std::string> registers({
			"al","ah","ax","eax","rax",
			"bl","bh","bx","ebx","rbx",
			"cl","ch","cx","ecx","rcx",
			"dh","dl","dx","edx","rdx",
			"r4","r5","r6","r7",
			"ebp","rbp","esp","rsp",
			"eip","rip","eflags","rflags"
		});
	}
}

template<>
lvalue po::temporary(amd64_tag)
{
	return variable("t" + std::to_string(po::amd64::next_unused++),64);
}

template<>
const std::vector<std::string> &po::registers(amd64_tag)
{
	return po::amd64::registers;
}

template<>
uint8_t po::width(std::string n, amd64_tag)
{
	ensure(n.size() >= 2);

	if(n.c_str()[0] == 'r')
		return 64;
	else if(n.c_str()[0] == 'e')
		return 32;
	else if(n.c_str()[0] == 't')
		return 64;
	else if(n.c_str()[1] == 'l' || n.c_str()[1] == 'h')
		return 8;
	else if(n.c_str()[1] == 'x')
		return 16;
	else
		ensure(false);
}

namespace pls = std::placeholders;

boost::optional<prog_loc> po::amd64::disassemble(boost::optional<prog_loc> prog, po::slab bytes, const po::ref& r)
{
	disassembler<amd64_tag> main, opsize_prfix, rex_prfix, rexw_prfix,
									imm8_a, imm16_a, imm32_a, imm64_a,
									imm8_b, imm16_b, imm32_b, imm64_b,
									imm8_c, imm16_c, imm32_c, imm64_c,
									modrm0, modrm1, modrm2,
									modrm3, modrm4, modrm5, modrm6,
									modrm7, sib,
									rm8, rm16, rm32, rm64,
									rm8_0, rm16_0, rm32_0, rm64_0,
									rm8_1, rm16_1, rm32_1, rm64_1,
									rm8_2, rm16_2, rm32_2, rm64_2,
									rm8_3, rm16_3, rm32_3, rm64_3,
									rm8_4, rm16_4, rm32_4, rm64_4,
									rm8_5, rm16_5, rm32_5, rm64_5,
									rm8_6, rm16_6, rm32_6, rm64_6,
									rm8_7, rm16_7, rm32_7, rm64_7;

	std::function<void(const std::string&,int,sm&)> simple = [&](const std::string& m,int j,sm& st)
	{
		std::cerr << m << ": " << std::hex;
		for(auto b: st.tokens)
			std::cerr << (unsigned int)b << " ";
		std::cerr << std::dec << std::endl;
		st.jump(st.address + st.tokens.size());
	};

	opsize_prfix | 0x66 = [](sm& st) {};
	rex_prfix | "01000 r@. x@. b@." = [](sm& st) {};
	rexw_prfix | "01001 r@. x@. b@." = [](sm& st) {};

	imm8_a | "a@........" = [](sm& st) {};
	imm8_b | "b@........" = [](sm& st) {};
	imm8_c | "a@........" = [](sm& st) {};

	imm16_a | imm8_a | "a@........" = [](sm& st) {};
	imm16_b | imm8_b | "b@........" = [](sm& st) {};
	imm16_c | imm8_c | "c@........" = [](sm& st) {};

	imm32_a | imm16_a | "a@........" | "a@........" = [](sm& st) {};
	imm32_b | imm16_b | "b@........" | "b@........" = [](sm& st) {};
	imm32_c | imm16_c | "c@........" | "c@........" = [](sm& st) {};

	imm64_a | imm32_a | "a@........" | "a@........" | "a@........" | "a@........" = [](sm& st) {};
	imm64_b | imm32_b | "b@........" | "b@........" | "b@........" | "b@........" = [](sm& st) {};
	imm64_c | imm32_c | "c@........" | "c@........" | "c@........" | "c@........" = [](sm& st) {};

	// 64 bit
	disassembler<amd64_tag> imm_a(imm32_a), imm_b(imm32_b), imm_c(imm32_c);

	modrm0 | "mod@.. 000 rm@..."  = [](sm& st) {};
	modrm1 | "mod@.. 001 rm@..."  = [](sm& st) {};
	modrm2 | "mod@.. 010 rm@..."  = [](sm& st) {};
	modrm3 | "mod@.. 011 rm@..."  = [](sm& st) {};
	modrm4 | "mod@.. 100 rm@..."  = [](sm& st) {};
	modrm5 | "mod@.. 101 rm@..."  = [](sm& st) {};
	modrm6 | "mod@.. 110 rm@..."  = [](sm& st) {};
	modrm7 | "mod@.. 111 rm@..."  = [](sm& st) {};

	// direct addressing
	rm8 | "00 reg@... rm@..." = [](sm& st) {};
	rm16 | "00 reg@... rm@..." = [](sm& st) {};
	rm32 | "00 reg@... rm@..." = [](sm& st) {};
	rm64 | "00 reg@... rm@..." = [](sm& st) {};

	rm8_0 | "00 000 rm@..." = [](sm& st) {};
	rm16_0 | "00 000 rm@..." = [](sm& st) {};
	rm32_0 | "00 000 rm@..." = [](sm& st) {};
	rm64_0 | "00 000 rm@..." = [](sm& st) {};

	rm8_1 | "00 001 rm@..." = [](sm& st) {};
	rm16_1 | "00 001 rm@..." = [](sm& st) {};
	rm32_1 | "00 001 rm@..." = [](sm& st) {};
	rm64_1 | "00 001 rm@..." = [](sm& st) {};

	rm8_2 | "00 010 rm@..." = [](sm& st) {};
	rm16_2 | "00 010 rm@..." = [](sm& st) {};
	rm32_2 | "00 010 rm@..." = [](sm& st) {};
	rm64_2 | "00 010 rm@..." = [](sm& st) {};

	rm8_3 | "00 011 rm@..." = [](sm& st) {};
	rm16_3 | "00 011 rm@..." = [](sm& st) {};
	rm32_3 | "00 011 rm@..." = [](sm& st) {};
	rm64_3 | "00 011 rm@..." = [](sm& st) {};

	rm8_4 | "00 100 rm@..." = [](sm& st) {};
	rm16_4 | "00 100 rm@..." = [](sm& st) {};
	rm32_4 | "00 100 rm@..." = [](sm& st) {};
	rm64_4 | "00 100 rm@..." = [](sm& st) {};

	rm8_5 | "00 101 rm@..." = [](sm& st) {};
	rm16_5 | "00 101 rm@..." = [](sm& st) {};
	rm32_5 | "00 101 rm@..." = [](sm& st) {};
	rm64_5 | "00 101 rm@..." = [](sm& st) {};

	rm8_6 | "00 110 rm@..." = [](sm& st) {};
	rm16_6 | "00 110 rm@..." = [](sm& st) {};
	rm32_6 | "00 110 rm@..." = [](sm& st) {};
	rm64_6 | "00 110 rm@..." = [](sm& st) {};

	rm8_7 | "00 111 rm@..." = [](sm& st) {};
	rm16_7 | "00 111 rm@..." = [](sm& st) {};
	rm32_7 | "00 111 rm@..." = [](sm& st) {};
	rm64_7 | "00 111 rm@..." = [](sm& st) {};

	// indirect addressing
	rm8 | "11 reg@... rm@..." | imm8_a = [](sm& st) {};
	rm16 | "11 reg@... rm@..." | imm16_a = [](sm& st) {};
	rm32 | "11 reg@... rm@..." | imm32_a = [](sm& st) {};
	rm64 | "11 reg@... rm@..." | imm64_a = [](sm& st) {};

	rm8_0 | "11 000 rm@..." | imm8_a = [](sm& st) {};
	rm16_0 | "11 000 rm@..." | imm16_a = [](sm& st) {};
	rm32_0 | "11 000 rm@..." | imm32_a = [](sm& st) {};
	rm64_0 | "11 000 rm@..." | imm64_a = [](sm& st) {};

	rm8_1 | "11 001 rm@..." | imm8_a = [](sm& st) {};
	rm16_1 | "11 001 rm@..." | imm16_a = [](sm& st) {};
	rm32_1 | "11 001 rm@..." | imm32_a = [](sm& st) {};
	rm64_1 | "11 001 rm@..." | imm64_a = [](sm& st) {};

	rm8_2 | "11 010 rm@..." | imm8_a = [](sm& st) {};
	rm16_2 | "11 010 rm@..." | imm16_a = [](sm& st) {};
	rm32_2 | "11 010 rm@..." | imm32_a = [](sm& st) {};
	rm64_2 | "11 010 rm@..." | imm64_a = [](sm& st) {};

	rm8_3 | "11 011 rm@..." | imm8_a = [](sm& st) {};
	rm16_3 | "11 011 rm@..." | imm16_a = [](sm& st) {};
	rm32_3 | "11 011 rm@..." | imm32_a = [](sm& st) {};
	rm64_3 | "11 011 rm@..." | imm64_a = [](sm& st) {};

	rm8_4 | "11 100 rm@..." | imm8_a = [](sm& st) {};
	rm16_4 | "11 100 rm@..." | imm16_a = [](sm& st) {};
	rm32_4 | "11 100 rm@..." | imm32_a = [](sm& st) {};
	rm64_4 | "11 100 rm@..." | imm64_a = [](sm& st) {};

	rm8_5 | "11 101 rm@..." | imm8_a = [](sm& st) {};
	rm16_5 | "11 101 rm@..." | imm16_a = [](sm& st) {};
	rm32_5 | "11 101 rm@..." | imm32_a = [](sm& st) {};
	rm64_5 | "11 101 rm@..." | imm64_a = [](sm& st) {};

	rm8_6 | "11 110 rm@..." | imm8_a = [](sm& st) {};
	rm16_6 | "11 110 rm@..." | imm16_a = [](sm& st) {};
	rm32_6 | "11 110 rm@..." | imm32_a = [](sm& st) {};
	rm64_6 | "11 110 rm@..." | imm64_a = [](sm& st) {};

	rm8_7 | "11 111 rm@..." | imm8_a = [](sm& st) {};
	rm16_7 | "11 111 rm@..." | imm16_a = [](sm& st) {};
	rm32_7 | "11 111 rm@..." | imm32_a = [](sm& st) {};
	rm64_7 | "11 111 rm@..." | imm64_a = [](sm& st) {};

	sib | "scale@.. index@... base@..."  = [](sm& st) {};

	// 32 bits only
	main | 0x37 = std::bind(simple,"AAA",1,std::placeholders::_1);
	main | 0xd5 | "i@........" = std::bind(simple,"AAD imm8",2,std::placeholders::_1);
	main | 0xd4 | "i@........" = std::bind(simple,"AAM imm8",2,std::placeholders::_1);
	main | 0x3f = std::bind(simple,"AAS",1,std::placeholders::_1);

	// ADC
	main						| 0x14 | imm8_a				= std::bind(simple,"ADC AL, imm8",2,pls::_1);

	main | opsize_prfix	| 0x15 | imm16_a				= std::bind(simple,"ADC AX, imm16",4,pls::_1);
	main						| 0x15 | imm32_a				= std::bind(simple,"ADC EAX, imm32",5,pls::_1);
	main | rexw_prfix		| 0x15 | imm32_a				= std::bind(simple,"ADC RAX, imm32",6,pls::_1);

	main 						| 0x80 | rm8_2 | imm8_b	= std::bind(simple,"ADC r/m8, imm8",3,pls::_1);
	main | rex_prfix		| 0x80 | rm8_2 | imm8_b	= std::bind(simple,"ADC r/m8, imm8",4,pls::_1);

	main | opsize_prfix	| 0x81 | rm16_2 | imm16_b	= std::bind(simple,"ADC r/m16, imm16",5,pls::_1);
	main 						| 0x81 | rm32_2 | imm32_b	= std::bind(simple,"ADC r/m32, imm32",6,pls::_1);
	main | rexw_prfix		| 0x81 | rm64_2 | imm32_b	= std::bind(simple,"ADC r/m64, imm32",7,pls::_1);

	main | opsize_prfix	| 0x83 | rm16_2 | imm8_b	= std::bind(simple,"ADC r/m16, imm8",4,pls::_1);
	main 						| 0x83 | rm32_2 | imm8_b	= std::bind(simple,"ADC r/m32, imm8",3,pls::_1);
	main | rexw_prfix		| 0x83 | rm64_2 | imm8_b	= std::bind(simple,"ADC r/m64, imm8",4,pls::_1);

	main 						| 0x10 | rm8		= std::bind(simple,"ADC r/m8, r8",6,pls::_1);
	main | rex_prfix		| 0x10 | rm8		= std::bind(simple,"ADC r/m8, r8",7,pls::_1);

	main | opsize_prfix	| 0x11 | rm16					= std::bind(simple,"ADC r/m16, r16",3,pls::_1);
	main 						| 0x11 | rm32					= std::bind(simple,"ADC r/m32, r32",2,pls::_1);
	main | rexw_prfix		| 0x11 | rm64					= std::bind(simple,"ADC r/m64, r64",3,pls::_1);

	main 						| 0x12 | rm8					= std::bind(simple,"ADC r8, r/m8",2,pls::_1);
	main | rex_prfix		| 0x12 | rm8					= std::bind(simple,"ADC r8, r/m8",3,pls::_1);

	main | opsize_prfix	| 0x13 | rm16					= std::bind(simple,"ADC r16, r/m16",3,pls::_1);
	main 						| 0x13 | rm32					= std::bind(simple,"ADC r32, r/m32",2,pls::_1);
	main | rexw_prfix		| 0x13 | rm64					= std::bind(simple,"ADC r64, r/m64",3,pls::_1);

	return program::disassemble<amd64_tag>(main,bytes,r,prog);
}

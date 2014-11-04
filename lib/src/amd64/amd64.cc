#include <panopticon/disassembler.hh>

#include <panopticon/amd64/amd64.hh>
#include <panopticon/amd64/util.hh>

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

// 8 bit gp registers
const variable al = variable("al",8),
					bl = variable("bl",8),
					cl = variable("cl",8),
					dl = variable("dl",8),
					ah = variable("al",8),
					bh = variable("bl",8),
					ch = variable("cl",8),
					dh = variable("dl",8),

// 16 bit gp registers
					ax = variable("ax",16),
					bx = variable("bx",16),
					cx = variable("cx",16),
					dx = variable("dx",16),

// 32 bit gp registers
					eax = variable("eax",32),
					ebx = variable("ebx",32),
					ecx = variable("ecx",32),
					edx = variable("edx",32),

// 64 bit gp registers
					rax = variable("rax",64),
					rbx = variable("rbx",64),
					rcx = variable("rcx",64),
					rdx = variable("rdx",64),
					r4 = variable("r4",64),
					r5 = variable("r5",64),
					r6 = variable("r6",64),
					r7 = variable("r7",64),

// 32 bit management registers
					esp = variable("esp",32),
					ebp = variable("ebp",32),
					eip = variable("eip",32),
					//eflags = variable("eflags",32),
					CF = variable("CF",1),
					PF = variable("PF",1),
					AF = variable("AF",1),
					ZF = variable("ZF",1),
					SF = variable("SF",1),
					TF = variable("TF",1),
					IF = variable("IF",1),
					DF = variable("DF",1),
					OF = variable("OF",1),
					IOPL = variable("IOPL",2),
					NT = variable("NT",1),
					RF = variable("RF",1),
					VM = variable("VM",1),
					AC = variable("AC",1),
					VIF = variable("VIF",1),
					VIP = variable("VIP",1),
					ID = variable("ID",1),

// 64 bit management registers
					rsp = variable("rsp",64),
					rbp = variable("rbp",64),
					rip = variable("rip",64),
					rflags = variable("rflags",64);

namespace pls = std::placeholders;

boost::optional<prog_loc> po::amd64::disassemble(boost::optional<prog_loc> prog, po::slab bytes, const po::ref& r)
{
	disassembler<amd64_tag> main, opsize_prfix, rex_prfix, rexw_prfix,
									generic_prfx, addrsize_prfx, rep_prfx,
									imm8_a, imm16_a, imm32_a, imm64_a,
									imm8_b, imm16_b, imm32_b, imm64_b,
									imm8_c, imm16_c, imm32_c, imm64_c,
									sib,
									rm8, rm16, rm32, rm64,
									rm8_0, rm16_0, rm32_0, rm64_0,
									rm8_1, rm16_1, rm32_1, rm64_1,
									rm8_2, rm16_2, rm32_2, rm64_2,
									rm8_3, rm16_3, rm32_3, rm64_3,
									rm8_4, rm16_4, rm32_4, rm64_4,
									rm8_5, rm16_5, rm32_5, rm64_5,
									rm8_6, rm16_6, rm32_6, rm64_6,
									rm8_7, rm16_7, rm32_7, rm64_7;

	opsize_prfix[0x66] = [](sm& st) {};
	addrsize_prfx[0x67] = [](sm& st) {};
	rep_prfx[0xf3] = [](sm& st) {};
	rex_prfix [ "01000 r@. x@. b@."_e] = [](sm& st) {};
	rexw_prfix [ "01001 r@. x@. b@."_e] = [](sm& st) {};

	generic_prfx [rep_prfx]			= [](sm& st) {};
	generic_prfx [addrsize_prfx]	= [](sm& st) {};

	imm8_a [ "a@........"_e] = [](sm& st) { st.state.operand_a = constant(st.capture_groups.at("a")); };
	imm8_b [ "b@........"_e] = [](sm& st) { st.state.operand_b = constant(st.capture_groups.at("b")); };
	imm8_c [ "a@........"_e] = [](sm& st) { st.state.operand_c = constant(st.capture_groups.at("c")); };

	imm16_a [ imm8_a >> "a@........"_e] = [](sm& st) { st.state.operand_a = constant(st.capture_groups.at("a")); };
	imm16_b [ imm8_b >> "b@........"_e] = [](sm& st) { st.state.operand_b = constant(st.capture_groups.at("b")); };
	imm16_c [ imm8_c >> "c@........"_e] = [](sm& st) { st.state.operand_c = constant(st.capture_groups.at("c")); };

	imm32_a [ imm16_a >> "a@........"_e >> "a@........"_e] = [](sm& st) { st.state.operand_a = constant(st.capture_groups.at("a")); };
	imm32_b [ imm16_b >> "b@........"_e >> "b@........"_e] = [](sm& st) { st.state.operand_b = constant(st.capture_groups.at("b")); };
	imm32_c [ imm16_c >> "c@........"_e >> "c@........"_e] = [](sm& st) { st.state.operand_c = constant(st.capture_groups.at("c")); };

	imm64_a [ imm32_a >> "a@........"_e >> "a@........"_e >> "a@........"_e >> "a@........"_e] = [](sm& st) { st.state.operand_a = constant(st.capture_groups.at("a")); };
	imm64_b [ imm32_b >> "b@........"_e >> "b@........"_e >> "b@........"_e >> "b@........"_e] = [](sm& st) { st.state.operand_b = constant(st.capture_groups.at("b")); };
	imm64_c [ imm32_c >> "c@........"_e >> "c@........"_e >> "c@........"_e >> "c@........"_e] = [](sm& st) { st.state.operand_c = constant(st.capture_groups.at("c")); };

	// 64 bit
	disassembler<amd64_tag> imm_a(imm32_a), imm_b(imm32_b), imm_c(imm32_c);

	// sib
	sib [ "00 index@... base@..."_e >> imm32_a ] = [](sm& st) {};
	sib [ "01 index@... base@..."_e >> imm8_a ] = [](sm& st) {};
	sib [ "10 index@... base@..."_e >> imm32_a ] = [](sm& st) {};
	sib [ "scale@.. index@... base@..."_e] = [](sm& st) {};

	// direct addressing
	rm8[ "00 reg@... 101"_e >> imm32_a ] = [](sm& st) {};
	rm8[ "00 reg@... 100"_e >> sib ] = [](sm& st) {};
	rm8[ "00 reg@... rm@..."_e] = [](sm& st) {};
	rm16[ "00 reg@... 101"_e >> imm32_a ] = [](sm& st) {};
	rm16[ "00 reg@... 100"_e >> sib ] = [](sm& st) {};
	rm16[ "00 reg@... rm@..."_e] = [](sm& st) {};
	rm32[ "00 reg@... 101"_e >> imm32_a ] = [](sm& st) {};
	rm32[ "00 reg@... 100"_e >> sib ] = [](sm& st) {};
	rm32[ "00 reg@... rm@..."_e] = [](sm& st) {};
	rm64[ "00 reg@... 101"_e >> imm32_a ] = [](sm& st) {};
	rm64[ "00 reg@... 100"_e >> sib ] = [](sm& st) {};
	rm64[ "00 reg@... rm@..."_e] = [](sm& st) {};

	rm8_0[ "00 000 101"_e >> imm32_a ] = [](sm& st) {};
	rm8_0[ "00 000 100"_e >> sib ] = [](sm& st) {};
	rm8_0[ "00 000 rm@..."_e] = [](sm& st) {};
	rm16_0[ "00 000 101"_e >> imm32_a ] = [](sm& st) {};
	rm16_0[ "00 000 100"_e >> sib ] = [](sm& st) {};
	rm16_0[ "00 000 rm@..."_e] = [](sm& st) {};
	rm32_0[ "00 000 101"_e >> imm32_a ] = [](sm& st) {};
	rm32_0[ "00 000 100"_e >> sib ] = [](sm& st) {};
	rm32_0[ "00 000 rm@..."_e] = [](sm& st) {};
	rm64_0[ "00 000 101"_e >> imm32_a ] = [](sm& st) {};
	rm64_0[ "00 000 100"_e >> sib ] = [](sm& st) {};
	rm64_0[ "00 000 rm@..."_e] = [](sm& st) {};

	rm8_1[ "00 001 rm@..."_e] = [](sm& st) {};
	rm16_1[ "00 001 rm@..."_e] = [](sm& st) {};
	rm32_1[ "00 001 rm@..."_e] = [](sm& st) {};
	rm64_1[ "00 001 rm@..."_e] = [](sm& st) {};

	rm8_2[ "00 010 101"_e >> imm32_a ] = [](sm& st) {};
	rm8_2[ "00 010 100"_e >> sib  ]= [](sm& st) {};
	rm8_2[ "00 010 rm@..."_e] = [](sm& st) {};
	rm16_2[ "00 010 101"_e >> imm32_a ] = [](sm& st) {};
	rm16_2[ "00 010 100"_e >> sib ] = [](sm& st) {};
	rm16_2[ "00 010 rm@..."_e] = [](sm& st) {};
	rm32_2[ "00 010 101"_e >> imm32_a ] = [](sm& st) {};
	rm32_2[ "00 010 100"_e >> sib ] = [](sm& st) {};
	rm32_2[ "00 010 rm@..."_e] = [](sm& st) {};
	rm64_2[ "00 010 101"_e >> imm32_a ] = [](sm& st) {};
	rm64_2[ "00 010 100"_e >> sib ] = [](sm& st) {};
	rm64_2[ "00 010 rm@..."_e] = [](sm& st) {};

	rm8_3[ "00 011 rm@..."_e] = [](sm& st) {};
	rm16_3[ "00 011 rm@..."_e] = [](sm& st) {};
	rm32_3[ "00 011 rm@..."_e] = [](sm& st) {};
	rm64_3[ "00 011 rm@..."_e] = [](sm& st) {};

	rm8_4[ "00 100 rm@..."_e] = [](sm& st) {};
	rm16_4[ "00 100 rm@..."_e] = [](sm& st) {};
	rm32_4[ "00 100 rm@..."_e] = [](sm& st) {};
	rm64_4[ "00 100 rm@..."_e] = [](sm& st) {};

	rm8_5[ "00 101 rm@..."_e] = [](sm& st) {};
	rm16_5[ "00 101 rm@..."_e] = [](sm& st) {};
	rm32_5[ "00 101 rm@..."_e] = [](sm& st) {};
	rm64_5[ "00 101 rm@..."_e] = [](sm& st) {};

	rm8_6[ "00 110 rm@..."_e] = [](sm& st) {};
	rm16_6[ "00 110 rm@..."_e] = [](sm& st) {};
	rm32_6[ "00 110 rm@..."_e] = [](sm& st) {};
	rm64_6[ "00 110 rm@..."_e] = [](sm& st) {};

	rm8_7[ "00 111 rm@..."_e] = [](sm& st) {};
	rm16_7[ "00 111 rm@..."_e] = [](sm& st) {};
	rm32_7[ "00 111 rm@..."_e] = [](sm& st) {};
	rm64_7[ "00 111 rm@..."_e] = [](sm& st) {};

	// indirect addressing
	rm8[ "11 reg@... 101"_e >> imm32_a ] = [](sm& st) {};
	rm8[ "11 reg@... rm@..."_e] = [](sm& st) {};
	rm16[ "11 reg@... 101"_e >> imm32_a ] = [](sm& st) {};
	rm16[ "11 reg@... rm@..."_e] = [](sm& st) {};
	rm32[ "11 reg@... 101"_e >> imm32_a ] = [](sm& st) {};
	rm32[ "11 reg@... rm@..."_e] = [](sm& st) {};
	rm64[ "11 reg@... 101"_e >> imm32_a ] = [](sm& st) {};
	rm64[ "11 reg@... rm@..."_e] = [](sm& st) {};

	rm8_0[ "11 000 rm@..."_e] = [](sm& st) {};
	rm16_0[ "11 000 rm@..."_e] = [](sm& st) {};
	rm32_0[ "11 000 rm@..."_e] = [](sm& st) {};
	rm64_0[ "11 000 rm@..."_e] = [](sm& st) {};

	rm8_1[ "11 001 rm@..."_e] = [](sm& st) {};
	rm16_1[ "11 001 rm@..."_e] = [](sm& st) {};
	rm32_1[ "11 001 rm@..."_e] = [](sm& st) {};
	rm64_1[ "11 001 rm@..."_e] = [](sm& st) {};

	rm8_2[ "11 010 101"_e >> imm32_a ] = [](sm& st) {};
	rm8_2[ "11 010 rm@..."_e] = [](sm& st) {};
	rm16_2[ "11 010 101"_e >> imm32_a ] = [](sm& st) {};
	rm16_2[ "11 010 rm@..."_e] = [](sm& st) {};
	rm32_2[ "11 010 101"_e >> imm32_a ] = [](sm& st) {};
	rm32_2[ "11 010 rm@..."_e] = [](sm& st) {};
	rm64_2[ "11 010 101"_e >> imm32_a ] = [](sm& st) {};
	rm64_2[ "11 010 rm@..."_e] = [](sm& st) {};

	rm8_3[ "11 011 rm@..."_e] = [](sm& st) {};
	rm16_3[ "11 011 rm@..."_e] = [](sm& st) {};
	rm32_3[ "11 011 rm@..."_e] = [](sm& st) {};
	rm64_3[ "11 011 rm@..."_e] = [](sm& st) {};

	rm8_4[ "11 100 rm@..."_e] = [](sm& st) {};
	rm16_4[ "11 100 rm@..."_e] = [](sm& st) {};
	rm32_4[ "11 100 rm@..."_e] = [](sm& st) {};
	rm64_4[ "11 100 rm@..."_e] = [](sm& st) {};

	rm8_5[ "11 101 rm@..."_e] = [](sm& st) {};
	rm16_5[ "11 101 rm@..."_e] = [](sm& st) {};
	rm32_5[ "11 101 rm@..."_e] = [](sm& st) {};
	rm64_5[ "11 101 rm@..."_e] = [](sm& st) {};

	rm8_6[ "11 110 rm@..."_e] = [](sm& st) {};
	rm16_6[ "11 110 rm@..."_e] = [](sm& st) {};
	rm32_6[ "11 110 rm@..."_e] = [](sm& st) {};
	rm64_6[ "11 110 rm@..."_e] = [](sm& st) {};

	rm8_7[ "11 111 rm@..."_e] = [](sm& st) {};
	rm16_7[ "11 111 rm@..."_e] = [](sm& st) {};
	rm32_7[ "11 111 rm@..."_e] = [](sm& st) {};
	rm64_7[ "11 111 rm@..."_e] = [](sm& st) {};

	std::function<void(const std::string&,sm&)> simple = [&](const std::string& m,sm& st)
	{
		std::cerr << m << ": " << std::hex;
		for(auto b: st.tokens)
			std::cerr << (unsigned int)b << " ";
		std::cerr << std::dec << std::endl;
		st.jump(st.address + st.tokens.size());
	};

	// 32 bits only
	main[ *generic_prfx >> 0x37_e							] = std::bind(simple,"AAA",std::placeholders::_1);
	main[ *generic_prfx >> 0xd5_e >> "i@........"_e	] = std::bind(simple,"AAD imm8",std::placeholders::_1);
	main[ *generic_prfx >> 0xd4_e >> "i@........"_e	] = std::bind(simple,"AAM imm8",std::placeholders::_1);
	main[ *generic_prfx >> 0x3f_e							] = std::bind(simple,"AAS",std::placeholders::_1);

	// ADC
	std::function<void(cg&,rvalue,rvalue)> adc = [](cg& m, rvalue a, rvalue b)
	{
		m.assign(to_lvalue(a),(a + b + CF) % 0x100000000ull);
	};

	main[ *generic_prfx >>						0x14 >> imm8_a					] = unary("adc",std::bind(adc,pls::_1,al,pls::_2));

	main[ *generic_prfx >> opsize_prfix	>> 0x15 >> imm16_a				] = unary("adc",std::bind(adc,pls::_1,ax,pls::_2));
	main[ *generic_prfx >>						0x15 >> imm32_a				] = unary("adc",std::bind(adc,pls::_1,eax,pls::_2));
	main[ *generic_prfx >> rexw_prfix	>> 0x15 >> imm32_a				] = unary("adc",std::bind(adc,pls::_1,rax,pls::_2));

	/*main[ *generic_prfx >>						0x80 >> rm8_2 >> imm8_b		] = binary("adc",adc);
	main[ *generic_prfx >> rex_prfix		>> 0x80 >> rm8_2 >> imm8_b		] = binary("adc",adc);

	main[ *generic_prfx >> opsize_prfix	>> 0x81 >> rm16_2 >> imm16_b	] = binary("adc",adc);
	main[ *generic_prfx >>						0x81 >> rm32_2 >> imm32_b	] = binary("adc",adc);
	main[ *generic_prfx >> rexw_prfix	>> 0x81 >> rm64_2 >> imm32_b	] = binary("adc",adc);

	main[ *generic_prfx >> opsize_prfix	>> 0x83 >> rm16_2 >> imm8_b	] = binary("adc",adc);
	main[ *generic_prfx >>						0x83 >> rm32_2 >> imm8_b	] = binary("adc",adc);
	main[ *generic_prfx >> rexw_prfix	>> 0x83 >> rm64_2 >> imm8_b	] = binary("adc",adc);

	main[ *generic_prfx >>						0x10 >> rm8						] = binary("adc",adc);
	main[ *generic_prfx >> rex_prfix		>> 0x10 >> rm8						] = binary("adc",adc);

	main[ *generic_prfx >> opsize_prfix	>> 0x11 >> rm16					] = binary("adc",adc);
	main[ *generic_prfx >>						0x11 >> rm32					] = binary("adc",adc);
	main[ *generic_prfx >> rexw_prfix	>> 0x11 >> rm64					] = binary("adc",adc);

	main[ *generic_prfx >>						0x12 >> rm8						] = binary("adc",adc);
	main[ *generic_prfx >> rex_prfix		>> 0x12 >> rm8						] = binary("adc",adc);

	main[ *generic_prfx >> opsize_prfix	>> 0x13 >> rm16					] = binary("adc",adc);
	main[ *generic_prfx >> 						0x13 >> rm32					] = binary("adc",adc);
	main[ *generic_prfx >> rexw_prfix	>> 0x13 >> rm64					] = binary("adc",adc);*/

	return program::disassemble<amd64_tag>(main,bytes,r,prog);
}

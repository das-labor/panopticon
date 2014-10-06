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
	disassembler<amd64_tag> main, opsize_prfix, rex_prfix,
									imm8_a, imm16_a, imm32_a, imm64_a,
									imm8_b, imm16_b, imm32_b, imm64_b,
									imm8_c, imm16_c, imm32_c, imm64_c;

	std::function<void(const std::string&,sm&)> simple = [&](const std::string& m,sm& st)
	{
		std::cerr << m << std::endl;
		st.jump(st.address + st.tokens.size());
	};

	opsize_prfix | 0x66 = [](sm& st) { /*st.operand_size_prefix = true;*/ };
	rex_prfix | 0x66 = [](sm& st) { /*st.operand_size_prefix = true;*/ };

	// 32 bits only
	main | 0x37 = std::bind(simple,"AAA",std::placeholders::_1);
	main | 0xd5 | "i@........" = std::bind(simple,"AAD imm8",std::placeholders::_1);
	main | 0xd4 | "i@........" = std::bind(simple,"AAM imm8",std::placeholders::_1);
	main | 0x3f = std::bind(simple,"AAS",std::placeholders::_1);

	// ADC
	main						| 0x14 | imm8_a	= std::bind(simple,"ADC AL, imm8",pls::_1);
	main | opsize_prfix	| 0x15 | imm16_a	= std::bind(simple,"ADC AX, imm16",pls::_1);
	main						| 0x15 | imm32_a	= std::bind(simple,"ADC EAX, imm32",pls::_1);
	main | rex_prfix		| 0x15 | imm64_a	= std::bind(simple,"ADC RAX, imm64",pls::_1);

	return program::disassemble<amd64_tag>(main,bytes,r,prog);
}

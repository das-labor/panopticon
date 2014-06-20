#include <panopticon/disassembler.hh>

#pragma once

struct test_tag {};
extern unsigned int ununsed;
extern std::vector<std::string> regs;

namespace po
{
	template<>
	struct architecture_traits<test_tag>
	{
		using token_type = unsigned char;
	};

	template<>
	lvalue temporary(test_tag);

	template<>
	const std::vector<std::string> &registers(test_tag);

	template<>
	uint8_t width(std::string n, test_tag);
}

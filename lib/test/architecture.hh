#ifndef TEST_ARCHITECTURE_HH
#define TEST_ARCHITECTURE_HH

#include <panopticon/architecture.hh>

struct test_tag {};
unsigned int ununsed = 0;
std::vector<std::string> regs({"a","b","c","d"});

namespace po
{
	template<>
	struct architecture_traits<test_tag>
	{
		typedef unsigned char token_type;
	};

	template<>
	lvalue temporary(test_tag)
	{
		return variable("t" + std::to_string(ununsed++),16);
	}

	template<>
	const std::vector<std::string> &registers(test_tag)
	{
		return regs;
	}

	template<>
	uint8_t width(std::string n, test_tag)
	{
		return 8;
	}
}

#endif

#include "architecture.hh"

unsigned int ununsed = 0;
std::vector<std::string> regs({"a","b","c","d"});

template<>
po::lvalue po::temporary(test_tag)
{
	return po::variable("t" + std::to_string(ununsed++),16);
}

template<>
const std::vector<std::string>& po::registers(test_tag)
{
	return regs;
}

template<>
uint8_t po::width(std::string n, test_tag)
{
	return 8;
}

template<>
po::lvalue po::temporary(wtest_tag)
{
	return po::variable("t" + std::to_string(ununsed++),16);
}

template<>
const std::vector<std::string>& po::registers(wtest_tag)
{
	return regs;
}

template<>
uint8_t po::width(std::string n, wtest_tag)
{
	return 8;
}

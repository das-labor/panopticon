#ifndef ARCHITECTURE_HH
#define ARCHITECTURE_HH

#include <string>
#include <vector>

#include <mnemonic.hh>

namespace po
{
	template<typename T>
	struct architecture_traits
	{
		typedef void token_type;
	};

	template<typename T>
	lvalue temporary(T);

	template<typename T>
	const std::vector<std::string> &registers(T);

	template<typename T>
	uint8_t width(::std::string n, T);
}

#endif

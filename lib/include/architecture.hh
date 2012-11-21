#ifndef ARCHITECTURE_HH
#define ARCHITECTURE_HH

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
}

#endif

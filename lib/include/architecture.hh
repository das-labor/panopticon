#ifndef ARCHITECTURE_HH
#define ARCHITECTURE_HH

#include "mnemonic.hh"

template<typename T>
struct architecture_traits
{
	typedef void token_type;
};

template<typename T>
var_ptr temporary(T);

#endif

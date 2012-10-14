#ifndef ARCHITECTURE_HH
#define ARCHITECTURE_HH

#include "mnemonic.hh"

template<typename T>
struct architecture_traits
{
	typedef void token_type;
};

template<typename T>
bool valid(T,const name &);

template<typename T>
unsigned int width(T,const name &);

template<typename T>
name unused(T);

#endif

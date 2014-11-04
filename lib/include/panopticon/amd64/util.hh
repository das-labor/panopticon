#include <panopticon/value.hh>

#include <panopticon/amd64/amd64.hh>

#pragma once

namespace po
{
	namespace amd64
	{
		memory byte(rvalue);
		memory word(rvalue);
		memory dword(rvalue);
		memory qword(rvalue);
		memory byte(unsigned int);
		memory word(unsigned int);
		memory dword(unsigned int);
		memory qword(unsigned int);

		sem_action unary(std::string const&,std::function<void(cg&,rvalue)>);
		sem_action binary(std::string const&,std::function<void(cg&,rvalue,rvalue)>);
		sem_action branch(std::string const&, rvalue, bool);
	}
}

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

		std::pair<rvalue,rvalue> decode_rm(sm const&);
		std::pair<rvalue,rvalue> decode_mr(sm const&);
		std::pair<rvalue,rvalue> decode_mi(sm const&);
		std::pair<rvalue,rvalue> decode_i(sm const&);
		std::tuple<rvalue,rvalue,rvalue> decode_rvm(sm const&);
		std::tuple<rvalue,rvalue,rvalue> decode_rmv(sm const&);
		std::tuple<rvalue,rvalue,rvalue,rvalue> decode_rvmi(sm const&);
		std::tuple<rvalue,rvalue,rvalue> decode_rmi(sm const&);

		variable decode_reg8(unsigned int,bool);
		variable decode_reg16(unsigned int);
		variable decode_reg32(unsigned int);
		variable decode_reg64(unsigned int);

		lvalue decode_modrm(unsigned int,unsigned int,boost::optional<unsigned int>,boost::optional<unsigned int>,amd64_state::OperandSize,amd64_state::AddressSize);
		variable select_reg(amd64_state::OperandSize,unsigned int);
		memory select_mem(amd64_state::AddressSize,rvalue);

		sem_action nonary(std::string const&,std::function<void(cg&)>);
		sem_action unary(std::string const&,std::function<rvalue(sm const&)>,std::function<void(cg&,rvalue)>);
		sem_action binary(std::string const&,std::function<std::pair<rvalue,rvalue>(sm const&)>,std::function<void(cg&,rvalue,rvalue)>);
		sem_action branch(std::string const&, rvalue, bool);
	}
}

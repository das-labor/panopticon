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
		memory byte(uint64_t);
		memory word(uint64_t);
		memory dword(uint64_t);
		memory qword(uint64_t);

		std::pair<rvalue,rvalue> decode_rm(sm const&,cg&);
		std::pair<rvalue,rvalue> decode_mr(sm const&,cg&);
		std::pair<rvalue,rvalue> decode_mi(sm const&,cg&);
		std::pair<rvalue,rvalue> decode_i(sm const&,cg&);
		std::tuple<rvalue,rvalue,rvalue> decode_rvm(sm const&,cg&);
		std::tuple<rvalue,rvalue,rvalue> decode_rmv(sm const&,cg&);
		std::tuple<rvalue,rvalue,rvalue,rvalue> decode_rvmi(sm const&,cg&);
		std::tuple<rvalue,rvalue,rvalue> decode_rmi(sm const&,cg&);

		variable decode_reg8(unsigned int r_reg,bool rex);
		variable decode_reg16(unsigned int r_reg);
		variable decode_reg32(unsigned int r_reg);
		variable decode_reg64(unsigned int r_reg);

		lvalue decode_modrm(
			unsigned int mod,
			unsigned int b_rm,	// B.R/M
			boost::optional<uint64_t> disp,
			boost::optional<std::tuple<uint64_t,uint64_t,uint64_t>> sib, // scale, X.index, B.base
			amd64_state::OperandSize os,
			amd64_state::AddressSize as,
			cg& c);

		memory decode_sib(
			unsigned int mod,
			unsigned int scale,
			unsigned int x_index,
			unsigned int b_base,
			boost::optional<uint64_t> disp,
			amd64_state::AddressSize,cg&);

		variable select_reg(amd64_state::OperandSize,unsigned int);
		memory select_mem(amd64_state::AddressSize,rvalue);

		sem_action unary(std::string const&,std::function<rvalue(sm const&,cg&)>,std::function<void(cg&,rvalue)>);
		sem_action binary(std::string const&,std::function<std::pair<rvalue,rvalue>(sm const&,cg&)>,std::function<void(cg&,rvalue,rvalue)>);
		sem_action branch(std::string const&, rvalue, bool);
	}
}

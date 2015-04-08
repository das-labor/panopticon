/*
 * This file is part of Panopticon (http://panopticon.re).
 * Copyright (C) 2014 Kai Michaelis
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

#include <panopticon/value.hh>
#include <panopticon/amd64/traits.hh>

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

		rvalue decode_m(sm const&,cg&);
		rvalue decode_d(sm const&,cg&);
		rvalue decode_imm(sm const&,cg&);
		rvalue decode_moffs(sm const&,cg&);
		rvalue decode_rm1(sm const&,cg&);
		std::pair<rvalue,rvalue> decode_i(sm const&,cg&);
		std::pair<rvalue,rvalue> decode_rm(sm const&,cg&);
		std::pair<rvalue,rvalue> decode_fd(sm const&,cg&);
		std::pair<rvalue,rvalue> decode_td(sm const&,cg&);
		std::pair<rvalue,rvalue> decode_sregm(sm const&,cg&);
		std::pair<rvalue,rvalue> decode_msreg(sm const&,cg&);
		std::pair<rvalue,rvalue> decode_dbgrm(sm const&,cg&);
		std::pair<rvalue,rvalue> decode_rmdbg(sm const&,cg&);
		std::pair<rvalue,rvalue> decode_ctrlrm(sm const&,cg&);
		std::pair<rvalue,rvalue> decode_rmctrl(sm const&,cg&);
		std::pair<rvalue,rvalue> decode_mr(sm const&,cg&);
		std::pair<rvalue,rvalue> decode_mi(sm const&,cg&);
		std::pair<rvalue,rvalue> decode_m1(sm const&,cg&);
		std::pair<rvalue,rvalue> decode_mc(sm const&,cg&);
		std::pair<rvalue,rvalue> decode_ii(sm const&,cg&);
		std::tuple<rvalue,rvalue,rvalue> decode_rvm(sm const&,cg&);
		std::tuple<rvalue,rvalue,rvalue> decode_rmv(sm const&,cg&);
		std::tuple<rvalue,rvalue,rvalue> decode_rmi(sm const&,cg&);
		std::tuple<rvalue,rvalue,rvalue> decode_mri(sm const&,cg&);
		std::tuple<rvalue,rvalue,rvalue,rvalue> decode_rvmi(sm const&,cg&);

		variable decode_reg8(unsigned int r_reg,bool rex);
		variable decode_reg16(unsigned int r_reg);
		variable decode_reg32(unsigned int r_reg);
		variable decode_reg64(unsigned int r_reg);

		template<unsigned int I>
		rvalue reg(sm const& st,cg&);
		template<unsigned int I>
		rvalue regd(sm const& st,cg&);
		template<unsigned int I>
		rvalue regb(sm const& st,cg&);

#define reg_a reg<0>
#define reg_c reg<1>
#define reg_d reg<2>
#define reg_b reg<3>
#define reg_sp reg<4>
#define reg_bp reg<5>
#define reg_si reg<6>
#define reg_di reg<7>
#define regd_a regd<0>
#define regd_c regd<1>
#define regd_d regd<2>
#define regd_b regd<3>
#define regd_sp regd<4>
#define regd_bp regd<5>
#define regd_si regd<6>
#define regd_di regd<7>
#define regb_a regb<0>
#define regb_c regb<1>
#define regb_d regb<2>
#define regb_b regb<3>
#define regb_sp regb<4>
#define regb_bp regb<5>
#define regb_si regb<6>
#define regb_di regb<7>

		lvalue decode_modrm(
			unsigned int mod,
			unsigned int b_rm,	// B.R/M
			boost::optional<constant> disp,
			boost::optional<std::tuple<unsigned int,unsigned int,unsigned int>> sib, // scale, X.index, B.base
			amd64_state::OperandSize os,
			amd64_state::AddressSize as,
			amd64_state::Mode mode,
			bool rex,
			cg& c);

		memory decode_sib(
			unsigned int mod,
			unsigned int scale,
			unsigned int x_index,
			unsigned int b_base,
			boost::optional<constant> disp,
			amd64_state::OperandSize,cg&);

		variable select_reg(amd64_state::OperandSize,unsigned int,bool);
		memory select_mem(amd64_state::OperandSize,rvalue);

		sem_action nonary(std::string const&,std::function<void(cg&)>);
		sem_action unary(std::string const&,std::function<rvalue(sm const&,cg&)>,std::function<void(cg&,rvalue)>);
		sem_action unary(std::string const& op, rvalue arg, std::function<void(cg&,rvalue)> func);

		sem_action binary(std::string const&,std::function<std::pair<rvalue,rvalue>(sm const&,cg&)>,std::function<void(cg&,rvalue,rvalue)>);
		sem_action binary(std::string const&, rvalue, std::function<rvalue(sm const&,cg&)>,std::function<void(cg&,rvalue,rvalue)>);
		sem_action binary(std::string const&, std::function<rvalue(sm const&,cg&)>,rvalue,std::function<void(cg&,rvalue,rvalue)>);
		sem_action binary(std::string const&, rvalue, rvalue,std::function<void(cg&,rvalue,rvalue)>);
		sem_action binary(std::string const&, std::function<rvalue(sm const&,cg&)>, std::function<rvalue(sm const&,cg&)>,std::function<void(cg&,rvalue,rvalue)>);

		sem_action trinary(std::string const&,std::function<std::tuple<rvalue,rvalue,rvalue>(sm const&,cg&)>,std::function<void(cg&,rvalue,rvalue,rvalue)>);
		sem_action trinary(std::string const&,std::function<std::pair<rvalue,rvalue>(sm const&,cg&)>,rvalue,std::function<void(cg&,rvalue,rvalue,rvalue)>);
		sem_action branch(std::string const&, rvalue, bool);
	}
}

template<unsigned int I>
po::rvalue po::amd64::reg(sm const& st,cg&)
{
	unsigned int reg = (st.capture_groups.count("b") && st.capture_groups.at("b") == 1 ? 8 : 0) + I;
	return select_reg(st.state.op_sz,reg,st.state.rex);
}

template<unsigned int I>
po::rvalue po::amd64::regd(sm const& st,cg&)
{
	amd64_state::OperandSize opsz = st.state.op_sz;
	unsigned int reg = (st.capture_groups.count("b") && st.capture_groups.at("b") == 1 ? 8 : 0) + I;

	if(st.state.mode == amd64_state::LongMode && st.state.op_sz == amd64_state::OpSz_32)
		opsz = amd64_state::OpSz_64;

	return select_reg(opsz,reg,st.state.rex);
}

template<unsigned int I>
po::rvalue po::amd64::regb(sm const& st,cg&)
{
	return select_reg(amd64_state::OpSz_8,(st.capture_groups.count("b") && st.capture_groups.at("b") == 1 ? 8 : 0) + I,st.state.rex);
}

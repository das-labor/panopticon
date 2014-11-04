#include <panopticon/program.hh>

#pragma once

namespace po
{
	// architecture_traits
	struct amd64_tag {};

	struct amd64_state
	{
		enum AddressSize
		{
			AddrSz_64,
			AddrSz_32,
			AddrSz_16,
		};

		enum OperandSize
		{
			OpSz_64,
			OpSz_32,
			OpSz_16,
			OpSz_8,
		};

		AddressSize addr_sz;
		OperandSize op_sz;

		boost::optional<rvalue> operand_a;
		boost::optional<rvalue> operand_b;
		boost::optional<rvalue> operand_c;
	};

	template<>
	struct architecture_traits<amd64_tag>
	{
		using token_type = uint8_t;
		using state_type = amd64_state;
	};

	template<>
	lvalue temporary(amd64_tag);

	template<>
	const std::vector<std::string> &registers(amd64_tag);

	template<>
	uint8_t width(std::string n, amd64_tag);

	namespace amd64
	{
		typedef sem_state<amd64_tag> sm;
		typedef std::function<void(sm &)> sem_action;
		typedef code_generator<amd64_tag> cg;

		boost::optional<prog_loc> disassemble(boost::optional<prog_loc>, po::slab, const po::ref&);
	}
}

#include <panopticon/program.hh>

#pragma once

namespace po
{
	// architecture_traits
	struct amd64_tag
	{
		enum
		{
			AMD64_PROTECTED_MODE,
			AMD64_REAL_ADDRESS_MODE,
			AMD64_COMPAT_MODE,
			AMD64_64_MODE
		} mode;
	};

	template<>
	struct architecture_traits<amd64_tag>
	{
		typedef uint8_t token_type;
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

		prog_loc disassemble(boost::optional<prog_loc>, po::slab, const po::ref&);
	}
}

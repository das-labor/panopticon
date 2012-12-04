#ifndef AVR_HH
#define AVR_HH

#include <flowgraph.hh>
#include <architecture.hh>

namespace po 
{
	// architecture_traits
	struct avr_tag {};

	template<>
	struct architecture_traits<avr_tag>
	{
		typedef uint16_t token_type;
	};

	template<>
	lvalue temporary(avr_tag);

	namespace avr
	{
		typedef sem_state<avr_tag> sm;
		typedef ::std::function<void(sm &)> sem_action;
		typedef code_generator<avr_tag> cg;

		flow_ptr disassemble(::std::vector<uint16_t> &bytes,addr_t entry, flow_ptr flow = 0, std::function<void(proc_ptr,unsigned int)> signal = std::function<void(proc_ptr,unsigned int)>());
	}
}

#endif

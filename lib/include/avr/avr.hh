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

	template<>
	const std::vector<std::string> &registers(avr_tag);

	template<>
	uint8_t width(std::string n, avr_tag);

	namespace avr
	{
		typedef sem_state<avr_tag> sm;
		typedef ::std::function<void(sm &)> sem_action;
		typedef code_generator<avr_tag> cg;

		flow_ptr disassemble(::std::vector<uint16_t> &bytes,addr_t entry, flow_ptr flow = 0, disassemble_cb signal = disassemble_cb());
	}
}

#endif

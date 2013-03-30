#ifndef INFLATE_HH
#define INFLATE_HH

#include <sstream>

namespace po
{
	class odotstream : public std::ostringstream
	{
	public:
		odotstream(void);

		bool calls;
		bool body;
		bool subgraph;
	};
	
	odotstream &operator<<(odotstream &os, odotstream &(*func)(odotstream &os));

	/*odotstream &operator<<(odotstream &os, const procedure &p);
	odotstream &operator<<(odotstream &os, const mnemonic &m);
	odotstream &operator<<(odotstream &os, const instr &i);
	odotstream &operator<<(odotstream &os, rvalue v);*/

	odotstream &calls(odotstream &os);
	odotstream &nocalls(odotstream &os);
	odotstream &body(odotstream &os);
	odotstream &nobody(odotstream &os);
	odotstream &subgraph(odotstream &os);
	odotstream &nosubgraph(odotstream &os);

	//std::string turtle(flow_ptr fg);
	//std::string graphviz(flow_ptr fg);

	template<typename T>
	std::string unique_name(const T &t)
	{
		return std::string("generic_") + std::to_string((uintptr_t)&t);
	}

	template<typename T>
	odotstream &operator<<(odotstream &os, const T &t)
	{
		static_cast<std::ostringstream &>(os) << t;
		return os;
	}
}

#endif

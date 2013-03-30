#ifndef INFLATE_HH
#define INFLATE_HH

#include <sstream>

#include <flowgraph.hh>

namespace po
{
	class odotstream : public std::ostringstream
	{
	public:
		odotstream(void);
	};

	odotstream &operator<<(odotstream &os, const flowgraph &f);
	odotstream &operator<<(odotstream &os, const procedure &p);
	odotstream &operator<<(odotstream &os, const mnemonic &m);
	odotstream &operator<<(odotstream &os, const instr &i);
	odotstream &operator<<(odotstream &os, rvalue v);

	std::string turtle(flow_ptr fg);
	std::string graphviz(flow_ptr fg);
}

#endif

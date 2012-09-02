#ifndef FLOWGRAPH_HH
#define FLOWGRAPH_HH

#include <memory>
#include <set>

#include "procedure.hh"
#include "decoder.hh"

using namespace std;

typedef shared_ptr<struct flowgraph> flow_ptr;

struct flowgraph
{
	set<proc_ptr> procedures;
};

template<typename token,typename tokiter>
flow_ptr disassemble(const decoder<token,tokiter> &main, vector<token> tokens, addr_t offset = 0, bool cf_sensitive = true)
{
	flow_ptr ret(new flowgraph());

	ret->procedures.insert(disassemble_procedure(main,tokens,offset,cf_sensitive));
	return ret;
}

string graphviz(flow_ptr fg);

#endif

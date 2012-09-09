#ifndef ABSINTERP_HH
#define ABSINTERP_HH

#include <functional>
#include <map>
#include <algorithm>
#include <numeric>
#include <memory>

using namespace std;

#include "basic_block.hh"
#include "mnemonic.hh"
#include "procedure.hh"

// per basic block
template<typename D,typename L>
map<bblock_ptr,L> *abstract_interpretation(proc_ptr proc)
{
	// MOP
	bool modified = true;
	map<bblock_ptr,L> *last_states = new map<bblock_ptr,L>();
	D tag;
	procedure::iterator i,iend;

	// initialize all abstract states to bottom()
	tie(i,iend) = proc->rev_postorder();
	for_each(i,iend,[&](bblock_ptr bb) { last_states->insert(make_pair(bb,bottom(tag))); });

	while(modified)
	{
		map<bblock_ptr,L> *states = new map<bblock_ptr,L>();

		modified = false;
		tie(i,iend) = proc->rev_postorder();

		for_each(i,iend,[&](bblock_ptr bb)
		{
			basic_block::pred_iterator k,kend;
			instr_iterator j,jend;

			// supremum of all predecessor states
			tie(k,kend) = bb->predecessors();
			L lat = accumulate(k,kend,bottom(tag),[&](const L l, bblock_ptr pred) { return supremum(tag,l,last_states->at(pred)); });
			
			// accumulate semantics of the basic block
			tie(j,jend) = bb->instructions();
			lat = accumulate(j,jend,lat,[&](const L l, instr_cptr i) { return abstraction(tag,l,i); });

			states->insert(make_pair(bb,lat));
		});

		modified = !all_of(states->begin(),states->end(),[&](const pair<bblock_ptr,L> p) { return equal(tag,last_states->at(p.first),p.second); });
		delete last_states;
		last_states = states;
	}

	return last_states;
}

// taint domain
struct taint_domain {};
typedef shared_ptr<map<name,set<name>>> taint_lattice;
taint_lattice bottom(taint_domain);
bool equal(taint_domain,const taint_lattice a, const taint_lattice b);
taint_lattice supremum(taint_domain,const taint_lattice a, const taint_lattice b);
taint_lattice abstraction(taint_domain,const taint_lattice a, instr_cptr i);
ostream& operator<<(ostream &os, const taint_lattice l);
#endif

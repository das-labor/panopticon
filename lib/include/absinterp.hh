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
#include "persistent_map.hh"

// per basic block
template<typename D,typename L>
map<bblock_ptr,L> *abstract_interpretation(proc_ptr proc)
{
	// Worklist algo
	map<bblock_ptr,L> *states = new map<bblock_ptr,L>();
	D tag;
	set<bblock_ptr> worklist;

	// initialize all abstract states to bottom(), fill worklist
	copy(proc->basic_blocks.begin(),proc->basic_blocks.end(),inserter(worklist,worklist.begin()));
	for_each(proc->basic_blocks.begin(),proc->basic_blocks.end(),[&](bblock_ptr bb) { states->insert(make_pair(bb,bottom(tag))); });

	while(!worklist.empty())
	{
		set<bblock_ptr>::iterator i = worklist.begin();
		bblock_ptr bb = *i;

		worklist.erase(i);

		basic_block::pred_iterator k,kend;
		basic_block::succ_iterator l,lend;
		size_t sz = bb->instructions().size(), pos = 0;
		const instr_ptr *j = bb->instructions().data();

		// supremum of all predecessor states
		tie(k,kend) = bb->predecessors();
		tie(l,lend) = bb->successors();
		L lat = accumulate(k,kend,bottom(tag),[&](const L l, bblock_ptr pred) { return supremum(tag,l,states->at(pred)); });
			
		// accumulate semantics of the basic block
		while(pos < sz)
			lat = abstraction(tag,lat,j[pos++]);
		
		if(!equal(tag,lat,states->at(bb)))
			copy(l,lend,inserter(worklist,worklist.begin()));
		
		states->erase(bb);
		states->insert(make_pair(bb,lat));
	
		cout << worklist.size() << endl;
	}

	return states;
}

// taint domain
struct taint_domain {};
typedef shared_ptr<persistent_map<name,set<name>>> taint_lattice;
taint_lattice bottom(taint_domain);
bool equal(taint_domain,const taint_lattice a, const taint_lattice b);
taint_lattice supremum(taint_domain,const taint_lattice a, const taint_lattice b);
taint_lattice abstraction(taint_domain,const taint_lattice a, instr_cptr i);
ostream& operator<<(ostream &os, const taint_lattice l);
ostream& operator<<(ostream &os, const set<name> &e);

// cprop domain
struct cprop_domain {};
struct cprop_element
{
	enum Type
	{
		Bottom = 0,
		NonConst = 2,
		Const = 1
	};

	cprop_element(void) : type(Bottom), value(0) {};
	cprop_element(Type t) : type(t), value(0) { assert((int)t >= 0 && (int)t <= 2); };
	cprop_element(unsigned int v) : type(Const), value(v) {};
	
	Type type;
	unsigned int value;
};
bool operator==(const cprop_element &a, const cprop_element &b);
bool operator!=(const cprop_element &a, const cprop_element &b);
typedef shared_ptr<persistent_map<name,cprop_element>> cprop_lattice;
cprop_lattice bottom(cprop_domain);
bool equal(cprop_domain,const cprop_lattice a, const cprop_lattice b);
cprop_lattice supremum(cprop_domain,const cprop_lattice a, const cprop_lattice b);
cprop_element supremum(const cprop_element a, const cprop_element b);
cprop_lattice abstraction(cprop_domain,const cprop_lattice a, instr_cptr i);
ostream& operator<<(ostream &os, const cprop_lattice l);
ostream& operator<<(ostream &os, const cprop_element &e);

#endif

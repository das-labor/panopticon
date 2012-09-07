#ifndef DFLOW_HH
#define DFLOW_HH

#include <memory>
#include <set>
#include <map>

using namespace std;

typedef shared_ptr<struct dom> dom_ptr;
typedef shared_ptr<struct live> live_ptr;

#include "procedure.hh"
#include "basic_block.hh"

struct dom
{
	dtree_ptr root;
	map<bblock_ptr,dtree_ptr> tree;
};

struct live
{
	set<name> names;									// global (procedure-wide) names
	map<name,set<bblock_ptr>> usage;	// maps names to blocks that use them

	map<bblock_ptr,set<name>> uevar;		// up exposed variables
	map<bblock_ptr,set<name>> varkill;	// overwritten vars
	map<bblock_ptr,set<name>> liveout;
};

set<name> set_difference(set<name> a, set<name> b);
set<name> set_union(set<name> a, set<name> b);
set<name> set_intersection(set<name> a, set<name> b);

dom_ptr dominance_tree(proc_ptr proc);
void ssa(proc_ptr proc, dom_ptr dominance, live_ptr live);
live_ptr liveness(proc_ptr proc);

#endif

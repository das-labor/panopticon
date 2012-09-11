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

template<typename T>
set<T> set_difference(const set<T> &a, const set<T> &b)
{
	set<T> ret;
	set_difference(a.begin(),a.end(),b.begin(),b.end(),inserter(ret,ret.begin()));
	return ret;
}

template<typename T>
set<T> set_union(const set<T> &a, const set<T> &b)
{
	set<T> ret;
	//set_union(a.begin(),a.end(),b.begin(),b.end(),inserter(ret,ret.begin()));
	merge(a.begin(),a.end(),b.begin(),b.end(),inserter(ret,ret.begin()));
	return ret;
}

template<typename T>
set<T> sset_union(const set<T> &a, const set<T> &b)
{
	set<T> ret;
	set_union(a.begin(),a.end(),b.begin(),b.end(),inserter(ret,ret.begin()));
	return ret;
}


template<typename T>
set<T> set_intersection(const set<T> &a, const set<T> &b)
{
	set<T> ret;
	set_intersection(a.begin(),a.end(),b.begin(),b.end(),inserter(ret,ret.begin()));
	return ret;
}

dom_ptr dominance_tree(proc_ptr proc);
void ssa(proc_ptr proc, dom_ptr dominance, live_ptr live);
live_ptr liveness(proc_ptr proc);

#endif

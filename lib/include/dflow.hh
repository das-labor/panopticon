#ifndef DFLOW_HH
#define DFLOW_HH

#include <memory>
#include <set>
#include <map>
#include <algorithm>

namespace po
{
	typedef ::std::shared_ptr<struct dom> dom_ptr;
	typedef ::std::shared_ptr<struct live> live_ptr;
}

#include <procedure.hh>
#include <basic_block.hh>

namespace po
{
	struct dom
	{
		dtree_ptr root;
		::std::map<bblock_cptr,dtree_ptr> tree;
	};

	struct live
	{
		::std::set< ::std::string> names;									// global (procedure-wide) names
		::std::map< ::std::string,::std::set<bblock_cptr>> usage;	// maps names to blocks that use them

		::std::map<bblock_cptr,::std::set< ::std::string>> uevar;		// up exposed variables
		::std::map<bblock_cptr,::std::set< ::std::string>> varkill;	// overwritten vars
		::std::map<bblock_cptr,::std::set< ::std::string>> liveout;
	};

	template<typename T>
	::std::set<T> set_difference(const ::std::set<T> &a, const ::std::set<T> &b)
	{
		::std::set<T> ret;
		::std::set_difference(a.begin(),a.end(),b.begin(),b.end(),::std::inserter(ret,ret.begin()));
		return ret;
	}

	template<typename T>
	::std::set<T> set_union(const ::std::set<T> &a, const ::std::set<T> &b)
	{
		::std::set<T> ret;
		//set_union(a.begin(),a.end(),b.begin(),b.end(),inserter(ret,ret.begin()));
		::std::merge(a.begin(),a.end(),b.begin(),b.end(),::std::inserter(ret,ret.begin()));
		return ret;
	}

	template<typename T>
	::std::set<T> sset_union(const ::std::set<T> &a, const ::std::set<T> &b)
	{
		::std::set<T> ret;
		::std::set_union(a.begin(),a.end(),b.begin(),b.end(),::std::inserter(ret,ret.begin()));
		return ret;
	}

	template<typename T>
	::std::set<T> set_intersection(const ::std::set<T> &a, const ::std::set<T> &b)
	{
		::std::set<T> ret;
		::std::set_intersection(a.begin(),a.end(),b.begin(),b.end(),::std::inserter(ret,ret.begin()));
		return ret;
	}

	dom_ptr dominance_tree(proc_ptr proc);
	void ssa(proc_ptr proc, dom_ptr dominance, live_ptr live);
	live_ptr liveness(proc_cptr proc);
}

#endif

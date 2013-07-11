#ifndef DFLOW_HH
#define DFLOW_HH

#include <memory>
#include <set>
#include <map>
#include <algorithm>

namespace po
{
	typedef std::shared_ptr<struct dom> dom_ptr;
	typedef std::shared_ptr<struct live> live_ptr;
}

#include <procedure.hh>
#include <basic_block.hh>

namespace po
{
	struct dom
	{
		dom(void) : root(0), tree() {}
		dtree_ptr root;
		std::map<bblock_cwptr,dtree_ptr> tree;
	};

	struct name
	{
		name(void) : base(""), width(0) {};
		name(std::string n, uint8_t w) : base(n), width(w) {};
		name(variable v) : base(v.name()), width(v.width()) {};

		bool operator==(const name &n) const { return base == n.base && width == n.width; };
		bool operator!=(const name &n) const { return !(*this == n); };
		bool operator<(const name &n) const { return base < n.base; };
		operator std::string(void) const { return base; };

		std::string base;
		uint8_t width;
	};

	struct live
	{
		live(void) : names(), usage(), uevar(), varkill(), liveout() {};
		std::set<name> names;	// global (procedure-wide) names w/ width
		std::map<name,std::set<bblock_cwptr>> usage;	// maps names to blocks that use them

		std::map<bblock_cwptr,std::set<name>> uevar;		// up exposed variables
		std::map<bblock_cwptr,std::set<name>> varkill;	// overwritten vars
		std::map<bblock_cwptr,std::set<name>> liveout;
	};

	template<typename T>
	std::set<T> set_difference(const std::set<T> &a, const std::set<T> &b)
	{
		std::set<T> ret;
		std::set_difference(a.begin(),a.end(),b.begin(),b.end(),std::inserter(ret,ret.begin()));
		return ret;
	}

	template<typename T>
	std::set<T> set_union(const std::set<T> &a, const std::set<T> &b)
	{
		std::set<T> ret;
		//set_union(a.begin(),a.end(),b.begin(),b.end(),inserter(ret,ret.begin()));
		std::merge(a.begin(),a.end(),b.begin(),b.end(),std::inserter(ret,ret.begin()));
		return ret;
	}

	template<typename T>
	std::set<T> sset_union(const std::set<T> &a, const std::set<T> &b)
	{
		std::set<T> ret;
		std::set_union(a.begin(),a.end(),b.begin(),b.end(),std::inserter(ret,ret.begin()));
		return ret;
	}

	template<typename T>
	std::set<T> set_intersection(const std::set<T> &a, const std::set<T> &b)
	{
		std::set<T> ret;
		std::set_intersection(a.begin(),a.end(),b.begin(),b.end(),std::inserter(ret,ret.begin()));
		return ret;
	}

	dom_ptr dominance_tree(proc_ptr proc);
	void ssa(proc_ptr proc, dom_ptr dominance, live_ptr live);
	live_ptr liveness(proc_cptr proc);
}

#endif

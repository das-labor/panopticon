#ifndef INTERPRETER_HH
#define INTERPRETER_HH

#include <vector>
#include <cmath>
#include <memory>
#include <map>
#include <set>

#include <mnemonic.hh>
#include <basic_block.hh>
#include <procedure.hh>

namespace po
{
	template <typename T>
	struct domain_traits {};

	template<typename T>
	::std::shared_ptr< ::std::map<rvalue,typename domain_traits<T>::lattice>> interpret(const proc_ptr proc, T tag = T())
	{
		typedef typename domain_traits<T>::lattice L;
		::std::shared_ptr< ::std::map<rvalue,typename domain_traits<T>::lattice>>	ret(new ::std::map<rvalue,typename domain_traits<T>::lattice>());
		::std::set<bblock_ptr> worklist(proc->basic_blocks);

		while(!worklist.empty())
		{
			bblock_ptr bb = *worklist.begin();
			bool modified = false;

			//::std::cout << "interpret " << bb->area() << ::std::endl;
			worklist.erase(worklist.begin());
			execute(bb,[&](const lvalue &left, instr::Function fn, const ::std::vector<rvalue> &right)
			{
				::std::vector<L> arguments;
				L res = ret->count(left) ? ret->at(left) : L();

				for(const rvalue &r: right)
					if(ret->count(r))
						arguments.emplace_back(ret->at(r));
					else
						arguments.emplace_back(L());

				//::std::cout << left << " = " << res << " -> ";

				if(fn == instr::Phi)
					res = accumulate(arguments.begin(),arguments.end(),res,[&](const L &acc, const L &x) { return supremum(acc,x,tag); });
				else
					res = supremum(execute(left,fn,right,arguments,tag),res,tag);

				//::std::cout << res << ::std::endl;

				
				if(!ret->count(left) || !(ret->at(left) == res))
				{
					//::std::cout << left << " changed!" << ::std::endl;
					modified = true;
				}

				if(ret->count(left))
					ret->erase(left);
				ret->insert(::std::make_pair(left,res));
			});

			if(modified)
			{
				auto p = bb->successors();
				copy(p.first,p.second,::std::inserter(worklist,worklist.end()));
				//::std::cout << "insert " << distance(p.first,p.second) << " successors" << ::std::endl;
			}

			::std::cout << worklist.size() << ::std::endl;
		}

		return ret;
	}

	template<typename I>
	struct concrete_interp {};

	template<typename I>
	struct domain_traits<concrete_interp<I>>
	{
		typedef I lattice;
	};

	template<typename I>
	I execute(const lvalue &left, instr::Function fn, const ::std::vector<rvalue> &concrete, const ::std::vector<I> &args,concrete_interp<I>)
	{
		switch(fn)
		{
		// Bitwise Not
		case instr::Not: return ~args[0];
		
		// Bitwise And
		case instr::And:	return args[0] & args[1];
		
		// Bitwise Or
		case instr::Or:	return args[0] | args[1];
		
		// Bitwize Xor
		case instr::Xor:	return args[0] ^ args[1];
		
		// Assign Intermediate
		case instr::Assign:	return args[0];
		
		// Unsigned right shift	*
		case instr::UShr:	return args[0] >> args[1];
		
		// Unsigned left shift *
		case instr::UShl:	return args[0] << args[1];
		
		// Slice
		case instr::Slice: return (args[0] >> args[1]) % (I)::std::pow(2,args[2]+1);
		
		// Concatenation
		//case instr::Concat: return args[0] << (sizeof(I) * 4) | args[1];
		
		// Addition
		case instr::Add:	return args[0] + args[1];
		
		// Subtraction
		case instr::Sub:	return args[0] - args[1];
		
		// Multiplication
		case instr::Mul:	return args[0] * args[1];
		
		// Unsigned Division
		case instr::UDiv:	return args[0] / args[1];
		
		// Unsigned Modulo reduction
		case instr::UMod:	return args[0] % args[1];
		
		default: assert(false);
		}
	}

	struct simple_sparse_constprop {};
	struct sscp_lattice
	{
		enum Type
		{
			NonConst = 2,
			Const = 1,
			Bottom = 0
		};

		sscp_lattice(void) : type(Bottom), value(0) {};
		sscp_lattice(Type t) : type(t), value(0) {};
		bool operator==(const sscp_lattice &a) const { return type == a.type && (type != Const || value == a.value); };
		
		Type type;
		uint64_t value;
	};

	template<>
	struct domain_traits<simple_sparse_constprop>
	{
		typedef sscp_lattice lattice;
	};

	sscp_lattice execute(const lvalue &left, instr::Function fn, const ::std::vector<rvalue> &concrete, const ::std::vector<sscp_lattice> &abstract, simple_sparse_constprop);
	sscp_lattice supremum(const sscp_lattice &a, const sscp_lattice &b, simple_sparse_constprop);
}

std::ostream &operator<<(std::ostream &os, const po::sscp_lattice &l);

#endif

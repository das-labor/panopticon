#include <vector>
#include <cmath>
#include <memory>
#include <map>
#include <set>

#include <boost/variant.hpp>

#include <panopticon/mnemonic.hh>
#include <panopticon/basic_block.hh>
#include <panopticon/procedure.hh>

/**
 * @file
 * @brief Abstract Interpretation framework
 *
 * Abstract Interpretation executes program using abstract
 * values. These are less precise than concrete values but
 * the algorithms computing them are guaranteed to terminate.
 *
 * This file defines a simple Abstract Interpreter over
 * Panopticon IL that can be plugged with arbitrary abstract
 * domains.
 *
 * Currently the only abstract domain implemented is Simple
 * Sparse Constant Propagation (SSCP).
 */

namespace po
{
	template <typename T>
	struct domain_traits {};

	struct meet {};
	struct join {};

	/**
	 * @brief Compute the Abstract Interpretation
	 *
	 * The function interprets a given procedure @ref proc
	 * using the abstract domain @ref tag. The @ref tag
	 * argument must be a model of the @c AbstractDomainConcept.
	 *
	 * The returned container maps every SSA variable in
	 * @ref proc to an instance of an element in the abstract domain.
	 */
	template<typename T>
	std::shared_ptr<std::unordered_map<rvalue,typename domain_traits<T>::lattice>> interpret(const proc_ptr proc, T tag = T())
	{
		typedef typename domain_traits<T>::lattice L;
		std::shared_ptr<std::unordered_map<rvalue,typename domain_traits<T>::lattice>> ret = std::make_shared(new std::unordered_map<rvalue,typename domain_traits<T>::lattice>());
		std::unordered_set<bblock_ptr> worklist(proc->basic_blocks);

		while(!worklist.empty())
		{
			bblock_ptr bb = *worklist.begin();
			bool modified = false;

			worklist.erase(worklist.begin());
			execute(bb,[&](const lvalue &left, instr::Function fn, const std::vector<rvalue> &right)
			{
				std::vector<L> arguments;
				L res = ret->count(left) ? ret->at(left) : L();

				for(const rvalue &r: right)
					if(ret->count(r))
						arguments.emplace_back(ret->at(r));
					else
						arguments.emplace_back(L());

				if(fn == instr::Phi)
					res = accumulate(arguments.begin(),arguments.end(),res,[&](const L &acc, const L &x) { return supremum(acc,x,tag); });
				else
					res = supremum(execute(left,fn,right,arguments,tag),res,tag);

				if(!ret->count(left) || !(ret->at(left) == res))
				{
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
			}

			std::cout << worklist.size() << std::endl;
		}

		return ret;
	}

	/**
	 * @brief Concrete semantics
	 * @group abstract_domains
	 *
	 * Concrete semantics of Panopticon IL over then domain of
	 * integers. The domain uses the standard C++ operators like
	 * plus and bitwise OR of the type I.
	 *
	 * @note This is not an abstract domain, hence the AI algorithm
	 * may not terminate.
	 */
	template<typename I>
	struct concrete_interp {};

	template<typename I>
	struct domain_traits<concrete_interp<I>>
	{
		using lattice = I;
	};

	/**
	 * Executes a IL statement using concrete semantics ot type I
	 * @internal
	 */
	template<typename I>
	I execute(const lvalue &left, instr::Function fn, const std::vector<rvalue> &concrete, const std::vector<I> &args,concrete_interp<I>)
	{
		switch(fn)
		{
		// Bitwise Not
		case instr::Not: return ~args[0];

		// Bitwise And
		case instr::And:	return args[0] & args[1];

		// Bitwise Or
		case instr::Or: return args[0] | args[1];

		// Bitwise Xor
		case instr::Xor:	return args[0] ^ args[1];

		// Assign Intermediate
		case instr::Assign: return args[0];

		// Unsigned right shift *
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

	/**
	 * @brief Simple Sparse Constant Propagation
	 * @ingroup abstract_domain
	 *
	 * A basic abstract domain where values are either
	 * unknown (Bottom), non constant (NonConst) or a
	 * constant integer.
	 *
	 * Useful for discovering values that do not depend
	 * on any input.
	 */
	struct simple_sparse_constprop {};
	using sscp_lattice = boost::variant<meet,join,uint64_t>;

	template<>
	struct domain_traits<simple_sparse_constprop>
	{
		using lattice = sscp_lattice;
	};

	/// @internal
	sscp_lattice execute(const lvalue &left, instr::Function fn, const std::vector<rvalue> &concrete, const std::vector<sscp_lattice> &abstract, simple_sparse_constprop);
	/// Computes the supremum of two sscp lattice elements
	sscp_lattice supremum(const sscp_lattice &a, const sscp_lattice &b, simple_sparse_constprop);
}

std::ostream &operator<<(std::ostream &os, const po::sscp_lattice &l);

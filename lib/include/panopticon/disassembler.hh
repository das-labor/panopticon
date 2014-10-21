#include <functional>
#include <list>
#include <map>
#include <string>
#include <iostream>
#include <vector>
#include <algorithm>
#include <stdexcept>
#include <memory>
#include <type_traits>
#include <cstring>
#include <string>
#include <memory>
#include <list>

#include <boost/optional.hpp>
#include <boost/variant.hpp>
#include <boost/iterator/reverse_iterator.hpp>

#include <panopticon/mnemonic.hh>
#include <panopticon/basic_block.hh>
#include <panopticon/code_generator.hh>
#include <panopticon/architecture.hh>
#include <panopticon/ensure.hh>
#include <panopticon/region.hh>

#pragma once

/**
 * @file
 * @brief Disassembler framework
 *
 * This is the lowest part of the analysis chain in Panopticon. The classes in this file turn raw
 * bytes into mnemonics and IL code. These are assembled into basic blocks, procedures and a flowgraph.
 *
 * The disassembler works as like a recursive decent parser for bit patterns. Internally instances of
 * subclasses of rule are strung together each consuming one token (integer) of input in their @c match
 * function, returning whenever the match was successful.
 *
 * The five important subclasses are
 * - conjunction: Takes two rules and calls both, passing the return value of the first into the second.
 * - disjunction: Takes a list of rules and calls each one returning the output of the first @c match function that is successful
 * - action: Takes a std::function and calls it with the current token stream. Always returns success.
 * - token_pat: Takes a string describing a bit pattern and returns successful if the current token matches.
 * - disassembler: Add a DSEL like interface to disjunction to ease the construction of a disassembler.
 *
 * The @c match functions of each class pass along a @c sem_state instance that is filled with informations of
 * the current token sequence. The function called in @c action uses @c sem_state to return a list of mnemonics and
 * successor addresses.
 *
 * The disassembler interprets a token stream. A token is a instance of a unsigned integer us arbitrary width. To
 * define it a specialization of the architecture_traits<> template is needed. All subclasses of rule are
 * parameterized with a type tag for architecture_traits.
 */

namespace po
{
	/**
	 * @brief Semantic state passing information about the tokens.
	 *
	 * A sem_state instance is passed down the chain of rule subclasses while it matches a
	 * token sequence. The state collects matched tokens and the values of capture groups defined
	 * along the way. The action instances pass it to the used supplied std::function callback. These
	 * add mnemonics and successor addresses to the sem_state. This information is used to construct
	 * basic blocks.
	 *
	 * The class includes helper functions to add mnemonics and jumps to its state.
	 */
	template<typename Tag>
	struct sem_state
	{
		using token = typename architecture_traits<Tag>::token_type;

		/**
		 * Construct a sem_state to analyze a token stream starting at address @c a
		 * @note The address is arbitrary.
		 */
		sem_state(offset a);

		/**
		 * Appends a @c len token long mnemonic for opcode @c n and operands @c ops,
		 * formatted according to @c fmt to the end of the mnemonic list.
		 * The @c fn argument is called with a code_generator that copies all IL into
		 * this new mnemonic.
		 */
		void mnemonic(size_t len, std::string n, std::string fmt = std::string(""), std::list<rvalue> ops = std::list<rvalue>(), std::function<void(code_generator<Tag>&)> fn = std::function<void(code_generator<Tag>&)>());

		/**
		 * Append a new mnemonic to this state. Overload for mnemonics with
		 * only one operand.
		 * @see mnemonic(size_t,std::string,std::string,std::list<rvalue>,std::function<void(code_generator<Tag>&)>)
		 */
		void mnemonic(size_t len, std::string n, std::string fmt, rvalue a, std::function<void(code_generator<Tag>&)> fn = std::function<void(code_generator<Tag>&)>());

		/**
		 * Append a new mnemonic to this state. Overload for mnemonics with
		 * only two operands.
		 * @see mnemonic(size_t,std::string,std::string,std::list<rvalue>,std::function<void(code_generator<Tag>&)>)
		 */
		void mnemonic(size_t len, std::string n, std::string fmt, rvalue a, rvalue b, std::function<void(code_generator<Tag>&)> fn = std::function<void(code_generator<Tag>&)>());

		/**
		 * Add a jump to this state. The class assumes that all mnemonics
		 * are executed as a sequence. After the last the position of the next mnemonic to
		 * be processed is chosen from a list of successor addresses (jumps).
		 * Each jump has a condition that is true in case the jump is taken. The jump
		 * target can be any rvalue.
		 *
		 * This function add a new possible successor address @c a that is chosen if
		 * the condition in @c g is true. An empty guard is always true.
		 */
		void jump(rvalue a, guard g = guard());

		/**
		 * Adds the address @c a to the set of possible successors.
		 * @see jump(rvalue, guard)
		 */
		void jump(offset a, guard g = guard());

		// in
		offset address;
		std::vector<token> tokens;
		std::map<std::string,unsigned int> capture_groups;

		// out
		std::list<po::mnemonic> mnemonics;
		std::list<std::pair<rvalue,guard>> jumps;

	private:
		offset next_address;
	};

	template<typename Tag>
	struct disassembler;

	struct token_expr
	{
		using iter = po::slab::iterator;

		struct conjunction { std::unique_ptr<token_expr> a, b; };
		struct option { std::unique_ptr<token_expr> e; };
		struct terminal { std::string s; };
		struct sub { void const* d; };

		using token_expr_union = boost::variant<
			conjunction,
			option,
			terminal,
			sub
		>;

		template<typename Tag>
		token_expr(disassembler<Tag> const&);
		token_expr(std::string const&);
		token_expr(unsigned long long);
		token_expr(token_expr const& e1,token_expr const& e2);
		token_expr(token_expr_union const& e);

		template<typename Tag>
		std::list<std::pair<
			std::list<std::pair<token,token>>,
			std::list<std::function<void(sem_state<Tag>&)>>
		>>
		to_pattern_list(void) const;

	private:
		token_expr_union _u;
	};

	token_expr operator*(token_expr const& e);
	token_expr operator""_e(char const* s,unsigned long l);
	token_expr operator""_e(unsigned long long l);
	token_expr operator>>(token_expr const& e1,token_expr const& e2);

	template<typename Tag>
	token_expr operator>>(token_expr const&,disassembler<Tag> const&);
	template<typename Tag>
	token_expr operator>>(disassembler<Tag> const&,token_expr const&);
	template<typename Tag>
	token_expr operator>>(disassembler<Tag> const&,disassembler<Tag> const&);

	/**
	 * @brief Matches a pattern of bits in a token
	 *
	 * This rule implements token patterns build either with strings or literal integers
	 */
	template<typename Tag>
	struct token_pat
	{
		using iter = po::slab::iterator;
		using token = typename architecture_traits<Tag>::token_type;

		token_pat(std::string const&);
		boost::optional<std::list<std::string,token>> matches(token) const;

	private:
		token _mask;
		token _pat;
		std::list<std::string,token> _capture;
	};

	/**
	 * @brief Thrown by disassembler to signal an invalid token pattern
	 */
	class token_pat_error : public std::invalid_argument
	{
	public:
		token_pat_error(std::string w = std::string("invalid token pattern"));
	};

	/**
	 * @brief Disassembles byte sequences into a stream of mnemonics.
	 *
	 * In order to be analyzed, object code from binaries has to be translated into Panopticons IL.
	 * This class scans an array of tokens (chunks of equal size) for patterns. If a match is found a
	 * function associated with this particular patters is called that returns a list of mnemonics
	 * and IL code that models the behaviour of the object code encoded in the matched token sequence.
	 *
	 * A user supplies patterns and functions that a disassembler instance uses to parse token streams.
	 * To help the readability of the rules a little domain specific embedded language is used to build
	 * the rules.
	 *
	 * A single disassembler instance holds any number of rules. The first matching rule is selected and the
	 * function associated with is is called. A rule is a sequence of token patterns and other disassembler
	 * instances. All patterns and disassemblers of a rule has to match in order for the function to be called.
	 *
	 * Each disassembler instance can have a default rule that has no token patterns or disassemblers and
	 * matches everything.
	 */
	template<typename Tag>
	struct disassembler
	{
		using iter = po::slab::iterator;
		using token = typename architecture_traits<Tag>::token_type;

		token_expr operator*(void) const { return token_expr(token_expr::token_expr_union(token_expr::option{std::unique_ptr<token_expr>(new token_expr(token_expr::token_expr_union(token_expr::sub{reinterpret_cast<void const*>(this)})))})); }
		std::function<void(void)>& operator[](token_expr const&);

		boost::optional<std::pair<iter,sem_state<Tag>>> try_match(iter b, iter e,sem_state<Tag> const&) const;

	private:
		std::list<std::pair<std::list<token_pat<Tag>>,std::function<void(void)>>> _pats;
	};

	template<typename Tag>
	void sem_state<Tag>::mnemonic(size_t len, std::string n, std::string fmt, std::list<rvalue> ops, std::function<void(code_generator<Tag>&)> fn)
	{
		std::list<instr> instrs;
		code_generator<Tag> cg(inserter(instrs,instrs.end()));

		try
		{
			dsl::current_code_generator = new dsl::callback_list(cg);

			if(fmt.empty())
				fmt = accumulate(ops.begin(),ops.end(),fmt,[](const std::string &acc, const rvalue &x)
					{ return acc + (acc.empty() ? "{8}" : ", {8}"); });

			// generate instr list
			if(fn)
				fn(cg);

			mnemonics.emplace_back(po::mnemonic(bound(next_address,next_address + len),n,fmt,ops.begin(),ops.end(),instrs.begin(),instrs.end()));
			next_address += len;
		}
		catch(...)
		{
			if(dsl::current_code_generator)
				delete dsl::current_code_generator;
			dsl::current_code_generator = 0;

			throw;
		}

		if(dsl::current_code_generator)
			delete dsl::current_code_generator;
		dsl::current_code_generator = 0;
	}

	template<typename Tag>
	void sem_state<Tag>::mnemonic(size_t len, std::string n, std::string fmt, rvalue a, std::function<void(code_generator<Tag>&)> fn)
	{
		std::list<rvalue> lst({a});
		return this->mnemonic(len,n,fmt,lst,fn);
	}

	template<typename Tag>
	void sem_state<Tag>::mnemonic(size_t len, std::string n, std::string fmt, rvalue a, rvalue b, std::function<void(code_generator<Tag>&)> fn)
	{
		return mnemonic(len,n,fmt,{a,b},fn);
	}

	template<typename Tag>
	void sem_state<Tag>::jump(rvalue a, guard g)
	{
		jumps.emplace_back(std::make_pair(a,g));
	}

	template<typename Tag>
	void sem_state<Tag>::jump(offset a, guard g)
	{
		jump(constant(a),g);
	}

	template<typename Tag>
	token_pat::token_pat(std::string const& c)
	: _mask(0), _pat(0), _capture()
	{
		int bit = sizeof(token) * 8 - 1;
		const char *p = c.c_str();
		std::unordered_map<std::string,token> cgs;
		boost::optional<token> cg_mask = boost::none;
		std::string cg_name;
		enum pstate { ANY, AT, PAT } ps = ANY;

		while(*p != 0 && bit >= 0)
		{
			switch(ps)
			{
				// scan 1/0, skip spaces, wait for start of capture pattern
				case ANY:
				{
					if(*p == '0' || *p == '1')
					{
						_pat |= (*p - '0') << bit;
						_mask |= 1 << bit;
						--bit;
						++p;
					}
					else if(isalpha(*p))
					{
						cg_name.assign(1,*p);
						ps = AT;
						++p;
					}
					else if(*p == ' ')
					{
						++p;
					}
					else
					{
						throw token_pat_error("invalid pattern at column " + std::to_string(p - c) + " '" + std::string(c) + "'");
					}

					break;
				}

				// scan name of capture pattern until '@'
				case AT:
				{
					if(*p == '@')
					{
						if(!cgs.count(cg_name))
							cgs.emplace(cg_name,0);
						cg_mask = &cgs[cg_name];
						ps = PAT;
						++p;
					}
					else if(isalpha(*p))
					{
						cg_name.append(1,*p);
						++p;
					}
					else
					{
						throw token_pat_error("invalid pattern at column " + std::to_string(p - c) + " '" + std::string(c) + "'");
					}
					break;
				}

				// scan '.' pattern
				case PAT:
				{
					if(*p == '.')
					{
						if(!cg_mask)
							throw token_pat_error();

						*cg_mask |= 1 << bit;
						--bit;
						++p;
					}
					else
					{
						ps = ANY;
					}
					break;
				}

				default:
				{
					throw token_pat_error("invalid pattern at column " + std::to_string(p-c) + " '" + std::string(c) + "'");
				}
			}
		}

		if(*p != 0)
			throw token_pat_error();

		// left extend a too short token pattern with zeros
		if(bit > -1)
		{
			int tshift = sizeof(token) * 8 - bit - 1, mshift = bit + 1;
			token t = 0;

			while(bit-- > -1)
				t = (t << 1) | 1;

			_mask = (_mask >> mshift) | (t << tshift);
			_pat = _pat >> mshift;
		}

		std::copy(cgs.begin(),cgs.end(),std::back_inserter(_capture));
	}

	template<typename Tag>
	boost::optional<std::list<std::string,token>> token_pat::matches(token t) const
	{
		if((t & _mask) == _pat)
		{
			std::list<std::pair<std::string,token>> ret;
			auto cg_iter = _capture.cbegin();

			while(cg_iter != _capture.cend())
			{
				token cg_mask = cg_iter->second;
				unsigned int res = 0;
				int bit = sizeof(token) * 8 - 1;
				auto i = std::find_if(ret.begin(),ret.end(),[&](std::pair<std::string,token>& p)
					{ return p.first == cg_iter->first; });

				if(i != ret.end())
				{
					res = i->second;
					ret.erase(i);
				}

				while(bit >= 0)
				{
					if((cg_mask >> bit) & 1)
						res = (res << 1) | ((t >> bit) & 1);
					--bit;
				}

				ret.emplace_back(cg_iter->first,res);
				++cg_iter;
			}

			return ret;
		}
		else
		{
			return boost::none;
		}
	}

	template<typename Tag>
	token_expr::token_expr(disassembler<Tag> const& d)
	: _u(sub{reinterpret_cast<void const*>(&d)})
	{}

	template<typename Tag>
	std::list<std::pair<
		std::list<std::pair<token,token>>,
		std::list<std::function<void(sem_state<Tag>&)>>
	>>
	token_expr::to_pattern_list(void) const
	{
		using token = architecture_traits<Tag>::token;
		using toklist = std::list<std::pair<token,token>>;
		using actlist = std::list<std::function<void(sem_state<Tag>&)>>;
		using ret_type = std::list<std::pair<toklist,actlist>>;

		struct vis : public boost::static_visitor<ret_type>
		{
			ret_type operator()(conjunction const& c) const
			{
				ret_type a = c.a->to_pattern_list();
				ret_type b = c.b->to_pattern_list();
				ret_type ret;

				for(auto x: a)
				{
					for(auto y: b)
					{
						toklist tl;
						actlist al;

						std::copy(x.first.begin(),x.first.end(),std::back_inserter(tl));
						std::copy(y.first.begin(),y.first.end(),std::back_inserter(tl));
						std::copy(x.second.begin(),x.second.end(),std::back_inserter(al));
						std::copy(y.second.begin(),y.second.end(),std::back_inserter(al));

						ret.emplace_back(tl,al);
					}
				}

				return ret;
			}

			ret_type operator()(terminal const& c) const
			{
				token_pat pat(c);
				toklist tl;

				tl.emplace_back(pat.mask(),pat.pattern());
				return ret_type({std::make_pair(tl,actlis())});
			}

			ret_type operator()(sub const& c) const
			{
				return reinterpret_cast<disassembler<Tag> const*>(c.d)->patterns();
			}

			ret_type operator()(option const& c) const
			{
				ret_type o = c.e->to_pattern_list();

				o.emplace_back(toklist(),actlist());

				return o;
			}
		};

		vis v();
		return boost::apply_visitor(_u,v);
	}
}

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
		 * Append a new mnemonic to this state. Overload for mnemonics with
		 * dynamic number of operands.
		 * @see mnemonic(size_t,std::string,std::string,std::list<rvalue>,std::function<void(code_generator<Tag>&)>)
		 */
		void mnemonic(size_t len, std::string n, std::string fmt, std::function<std::list<rvalue>(code_generator<Tag>&)> fn);

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
		std::map<std::string,uint64_t> capture_groups;

		// out
		std::list<po::mnemonic> mnemonics;
		std::list<std::pair<rvalue,guard>> jumps;

		typename architecture_traits<Tag>::state_type state;

	private:
		offset next_address;
	};

	template<typename Tag>
	struct disassembler;

	template<typename Tag>
	struct token_match
	{
		using token_type = typename architecture_traits<Tag>::token_type;

		std::list<std::pair<token_type,token_type>> patterns;
		std::list<std::function<void(sem_state<Tag>&)>> sem_actions;
		std::unordered_map<std::string,std::list<token_type>> cap_groups;
	};

	struct token_expr
	{
		using iter = po::slab::iterator;

		struct conjunction
		{
			conjunction(token_expr const& _a, token_expr const& _b)
			: a(new token_expr(_a)), b(new token_expr(_b))
			{}

			conjunction(conjunction const& c)
			: a(new token_expr(*c.a)), b(new token_expr(*c.b))
			{}

			conjunction& operator=(conjunction const& c)
			{
				if(this != &c)
				{
					a.reset(new token_expr(*c.a));
					b.reset(new token_expr(*c.b));
				}
				return *this;
			}

			std::unique_ptr<token_expr> a, b;
		};

		struct option
		{
			option(token_expr const& _e)
			: e(new token_expr(_e))
			{}

			option(option const& o)
			: e(new token_expr(*o.e))
			{}

			option& operator=(option const& o)
			{
				if(this != &o)
					e.reset(new token_expr(*o.e));
				return *this;
			}

			std::unique_ptr<token_expr> e;
		};
		struct terminal { boost::variant<std::string,unsigned long long> s; };
		struct sub { void const* d; };

		using token_expr_union = boost::variant<
			terminal,
			option,
			conjunction,
			sub
		>;

		template<typename Tag>
		token_expr(disassembler<Tag> const&);
		token_expr(std::string const&);
		token_expr(unsigned long long);
		token_expr(token_expr const& e1,token_expr const& e2);
		token_expr(token_expr_union const& e);

		token_expr(void) = delete;

		template<typename Tag>
		std::list<token_match<Tag>> to_match(void) const;

	private:
		token_expr_union _u;
	};

	token_expr operator*(token_expr const& e);
	token_expr operator"" _e(char const* s,unsigned long l);
	token_expr operator"" _e(unsigned long long l);
	token_expr operator>>(token_expr const& e1,token_expr const& e2);

	template<typename Tag>
	token_expr operator>>(token_expr const& t,disassembler<Tag> const& d)
	{
		return t >> token_expr(d);
	}

	template<typename Tag>
	token_expr operator>>(disassembler<Tag> const& d,token_expr const& t)
	{
		return token_expr(d) >> t;
	}

	template<typename Tag>
	token_expr operator>>(disassembler<Tag> const& d1,disassembler<Tag> const& d2)
	{
		return token_expr(d1) >> token_expr(d2);
	}

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
		token_pat(token);

		std::list<token_match<Tag>> to_match(void) const;

	private:
		token _mask;
		token _pat;
		std::list<std::pair<std::string,token>> _capture;
	};

	/**
	 * @brief Thrown by disassembler to signal an invalid token pattern
	 */
	class tokpat_error : public std::invalid_argument
	{
	public:
		tokpat_error(std::string w = std::string("invalid token pattern"));
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

		struct assignment_proxy
		{
			using piter = typename std::list<token_match<Tag>>::iterator;

			assignment_proxy(piter b,piter e) : _begin(b), _end(e) {};
			assignment_proxy& operator=(std::function<void(sem_state<Tag>&)> fn)
			{
				auto i = _begin;
				while(i != _end)
					(i++)->sem_actions.push_back(fn);

				return *this;
			}

		private:
			piter _begin, _end;
		};

		disassembler<Tag>& operator=(std::function<void(sem_state<Tag>&)>);
		disassembler<Tag>& operator=(disassembler<Tag> const&);
		token_expr operator*(void) const;

		assignment_proxy operator[](token_expr const&);
		assignment_proxy operator[](disassembler<Tag> const&);
		assignment_proxy operator[](token);
		assignment_proxy operator[](std::string const&);

		std::list<token_match<Tag>> const& patterns(void) const { return _pats; }

		boost::optional<std::pair<iter,sem_state<Tag>>> try_match(iter b, iter e,sem_state<Tag> const&) const;

	private:
		boost::optional<std::function<void(sem_state<Tag>&)>> _default;
		std::list<token_match<Tag>> _pats;
	};

	template<typename Tag>
	sem_state<Tag>::sem_state(offset a)
	: address(a), tokens(), capture_groups(), mnemonics(), jumps(), state(), next_address(a)
	{}

	template<typename Tag>
	void sem_state<Tag>::mnemonic(size_t len, std::string n, std::string fmt, std::function<std::list<rvalue>(code_generator<Tag>&)> fn)
	{
		std::list<instr> instrs;
		code_generator<Tag> cg(inserter(instrs,instrs.end()));

		ensure(fn);

		try
		{
			dsl::current_code_generator = new dsl::callback_list(cg);

			// generate instr list
			std::list<rvalue> ops = fn(cg);

			if(fmt.empty())
				fmt = accumulate(ops.begin(),ops.end(),fmt,[](const std::string &acc, const rvalue &x)
					{ return acc + (acc.empty() ? "{8}" : ", {8}"); });

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
	void sem_state<Tag>::mnemonic(size_t len, std::string n, std::string fmt, std::list<rvalue> ops, std::function<void(code_generator<Tag>&)> fn)
	{
		return this->mnemonic(len,n,fmt,[fn,ops](code_generator<Tag>& cg)
			{ if(fn) { fn(cg); } return ops; });
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
	token_pat<Tag>::token_pat(typename architecture_traits<Tag>::token_type t)
	: _mask(std::numeric_limits<typename architecture_traits<Tag>::token_type>::max()), _pat(t), _capture()
	{}

	template<typename Tag>
	token_pat<Tag>::token_pat(std::string const& _c)
	: _mask(0), _pat(0), _capture()
	{
		int bit = sizeof(token) * 8 - 1;
		char const* c = _c.c_str();
		char const* p = c;
		std::unordered_map<std::string,token> cgs;
		boost::optional<std::string> cg_name;
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
						cg_name = std::string(1,*p);
						ps = AT;
						++p;
					}
					else if(*p == ' ')
					{
						++p;
					}
					else
					{
						throw tokpat_error("invalid pattern at column " + std::to_string(p - c) + " '" + std::string(c) + "'");
					}

					break;
				}

				// scan name of capture pattern until '@'
				case AT:
				{
					if(*p == '@' && cg_name)
					{
						if(!cgs.count(*cg_name))
							cgs.emplace(*cg_name,0);
						ps = PAT;
						++p;
					}
					else if(isalpha(*p) && cg_name)
					{
						cg_name->append(1,*p);
						++p;
					}
					else
					{
						throw tokpat_error("invalid pattern at column " + std::to_string(p - c) + " '" + std::string(c) + "'");
					}
					break;
				}

				// scan '.' pattern
				case PAT:
				{
					if(*p == '.' && cg_name)
					{
						cgs[*cg_name] |= 1 << bit;
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
					throw tokpat_error("invalid pattern at column " + std::to_string(p-c) + " '" + std::string(c) + "'");
				}
			}
		}

		if(*p != 0)
			throw tokpat_error();

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
	std::list<token_match<Tag>> token_pat<Tag>::to_match(void) const
	{
		token_match<Tag> ret;

		ret.patterns.emplace_back(_mask,_pat);
		for(auto c: _capture)
		{
			ensure(!ret.cap_groups.count(c.first));
			std::list<token> l({c.second});
			ret.cap_groups.emplace(c.first,l);
		}

		return {ret};
	}

	template<typename Tag>
	token_expr::token_expr(disassembler<Tag> const& d)
	: _u(sub{reinterpret_cast<void const*>(&d)})
	{}

	template<typename Tag>
	std::list<token_match<Tag>> token_expr::to_match(void) const
	{
		using ret_type = std::list<token_match<Tag>>;
		using token_type = typename architecture_traits<Tag>::token_type;

		struct vis : public boost::static_visitor<ret_type>
		{
			ret_type operator()(conjunction const& c) const
			{
				ret_type a = c.a->to_match<Tag>();
				ret_type b = c.b->to_match<Tag>();
				ret_type ret;

				for(auto x: a)
				{
					for(auto y: b)
					{
						token_match<Tag> tm;

						std::copy(x.patterns.begin(),x.patterns.end(),std::back_inserter(tm.patterns));
						std::copy(y.patterns.begin(),y.patterns.end(),std::back_inserter(tm.patterns));
						std::copy(x.sem_actions.begin(),x.sem_actions.end(),std::back_inserter(tm.sem_actions));
						std::copy(y.sem_actions.begin(),y.sem_actions.end(),std::back_inserter(tm.sem_actions));

						tm.cap_groups = x.cap_groups;

						for(std::pair<std::string const,std::list<token_type>>& c: tm.cap_groups)
							while(c.second.size() < tm.patterns.size())
								c.second.emplace_back(0);

						for(auto c: y.cap_groups)
						{
							if(tm.cap_groups.count(c.first))
							{
								std::list<token_type> tmp;

								std::copy(tm.cap_groups.at(c.first).begin(),std::next(tm.cap_groups.at(c.first).begin(),x.patterns.size()),std::back_inserter(tmp));
								std::copy(c.second.begin(),c.second.end(),std::back_inserter(tmp));
								tm.cap_groups.erase(c.first);
								tm.cap_groups.emplace(c.first,tmp);
							}
							else
							{
								std::list<token_type> tmp(x.patterns.size(),0);

								std::copy(c.second.begin(),c.second.end(),std::back_inserter(tmp));
								tm.cap_groups.emplace(c.first,tmp);
							}
						}

						ensure(std::all_of(tm.cap_groups.begin(),tm.cap_groups.end(),[&](std::pair<std::string,std::list<token_type>> const& l)
							{ return l.second.size() == tm.patterns.size(); }));
						ret.emplace_back(tm);
					}
				}

				return ret;
			}

			ret_type operator()(terminal const& c) const
			{
				struct vis : public boost::static_visitor<token_pat<Tag>>
				{
					token_pat<Tag> operator()(std::string const& s) const { return token_pat<Tag>(s); }
					token_pat<Tag> operator()(unsigned long long i) const { return token_pat<Tag>(i); }
				};

				return boost::apply_visitor(vis(),c.s).to_match();
			}

			ret_type operator()(sub const& c) const
			{
				return reinterpret_cast<disassembler<Tag> const*>(c.d)->patterns();
			}

			ret_type operator()(option const& c) const
			{
				ret_type o = c.e->to_match<Tag>();

				o.emplace_back(token_match<Tag>());

				return o;
			}
		};

		vis v;
		return boost::apply_visitor(v,_u);
	}

	template<typename Tag>
	disassembler<Tag>& disassembler<Tag>::operator=(std::function<void(sem_state<Tag>&)> fn)
	{
		_default = fn;
		return *this;
	}

	template<typename Tag>
	disassembler<Tag>& disassembler<Tag>::operator=(disassembler<Tag> const& o)
	{
		if(this != &o)
		{
			_default = o._default;
			_pats = o._pats;
		}

		return *this;
	}

	template<typename Tag>
	token_expr disassembler<Tag>::operator*(void) const
	{
		token_expr e(token_expr::sub{reinterpret_cast<void const*>(this)});
		return token_expr(token_expr::token_expr_union(token_expr::option(e)));
	}

	template<typename Tag>
	typename disassembler<Tag>::assignment_proxy disassembler<Tag>::operator[](token_expr const& t)
	{
		auto pl = t.to_match<Tag>();
		size_t l = pl.size();

		std::move(pl.begin(),pl.end(),std::back_inserter(_pats));
		return assignment_proxy(std::next(_pats.begin(),_pats.size() - l),_pats.end());
	}

	template<typename Tag>
	typename disassembler<Tag>::assignment_proxy disassembler<Tag>::operator[](disassembler<Tag> const& d)
	{
		return operator[](token_expr(d));
	}

	template<typename Tag>
	typename disassembler<Tag>::assignment_proxy disassembler<Tag>::operator[](token t)
	{
		return operator[](token_expr(t));
	}

	template<typename Tag>
	typename disassembler<Tag>::assignment_proxy disassembler<Tag>::operator[](std::string const& s)
	{
		return operator[](token_expr(s));
	}

	template<typename Tag>
	boost::optional<std::pair<slab::iterator,sem_state<Tag>>> disassembler<Tag>::try_match(slab::iterator b, slab::iterator e,sem_state<Tag> const& _st) const
	{
		using token = typename architecture_traits<Tag>::token_type;

		std::list<token> read;
		size_t const len = std::distance(b,e);
		std::function<boost::optional<token>(void)> read_next;

		read_next = [&](void) -> boost::optional<token>
		{
			auto i = b + read.size() * sizeof(token);
			bool const defined = std::none_of(i,i + sizeof(token),[](po::tryte s) { return !s; });

			if(!defined)
				return boost::none;

			std::array<uint8_t,sizeof(token)> tmp;

			std::transform(i,i + sizeof(token),tmp.begin(),[](po::tryte b) { return *b; });
			return std::accumulate(tmp.rbegin(),tmp.rend(),0,[](token acc, uint8_t b) { return (acc << 8) | b; });
		};

		if(len > 0)
		{
			for(auto const& opt: _pats)
			{
				auto const& pattern = opt.patterns;
				auto const& actions = opt.sem_actions;

				if(len < pattern.size() * sizeof(token))
					continue;

				while(read.size() < pattern.size())
				{
					auto maybe_token = read_next();

					if(!maybe_token)
						break;
					else
						read.push_back(*maybe_token);
				}

				if(read.size() < pattern.size())
					continue;

				auto j = pattern.begin();
				auto k = read.begin();
				bool match = true;

				while(match && j != pattern.end())
				{
					ensure(k != read.end());

					match &= (j->first & *k) == j->second;
					++j;
					++k;
				}

				if(match)
				{
					sem_state<Tag> st(_st);

					for(auto cap: opt.cap_groups)
					{
						std::list<token> masks = cap.second;
						unsigned int res;

						ensure(masks.size() == pattern.size());

						if(st.capture_groups.count(cap.first))
						{
							res = st.capture_groups.at(cap.first);
							st.capture_groups.erase(cap.first);
						}
						else
						{
							res = 0;
						}

						auto t = read.begin();
						for(auto cg_mask: masks)
						{
							if(cg_mask == 0)
							{
								++t;
								continue;
							}

							ensure(t != k);
							int bit = sizeof(token) * 8 - 1;
							while(bit >= 0)
							{
								if((cg_mask >> bit) & 1)
									res = (res << 1) | ((*t >> bit) & 1);
								--bit;
							}

							++t;
						}

						st.capture_groups.emplace(cap.first,res);
					}

					std::copy(read.begin(),k,std::back_inserter(st.tokens));
					std::for_each(actions.begin(),actions.end(),[&](std::function<void(sem_state<Tag>&)> fn) { fn(st); });

					return std::make_pair(b + pattern.size() * sizeof(token),st);
				}
			}

			if(_default)
			{
				sem_state<Tag> st(_st);

				if(read.empty())
				{
					auto maybe_token = read_next();

					ensure(maybe_token);
					read.push_back(*maybe_token);
				}

				st.tokens.push_back(read.front());
				(*_default)(st);

				return std::make_pair(b + sizeof(token),st);
			}
		}

		return boost::none;
	}
}

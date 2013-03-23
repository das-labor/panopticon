#ifndef DISASSEMBLER_HH
#define DISASSEMBLER_HH

#include <functional>
#include <list>
#include <map>
#include <string>
#include <iostream>
#include <cassert>
#include <vector>
#include <algorithm>
#include <stdexcept>
#include <memory>

#include <architecture.hh>
#include <code_generator.hh>
#include <mnemonic.hh>
#include <basic_block.hh>

/**
 * @file
 * @brief Disassembler framework
 *
 * This is the lowest part of the analysis chain in Panopticum. The classes in this file turn raw 
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
 * - tokpat: Takes a string describing a bit pattern and returns successful if the current token matches.
 * - disassembler: Add a DSEL like interface to disjunction to ease the construction of a disassembler.
 *
 * The @c match functions of each class pass along a @c sem_state instance that is filled with informations of
 * the current token sequence. The function called in @c action uses @c sem_state to return a list of mnemonics and
 * successor addresses.
 *
 * The disassembler interprets a token stream. A token is a instance of a unsigned integer us arbitrary width. To
 * define it a specialization of the architecture_traits<> template is needed. All subclasses of rule are
 * parameterized with a type tag for architecture_traits.
 *
 * @todo The whole file leaks memory as fuck. Switch to shared_ptr.
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
		typedef typename architecture_traits<Tag>::token_type token;
		typedef typename std::vector<typename architecture_traits<Tag>::token_type>::iterator tokiter;

		/**
		 * Construct a sem_state to analyze a token stream starting at address @c a
		 * @note The address is arbitrary.
		 */
		sem_state(addr_t a);

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
		void jump(rvalue a, guard_ptr g = guard_ptr(new guard()));

		/**
		 * Adds the address @c a to the set of possible successors.
		 * @see jump(rvalue, guard_ptr)
		 */
		void jump(addr_t a, guard_ptr g = guard_ptr(new guard()));

		// in
		addr_t address;
		std::vector<token> tokens;
		std::map<std::string,unsigned int> capture_groups;
		
		// out
		std::list<po::mnemonic> mnemonics;
		std::list<std::pair<rvalue,guard_ptr>> jumps;
		
	private:
		addr_t next_address;
	};

	/**
	 * @brief Base of all parsing rules in a disassembler instance.
	 *
	 * Subclasses are required to implement @c match that consumes a part of
	 * the token stream.
	 */
	template<typename Tag>
	class rule
	{ 
	public: 
		typedef typename architecture_traits<Tag>::token_type token;
		typedef typename std::vector<typename architecture_traits<Tag>::token_type>::iterator tokiter;

		virtual ~rule(void);

		/**
		 * Apply this rule on the token stream delimited by @c begin and @c end, 
		 * using @c state to pass information to subsequent rules.
		 * @returns A pair with the first field true if the rule was successful and the second set to an iterator pointing to the next token to read by the next rules.
		 */
		virtual std::pair<bool,tokiter> match(tokiter begin, tokiter end, sem_state<Tag> &state) const = 0;
	};

	template<typename Tag>
	using rule_ptr = std::shared_ptr<rule<Tag>>;

	/**
	 * @brief Semantic action
	 *
	 * This class finished a rule sequence by calling a user-definied 
	 * function to carry out the translation of a matched token range to mnemonics.
	 */ 
	template<typename Tag>
	class action : public rule<Tag>
	{
	public:
		/// @param f Function to be called if this rule is tried to match.
		action(std::function<void(sem_state<Tag>&)> &f);

		virtual ~action(void);

		/**
		 * Calls the user-definied function.
		 * @returns Always success, iterator pointing to @c end.
		 */
		virtual std::pair<bool,typename rule<Tag>::tokiter> match(typename rule<Tag>::tokiter begin, typename rule<Tag>::tokiter end, sem_state<Tag> &state) const;

		std::function<void(sem_state<Tag>&)> semantic_action;
	};

	/**
	 * @brief Matches a pattern of bits in a token
	 *
	 * This rule implements token patterns build either with strings or literal integers
	 */
	template<typename Tag>
	class tokpat : public rule<Tag>
	{
	public: 	
		/**
		 * Constructs a new tokpa rule.
		 * @param m Token value to match.
		 * @param pat Bits to mask before comparing a candidate token to @c m. Used to realize match-all sequences like "10...".
		 * @param cg Capture groups. Key is the group name, value a bit mask describing the bits to save in the group.
		 */
		tokpat(typename rule<Tag>::token m, typename rule<Tag>::token pat, std::map< std::string,typename rule<Tag>::token> &cg);

		virtual ~tokpat(void);

		/// Matches one or no token.
		virtual std::pair<bool,typename rule<Tag>::tokiter> match(typename rule<Tag>::tokiter begin, typename rule<Tag>::tokiter end, sem_state<Tag> &state) const;

	private:
		typename rule<Tag>::token mask;
		typename rule<Tag>::token pattern;
		std::map< std::string,typename rule<Tag>::token> capture_patterns;
	};

	/**
	 * @brief OR rule
	 * 
	 * Tries a number of rules until one matches and returns its result.
	 */
	template<typename Tag>
	class disjunction : public rule<Tag>
	{
	public:
		virtual ~disjunction(void);

		/// Runs all rules with the supplied arguments and returns the result of the first successful rule.
		virtual std::pair<bool,typename rule<Tag>::tokiter> match(typename rule<Tag>::tokiter begin, typename rule<Tag>::tokiter end, sem_state<Tag> &state) const;

		/// Append the rule @c r to the end of the list of rules to run if @c match is called.
		void chain(rule_ptr<Tag> r);
						
	private:
		std::list<rule_ptr<Tag> > patterns;
	};

	/**
	 * @brief AND rule
	 *
	 * An instance of this class is constructed from two other rules. 
	 * The result of the first is put in the second on if the first is 
	 * successful. The result of the second is returned.
	 */
	template<typename Tag>
	class conjunction : public rule<Tag>
	{
	public:
		/// Construct a new instance using @c a and @c b as first and second rule to run.
		conjunction(rule_ptr<Tag> a, rule_ptr<Tag> b);
		
		virtual ~conjunction(void);

		/**
		 * Runs the first rule with the supplied arguments if it is successful the second 
		 * rule is run with that result of the first as arguments. The result of this 
		 * second rule is returned. If the first rule fails its result is returned.
		 */
		virtual std::pair<bool,typename rule<Tag>::tokiter> match(typename rule<Tag>::tokiter begin, typename rule<Tag>::tokiter end, sem_state<Tag> &state) const;
		
	private:
		rule_ptr<Tag> first, second;
	};

	/**
	 * @brief Thrown by disassembler to signal an invalid token pattern
	 */
	class tokpat_error : public std::invalid_argument
	{
	public:
		tokpat_error(void);
	};

	/**
	 * @brief Disassembles byte sequences into a stream of mnemonics.
	 *
	 * In order to be analyzed, object code from binaries has to be translated into Panopticums IL.
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
	class disassembler : public disjunction<Tag>
	{
	public:
		/// Constructs a disassembler with empty ruleset matching nothing.
		disassembler(void);

		virtual ~disassembler(void);

		/**
		 * Sets the function of the rule constructed last to @c f and starts a new rule.
		 * @note This function does not behave as a standard assignment operator!
		 * @returns self
		 */
		disassembler &operator=(std::function<void(sem_state<Tag>&)> f);
		
		/**
		 * Appends the token pattern @c i to the currently constructed rule.
		 * @note This function does not behave like a bitwise OR!
		 * @returns self to allow joining of | operations.
		 */
		disassembler &operator|(typename rule<Tag>::token i);
		
		/**
		 * @brief Adds a token pattern.
		 * A token pattern is a string describing a token as a sequence of bits. Simple patters
		 * consist of "0" and "1". The pattern "01101100" matches a token with value 108 decimal.
		 * Token patterns can include "." which match both 0 and 1 bits. The pattern "00.." matches
		 * 0, 1, 2 and 3 decimal. Spaces are ignored: "0 0 0" matches the same tokens as "000". 
		 * To get the concrete values of the bits matched with "." a capture group can be used. 
		 * Each group has a name and an associated range of "." signs. The pattern "a@..." defines
		 * the capture group "a" holding the value of the three lower bits the @ sign divides group
		 * name and sub pattern. Capture groups extend to the next space. If a capture group occurs 
		 * more than once in a pattern, its bits are concatenated in the order the show up in the
		 * pattern. The pattern "a@..0a@.." matches all tokens with the 3rd bit set to zero.
		 * The contents of a for token with value 01011 would be 0111. The name of a capture 
		 * group can only include upper and lower case letters. Empty capture groups ("a@") are allowed.
		 * Token patterns that are shorter than the token are left-extended with zeros. If the pattern
		 * is too wide a tokpat_error is thrown.
		 *
		 * @param c Token pattern formatted as above
		 * @returns self
		 * @throws tokpat_error On invalid token pattern
		 *
		 * @todo Make sure an exception is thrown if an invalid token pattern is feed into the function.
		 */
		disassembler &operator|(const char *c);

		/**
		 * Adds another disassembler to the currently constructed rule. The
		 * rule matches if any rule of @c dec matches. The function of @c runs before
		 * the function associated with this rule.
		 * @note This function does not behave like a bitwise OR!
		 * @returns self to allow joining of | operations.
		 */
		disassembler &operator|(disassembler<Tag> &dec);
		
		/**
		 * Tries to match a rule on the token sequence [begin,end), calling the associated function with @c state.
		 * @returns a pair. The boolean if true if a match was found and the iterator points to the token after the match.
		 */
		virtual std::pair<bool,typename rule<Tag>::tokiter> match(typename rule<Tag>::tokiter begin, typename rule<Tag>::tokiter end, sem_state<Tag> &state) const;
	
	protected:
		void append(rule_ptr<Tag> r);

	private:
		rule_ptr<Tag> current;
		std::shared_ptr<action<Tag>> failsafe;
	};

	template<typename Tag>
	action<Tag>::action(std::function<void(sem_state<Tag>&)> &f)
	: semantic_action(f) 
	{}

	template<typename Tag>
	action<Tag>::~action(void)
	{}

	// returns pair<is successful?,next token to consume>
	template<typename Tag>
	std::pair<bool,typename rule<Tag>::tokiter> action<Tag>::match(typename rule<Tag>::tokiter begin, typename rule<Tag>::tokiter end, sem_state<Tag> &state) const
	{
		if(this->semantic_action)
			semantic_action(state);
		return std::make_pair(true,begin);
	}

	template<typename Tag>
	rule<Tag>::~rule<Tag>(void)
	{}

	template<typename Tag>
	tokpat<Tag>::~tokpat(void)
	{}

	template<typename Tag>
	tokpat<Tag>::tokpat(typename rule<Tag>::token m, typename rule<Tag>::token pat, std::map< std::string,typename rule<Tag>::token> &cg)
	: mask(m), pattern(pat), capture_patterns(cg) 
	{}

	template<typename Tag>
	std::pair<bool,typename rule<Tag>::tokiter> tokpat<Tag>::match(typename rule<Tag>::tokiter begin, typename rule<Tag>::tokiter end, sem_state<Tag> &state) const
	{
		if(begin == end)
			return std::make_pair(false,begin);

		typename rule<Tag>::token t = *begin;

		if((t & mask) == pattern)
		{
			auto cg_iter = capture_patterns.cbegin();

			while(cg_iter != capture_patterns.cend())
			{
				typename rule<Tag>::token mask = cg_iter->second;
				unsigned int res = 0;
				int bit = sizeof(typename rule<Tag>::token) * 8 - 1;

				if(state.capture_groups.count(cg_iter->first))
					res = state.capture_groups.find(cg_iter->first)->second;

				while(bit >= 0)
				{
					if((mask >> bit) & 1)
						res = (res << 1) | ((t >> bit) & 1);
					--bit;
				}

				if(state.capture_groups.count(cg_iter->first))
					state.capture_groups.find(cg_iter->first)->second = res;
				else
					state.capture_groups.insert(std::make_pair(cg_iter->first,res));
				++cg_iter;
			}

			state.tokens.push_back(t);

			return std::make_pair(true,next(begin));
		}
		else
			return std::make_pair(false,begin);
	}

	template<typename Tag>
	sem_state<Tag>::sem_state(addr_t a)
	: address(a), next_address(a) 
	{}

	template<typename Tag>
	void sem_state<Tag>::mnemonic(size_t len, std::string n, std::string fmt, std::list<rvalue> ops, std::function<void(code_generator<Tag>&)> fn)
	{
		std::list<instr> instrs;
		code_generator<Tag> cg(inserter(instrs,instrs.end()));

		if(fmt.empty())
			fmt = accumulate(ops.begin(),ops.end(),fmt,[](const std::string &acc, const rvalue &x) 
				{ return acc + (acc.empty() ? "{8}" : ", {8}"); });

		// generate instr list
		if(fn) 
			fn(cg);

		mnemonics.emplace_back(po::mnemonic(range<addr_t>(next_address,next_address + len),n,fmt,ops.begin(),ops.end(),instrs.begin(),instrs.end())); 
		next_address += len;
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
	void sem_state<Tag>::jump(rvalue a, guard_ptr g)
	{
		jumps.emplace_back(std::make_pair(a,g));
	}
	
	template<typename Tag>
	void sem_state<Tag>::jump(addr_t a, guard_ptr g)
	{
		jump(constant(a),g);
	}
	
	template<typename Tag>
	disjunction<Tag>::~disjunction(void)
	{}

	template<typename Tag>
	std::pair<bool,typename rule<Tag>::tokiter> disjunction<Tag>::match(typename rule<Tag>::tokiter begin, typename rule<Tag>::tokiter end, sem_state<Tag> &state) const
	{
		auto i = patterns.cbegin();
		typename rule<Tag>::tokiter j;

		while(i != patterns.cend())
		{
			rule<Tag> &r(**i++);
			std::pair<bool,typename rule<Tag>::tokiter> ret = r.match(begin,end,state);

			if(ret.first)
				return ret;
		}

		return std::make_pair(false,begin);
	}

	template<typename Tag>
	void disjunction<Tag>::chain(rule_ptr<Tag> r)
	{
		patterns.push_back(r);
	}

	template<typename Tag>
	conjunction<Tag>::conjunction(rule_ptr<Tag> a, rule_ptr<Tag> b)
	: first(a), second(b) 
	{ 
		assert(a && b);	
	}
	
	template<typename Tag>
	conjunction<Tag>::~conjunction(void)
	{}

	template<typename Tag>
	std::pair<bool,typename rule<Tag>::tokiter> conjunction<Tag>::match(typename rule<Tag>::tokiter begin, typename rule<Tag>::tokiter end, sem_state<Tag> &state) const
	{
		bool cont;
		typename rule<Tag>::tokiter i;

		tie(cont,i) = first->match(begin,end,state);

		if(cont)
			return second->match(i,end,state);
		else
			return std::make_pair(false,begin);
	}

	template<typename Tag>
	disassembler<Tag>::disassembler(void)
	: current(0), failsafe(0)
	{}
	
	template<typename Tag>
	disassembler<Tag>::~disassembler(void)
	{}

	template<typename Tag>
	disassembler<Tag> &disassembler<Tag>::operator=(std::function<void(sem_state<Tag>&)> f)
	{
		if(this->current)
		{
			this->chain(rule_ptr<Tag>(new conjunction<Tag>(current,rule_ptr<Tag>(new action<Tag>(f)))));
			this->current = 0;
		}
		else
		{
			this->failsafe = std::shared_ptr<action<Tag>>(new action<Tag>(f));
		}
			
		return *this;
	}
	
	template<typename Tag>
	disassembler<Tag> &disassembler<Tag>::operator|(typename rule<Tag>::token i)
	{
		std::map< std::string,typename rule<Tag>::token> cgs;
		append(rule_ptr<Tag>(new tokpat<Tag>(~((typename rule<Tag>::token)0),i,cgs)));
		return *this;
	}

	template<typename Tag>
	disassembler<Tag> &disassembler<Tag>::operator|(const char *c)
	{
		typename rule<Tag>::token mask = 0, pattern = 0;
		int bit = sizeof(typename rule<Tag>::token) * 8 - 1;
		const char *p = c;
		std::map< std::string,typename rule<Tag>::token> cgs;
		typename rule<Tag>::token *cg_mask = 0;
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
						pattern |= (*p - '0') << bit;
						mask |= 1 << bit;
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
						std::cout << "invalid pattern at column " << (int)(p - c) << " '" << c << "'" << std::endl;
						assert(false);
					}

					break;
				}

				// scan name of capture pattern until '@'
				case AT:
				{
					if(*p == '@')
					{
						if(!cgs.count(cg_name))
							cgs.insert(std::pair< std::string,typename rule<Tag>::token>(cg_name,0));
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
						std::cout << "invalid pattern at column " << (int)(p-c) << " '" << c << "'" << std::endl;
						assert(false);
					}
					break;
				}

				// scan '.' pattern
				case PAT:
				{
					if(*p == '.')
					{
						assert(cg_mask);
						
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
					std::cout << "invalid pattern at column " << (int)(p-c) << " '" << c << "'" << std::endl;
					assert(false);
				}
			}
		}
		
		if(bit < -1)
			throw tokpat_error();

		append(rule_ptr<Tag>(new tokpat<Tag>(mask,pattern,cgs)));
		return *this;
	}

	template<typename Tag>
	disassembler<Tag> &disassembler<Tag>::operator|(disassembler<Tag> &dec)
	{
		append(rule_ptr<Tag>(&dec,[](disassembler *) {}));
		return *this;
	}

	template<typename Tag>
	void disassembler<Tag>::append(rule_ptr<Tag> r)
	{
		if(!current)
			current = r;
		else
			current = rule_ptr<Tag>(new conjunction<Tag>(current,r));
	}
	
	template<typename Tag>
	std::pair<bool,typename rule<Tag>::tokiter> disassembler<Tag>::match(typename rule<Tag>::tokiter begin, typename rule<Tag>::tokiter end, sem_state<Tag> &state) const
	{
		std::pair<bool,typename rule<Tag>::tokiter> ret = disjunction<Tag>::match(begin,end,state);

		if(!ret.first && failsafe && begin != end)
		{
			state.tokens.push_back(*begin);
			return failsafe->match(next(begin),end,state);
		}
		else
			return ret;
	}
}

#endif

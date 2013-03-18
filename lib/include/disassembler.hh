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

#include <architecture.hh>
#include <code_generator.hh>
#include <mnemonic.hh>
#include <basic_block.hh>

namespace po
{
	template<typename Tag>
	struct sem_state
	{
		typedef typename architecture_traits<Tag>::token_type token;
		typedef typename std::vector<typename architecture_traits<Tag>::token_type>::iterator tokiter;

		sem_state(addr_t a);

		void mnemonic(size_t len, std::string n, std::string fmt = std::string(""), std::list<rvalue> ops = std::list<rvalue>(), std::function<void(code_generator<Tag>&)> fn = std::function<void(code_generator<Tag>&)>());
		void mnemonic(size_t len, std::string n, std::string fmt, rvalue a, std::function<void(code_generator<Tag>&)> fn = std::function<void(code_generator<Tag>&)>());
		void mnemonic(size_t len, std::string n, std::string fmt, rvalue a, rvalue b, std::function<void(code_generator<Tag>&)> fn = std::function<void(code_generator<Tag>&)>());
		

		void jump(rvalue a, guard_ptr g = guard_ptr(new guard()));
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

	template<typename Tag>
	class rule
	{ 
	public: 
		typedef typename architecture_traits<Tag>::token_type token;
		typedef typename std::vector<typename architecture_traits<Tag>::token_type>::iterator tokiter;

		// returns pair<is successful?,next token to consume>
		virtual std::pair<bool,tokiter> match(tokiter begin, tokiter end, sem_state<Tag> &state) const = 0;
	};

	template<typename Tag>
	class action : public rule<Tag>
	{
	public:
		action(std::function<void(sem_state<Tag>&)> &f);
		virtual ~action(void);

		// returns pair<is successful?,next token to consume>
		virtual std::pair<bool,typename rule<Tag>::tokiter> match(typename rule<Tag>::tokiter begin, typename rule<Tag>::tokiter end, sem_state<Tag> &state) const;

		std::function<void(sem_state<Tag>&)> semantic_action;
	};

	template<typename Tag>
	class tokpat : public rule<Tag>
	{
	public: 	
		tokpat(typename rule<Tag>::token m, typename rule<Tag>::token pat, std::map< std::string,typename rule<Tag>::token> &cg);
		virtual std::pair<bool,typename rule<Tag>::tokiter> match(typename rule<Tag>::tokiter begin, typename rule<Tag>::tokiter end, sem_state<Tag> &state) const;

	private:
		typename rule<Tag>::token mask;
		typename rule<Tag>::token pattern;
		std::map< std::string,typename rule<Tag>::token> capture_patterns;
	};

	template<typename Tag>
	class disjunction : public rule<Tag>
	{
	public:
		virtual std::pair<bool,typename rule<Tag>::tokiter> match(typename rule<Tag>::tokiter begin, typename rule<Tag>::tokiter end, sem_state<Tag> &state) const;
		void chain(rule<Tag> *r);
						
	private:
		std::list<rule<Tag> *> patterns;
	};

	template<typename Tag>
	class conjunction : public rule<Tag>
	{
	public:
		conjunction(rule<Tag> *a, rule<Tag> *b);
		virtual std::pair<bool,typename rule<Tag>::tokiter> match(typename rule<Tag>::tokiter begin, typename rule<Tag>::tokiter end, sem_state<Tag> &state) const;
		
	private:
		rule<Tag> *first, *second;
	};

	template<typename Tag>
	class disassembler : public disjunction<Tag>
	{
	public:
		disassembler(void);

		disassembler &operator=(std::function<void(sem_state<Tag>&)> f);
		
		disassembler &operator|(typename rule<Tag>::token i);
		disassembler &operator|(const char *c);
		disassembler &operator|(disassembler<Tag> &dec);
		
		void append(rule<Tag> *r);
		virtual std::pair<bool,typename rule<Tag>::tokiter> match(typename rule<Tag>::tokiter begin, typename rule<Tag>::tokiter end, sem_state<Tag> &state) const;

	private:
		rule<Tag> *current;
		action<Tag> *failsafe;
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
	void disjunction<Tag>::chain(rule<Tag> *r)
	{
		patterns.push_back(r);
	}

	template<typename Tag>
	conjunction<Tag>::conjunction(rule<Tag> *a, rule<Tag> *b)
	: first(a), second(b) 
	{ 
		assert(a && b);	
	}

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
	disassembler<Tag> &disassembler<Tag>::operator=(std::function<void(sem_state<Tag>&)> f)
	{
		if(this->current)
		{
			this->chain(new conjunction<Tag>(current,new action<Tag>(f)));
			this->current = 0;
		}
		else
		{
			if(this->failsafe)
				delete this->failsafe;
			this->failsafe = new action<Tag>(f);
		}
			
		return *this;
	}
	
	template<typename Tag>
	disassembler<Tag> &disassembler<Tag>::operator|(typename rule<Tag>::token i)
	{
		std::map< std::string,typename rule<Tag>::token> cgs;
		append(new tokpat<Tag>(~((typename rule<Tag>::token)0),i,cgs));
		return *this;
	}

	/**
	 * @brief Adds a token pattern.
	 * A token pattern is a string describing a token as a seqence of bits. Simple patters
	 * consist of "0" and "1". The pattern "01101100" matches a token with value 108 decimal.
	 * Token patterns can include "." which match both 0 and 1 bits. The pattern "00.." matches
	 * 0, 1, 2 and 3 decimal. 
	 * To get the concrete values of the bits matched with "." a capture group can be used. 
	 * Each group has a name and an associated range of "." signs. The pattern "a@..." definies
	 * the capture group "a" holding the value of the three lower bits the @ sign divides group
	 * name and sub pattern. If a capture group occurs more than once in a pattern, its bits 
	 * are concatenated in the order the show up in the pattern. The pattern "a@..0a@.." matches
	 * all tokens with the 3rd bit set to zero. The contents of a for token with value 01011 
	 * would be 0111. The name of a capture group can only include upper and lower case letters.
	 * Empty capture groups ("a@") are allowed.
	 * Token patterns that are shorter than the token are left-extended with zeros. If the pattern
	 * is too wide a tokpat_error is thrown.
	 *
	 * @param c Token pattern formatted as above
	 * @returns self
	 * @throws tokpat_error On invalid token pattern
	 *
	 * @todo Make sure an exception is thrown if an invalid token pattern is feed into the function.
	 */
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
		
		assert(bit == -1);
		append(new tokpat<Tag>(mask,pattern,cgs));
		return *this;
	}

	template<typename Tag>
	disassembler<Tag> &disassembler<Tag>::operator|(disassembler<Tag> &dec)
	{
		append(&dec);
		return *this;
	}

	template<typename Tag>
	void disassembler<Tag>::append(rule<Tag> *r)
	{
		if(!current)
			current = r;
		else
			current = new conjunction<Tag>(current,r);
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

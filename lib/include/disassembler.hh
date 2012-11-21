#ifndef DISASSEMBLER_HH
#define DISASSEMBLER_HH

#include <functional>
#include <list>
#include <map>
#include <string>
#include <iostream>
#include <cassert>
#include <vector>

#include <architecture.hh>
#include <code_generator.hh>
#include <mnemonic.hh>
#include <basic_block.hh>
#include <procedure.hh>

namespace po
{
	template<typename Tag>
	struct sem_state
	{
		typedef typename architecture_traits<Tag>::token_type token;
		typedef typename ::std::vector<typename architecture_traits<Tag>::token_type>::iterator tokiter;

		sem_state(addr_t a) : address(a), next_address(a) {};

		void mnemonic(size_t len, ::std::string n, ::std::list<rvalue> ops = ::std::list<rvalue>(), ::std::function<void(code_generator<Tag>&)> fn = ::std::function<void(code_generator<Tag>&)>())
		{
			assert(len);

			bblock_ptr new_bb, adj;
			::std::list<instr> instrs;
			code_generator<Tag> cg(inserter(instrs,instrs.end()));

			// generate instr list
			if(fn) fn(cg);

			last = bblock_ptr(new basic_block());
			last->mutate_mnemonics([&](::std::vector<po::mnemonic> &ms) 
			{ 
				ms.emplace_back(po::mnemonic(range<addr_t>(next_address,next_address + len),n,ops.begin(),ops.end(),instrs.begin(),instrs.end())); 
			});
			basic_blocks.insert(last);
		
			next_address += len;
		};

		void mnemonic(size_t len, ::std::string n, rvalue a, ::std::function<void(code_generator<Tag>&)> fn = ::std::function<void(code_generator<Tag>&)>())
		{
			::std::list<rvalue> lst({a});
			return this->mnemonic(len,n,lst,fn);
		}
		
		void mnemonic(size_t len, ::std::string n, rvalue a, rvalue b, ::std::function<void(code_generator<Tag>&)> fn = ::std::function<void(code_generator<Tag>&)>())
		{
			return mnemonic(len,n,{a,b},fn);
		}

		void jump(rvalue a, guard_ptr g = guard_ptr(new guard()))
		{
			assert(last && !basic_blocks.empty());
			last->mutate_outgoing([&](::std::list<ctrans> &out) { out.emplace_back(ctrans(g,a)); });
		};
		
		void jump(addr_t a, guard_ptr g = guard_ptr(new guard()))
		{
			jump(constant(a),g);
		};

		// in
		addr_t address;
		::std::vector<token> tokens;
		::std::map< ::std::string,unsigned int> capture_groups;
		
		// out
		::std::set<bblock_ptr> basic_blocks;
		
	private:
		bblock_ptr last;
		addr_t next_address;
	};

	template<typename Tag>
	class rule
	{ 
	public: 
		typedef typename architecture_traits<Tag>::token_type token;
		typedef typename ::std::vector<typename architecture_traits<Tag>::token_type>::iterator tokiter;

		// returns pair<is successful?,next token to consume>
		virtual std::pair<bool,tokiter> match(tokiter begin, tokiter end, sem_state<Tag> &state) const = 0;
	};

	template<typename Tag>
	class action : public rule<Tag>
	{
	public:
		action(::std::function<void(sem_state<Tag>&)> &f) : semantic_action(f) {};
		virtual ~action(void) {};

		// returns pair<is successful?,next token to consume>
		virtual ::std::pair<bool,typename rule<Tag>::tokiter> match(typename rule<Tag>::tokiter begin, typename rule<Tag>::tokiter end, sem_state<Tag> &state) const
		{
			if(this->semantic_action)
				semantic_action(state);
			return ::std::make_pair(true,begin);
		};

		::std::function<void(sem_state<Tag>&)> semantic_action;
	};

	template<typename Tag>
	class tokpat : public rule<Tag>
	{
	public: 	
		tokpat(typename rule<Tag>::token m, typename rule<Tag>::token pat, ::std::map< ::std::string,typename rule<Tag>::token> &cg) : mask(m), pattern(pat), capture_patterns(cg) {};

		virtual ::std::pair<bool,typename rule<Tag>::tokiter> match(typename rule<Tag>::tokiter begin, typename rule<Tag>::tokiter end, sem_state<Tag> &state) const
		{
			if(begin == end)
				return ::std::make_pair(false,begin);

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
						state.capture_groups.insert(::std::make_pair(cg_iter->first,res));
					++cg_iter;
				}

				state.tokens.push_back(t);

				return ::std::make_pair(true,next(begin));
			}
			else
				return ::std::make_pair(false,begin);
		};

	private:
		typename rule<Tag>::token mask;
		typename rule<Tag>::token pattern;
		::std::map< ::std::string,typename rule<Tag>::token> capture_patterns;
	};

	template<typename Tag>
	class disjunction : public rule<Tag>
	{
	public:
		virtual ::std::pair<bool,typename rule<Tag>::tokiter> match(typename rule<Tag>::tokiter begin, typename rule<Tag>::tokiter end, sem_state<Tag> &state) const
		{
			auto i = patterns.cbegin();
			typename rule<Tag>::tokiter j;

			while(i != patterns.cend())
			{
				rule<Tag> &r(**i++);
				::std::pair<bool,typename rule<Tag>::tokiter> ret = r.match(begin,end,state);

				if(ret.first)
					return ret;
			}

			return ::std::make_pair(false,begin);
		};
		
		void chain(rule<Tag> *r)
		{
			patterns.push_back(r);
		}
				
	private:
		::std::list<rule<Tag> *> patterns;
	};

	template<typename Tag>
	class conjunction : public rule<Tag>
	{
	public:
		conjunction(rule<Tag> *a, rule<Tag> *b) : first(a), second(b) { assert(a && b);	};

		virtual ::std::pair<bool,typename rule<Tag>::tokiter> match(typename rule<Tag>::tokiter begin, typename rule<Tag>::tokiter end, sem_state<Tag> &state) const
		{
			bool cont;
			typename rule<Tag>::tokiter i;

			tie(cont,i) = first->match(begin,end,state);

			if(cont)
				return second->match(i,end,state);
			else
				return ::std::make_pair(false,begin);
		};

	private:
		rule<Tag> *first, *second;
	};

	template<typename Tag>
	class disassembler : public disjunction<Tag>
	{
	public:
		disassembler(void) : current(0), failsafe(0) {};

		disassembler &operator=(::std::function<void(sem_state<Tag>&)> f)
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
		
		disassembler &operator|(typename rule<Tag>::token i)
		{
			::std::map< ::std::string,typename rule<Tag>::token> cgs;
			append(new tokpat<Tag>(~((typename rule<Tag>::token)0),i,cgs));
			return *this;
		}

		disassembler &operator|(const char *c)
		{
			typename rule<Tag>::token mask = 0, pattern = 0;
			int bit = sizeof(typename rule<Tag>::token) * 8 - 1;
			const char *p = c;
			::std::map< ::std::string,typename rule<Tag>::token> cgs;
			typename rule<Tag>::token *cg_mask = 0;
			::std::string cg_name;
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
							::std::cout << "invalid pattern at column " << (int)(p - c) << " '" << c << "'" << ::std::endl;
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
								cgs.insert(::std::pair< ::std::string,typename rule<Tag>::token>(cg_name,0));
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
							::std::cout << "invalid pattern at column " << (int)(p-c) << " '" << c << "'" << ::std::endl;
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
						::std::cout << "invalid pattern at column " << (int)(p-c) << " '" << c << "'" << ::std::endl;
						assert(false);
					}
				}
			}
			
			assert(bit == -1);
			append(new tokpat<Tag>(mask,pattern,cgs));
			return *this;
		}

		disassembler &operator|(disassembler<Tag> &dec)
		{
			append(&dec);
			return *this;
		}

		void append(rule<Tag> *r)
		{
			if(!current)
				current = r;
			else
				current = new conjunction<Tag>(current,r);
		}
		
		virtual ::std::pair<bool,typename rule<Tag>::tokiter> match(typename rule<Tag>::tokiter begin, typename rule<Tag>::tokiter end, sem_state<Tag> &state) const
		{
			::std::pair<bool,typename rule<Tag>::tokiter> ret = disjunction<Tag>::match(begin,end,state);

			if(!ret.first && failsafe)
			{
				state.tokens.push_back(*begin);
				return failsafe->match(next(begin),end,state);
			}
			else
				return ret;
		}

	private:
		rule<Tag> *current;
		action<Tag> *failsafe;
	};

	void merge(proc_ptr proc, bblock_ptr block, addr_t anc_addr, guard_ptr g);
	void insert_bblock(proc_ptr proc, bblock_ptr block);

	template<typename Tag>
	void disassemble_procedure(proc_ptr proc, const disassembler<Tag> &main, ::std::vector<typename rule<Tag>::token> tokens, addr_t start)
	{
		// target, source mnemonic, guard
		::std::set<addr_t> todo;

		todo.insert(start);

		while(!todo.empty())
		{
			addr_t cur_addr = *todo.begin();
			sem_state<Tag> state(cur_addr);
			bool ret;
			typename rule<Tag>::tokiter i = tokens.begin();
		
			todo.erase(todo.begin());

			if(cur_addr >= tokens.size())
			{
				::std::cout << "boundary err" << ::std::endl;
				continue;
			}

			advance(i,cur_addr);
			tie(ret,i) = main.match(i,tokens.end(),state);
			
			for_each(state.basic_blocks.begin(),state.basic_blocks.end(),[&](const bblock_ptr &p)
			{
				basic_block::out_iterator i,iend;
				if(p->mnemonics().size())
					extend(proc,p);	
			});

			for_each(proc->basic_blocks.begin(),proc->basic_blocks.end(),[&](const bblock_ptr &bb)
			{
				for_each(bb->outgoing().begin(),bb->outgoing().end(),[&](const ctrans &ct)
				{ 
					if(!ct.bblock && ct.value.is_constant()) 
						todo.insert(ct.value.constant().value());
				});
			});
		}

		// entry may have been split
		if(proc->entry)
		{
			if(!proc->entry->mnemonics().empty())
				proc->entry = find_bblock(proc,proc->entry->mnemonics().front().area.begin);
			assert(proc->entry);
			proc->name = "proc_" + ::std::to_string(proc->entry->area().begin);
		}
	}
}

#endif

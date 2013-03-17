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
#include <procedure.hh>

namespace po
{
	template<typename Tag>
	struct sem_state
	{
		typedef typename architecture_traits<Tag>::token_type token;
		typedef typename ::std::vector<typename architecture_traits<Tag>::token_type>::iterator tokiter;

		sem_state(addr_t a) : address(a), next_address(a) {}

		void mnemonic(size_t len, ::std::string n, ::std::string fmt = ::std::string(""), ::std::list<rvalue> ops = ::std::list<rvalue>(), ::std::function<void(code_generator<Tag>&)> fn = ::std::function<void(code_generator<Tag>&)>())
		{
			::std::list<instr> instrs;
			code_generator<Tag> cg(inserter(instrs,instrs.end()));

			if(fmt.empty())
				fmt = accumulate(ops.begin(),ops.end(),fmt,[](const ::std::string &acc, const rvalue &x) 
					{ return acc + (acc.empty() ? "{8}" : ", {8}"); });

			// generate instr list
			if(fn) 
				fn(cg);

			mnemonics.emplace_back(po::mnemonic(range<addr_t>(next_address,next_address + len),n,fmt,ops.begin(),ops.end(),instrs.begin(),instrs.end())); 
			next_address += len;
		}

		void mnemonic(size_t len, ::std::string n, ::std::string fmt, rvalue a, ::std::function<void(code_generator<Tag>&)> fn = ::std::function<void(code_generator<Tag>&)>())
		{
			::std::list<rvalue> lst({a});
			return this->mnemonic(len,n,fmt,lst,fn);
		}
		
		void mnemonic(size_t len, ::std::string n, ::std::string fmt, rvalue a, rvalue b, ::std::function<void(code_generator<Tag>&)> fn = ::std::function<void(code_generator<Tag>&)>())
		{
			return mnemonic(len,n,fmt,{a,b},fn);
		}

		void jump(rvalue a, guard_ptr g = guard_ptr(new guard()))
		{
			jumps.emplace_back(::std::make_pair(a,g));
		}
		
		void jump(addr_t a, guard_ptr g = guard_ptr(new guard()))
		{
			jump(constant(a),g);
		}

		// in
		addr_t address;
		::std::vector<token> tokens;
		::std::map< ::std::string,unsigned int> capture_groups;
		
		// out
		::std::list<po::mnemonic> mnemonics;
		::std::list<::std::pair<rvalue,guard_ptr>> jumps;
		
	private:
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
		assert(proc && start != naddr);

		::std::set<addr_t> todo;
		::std::map<addr_t,mnemonic> mnemonics;
		::std::multimap<addr_t,addr_t> source, destination;

		// copy exsisting mnemonics and jumps into tables. TODO: cache tables in proc
		for(const bblock_ptr bb: proc->basic_blocks)
		{
			assert(bb);

			for(const mnemonic &m: bb->mnemonics())
			{
				assert(m.area.size());
				mnemonics.insert(::std::make_pair(m.area.last(),m));
			}

			for(const ctrans &ct: bb->outgoing())
			{
				source.insert(::std::make_pair(bb->area().last(),ct.value.constant().value()));
				destination.insert(::std::make_pair(ct.value.constant().value(),bb->area().last()));
			}
		}

		todo.insert(start);

		while(!todo.empty())
		{
			addr_t cur_addr = *todo.begin();
			sem_state<Tag> state(cur_addr);
			bool ret;
			typename rule<Tag>::tokiter i = tokens.begin();
			auto j = mnemonics.lower_bound(cur_addr);
		
			todo.erase(todo.begin());

			if(cur_addr >= tokens.size())
			{
				::std::cout << "boundary err" << ::std::endl;
				assert(false);
			}

			if(j == mnemonics.end() || !j->second.area.includes(cur_addr))
			{
				advance(i,cur_addr);
				tie(ret,i) = main.match(i,tokens.end(),state);
			
				if(ret)
				{
					addr_t last = 0;

					for(const mnemonic &m: state.mnemonics)
					{
						last = ::std::max(last,m.area.last());
						assert(mnemonics.insert(::std::make_pair(m.area.begin,m)).second);
					}
							
					for(const ::std::pair<rvalue,guard_ptr> &p: state.jumps)
					{
						if(p.first.is_constant())
						{
							addr_t target = p.first.constant().value();

							source.insert(::std::make_pair(last,target));
							destination.insert(::std::make_pair(target,last));
							todo.insert(target);
						}
						else
						{
							source.insert(::std::make_pair(last,naddr));
						}
					}
				}
				else
				{
					::std::cerr << "Failed to match anything at " << cur_addr << ::std::endl;
				}
			}
			else
			{
				if(j->first != cur_addr)
				{
					::std::cerr << "Overlapping mnemonics at " << cur_addr << " with \"" << "[" << j->second.area << "] " << j->second << "\"" << ::std::endl;
				}
			}

			
			/*
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
			});*/
		}
		
		::std::cout << "------ new basic block ------" << ::std::endl;

		auto cur_mne = mnemonics.begin(), first_mne = cur_mne;
		::std::map<addr_t,bblock_ptr> bblocks;
		::std::function<void(::std::map<addr_t,mnemonic>::iterator,::std::map<addr_t,mnemonic>::iterator)> make_bblock;
		make_bblock = [&](::std::map<addr_t,mnemonic>::iterator begin,::std::map<addr_t,mnemonic>::iterator end)
		{
			bblock_ptr bb(new basic_block());

			// copy mnemonics
			bb->mutate_mnemonics([&](::std::vector<mnemonic> &ms) 
			{
				::std::for_each(begin,end,[&](const ::std::pair<addr_t,mnemonic> &p)
				{ 
					ms.push_back(p.second); 
				}); 
			});

			proc->basic_blocks.insert(bb);
			assert(bblocks.insert(::std::make_pair(bb->area().last(),bb)).second);
		};

		while(cur_mne != mnemonics.end())
		{
			auto next_mne = ::std::next(cur_mne);
			const mnemonic &mne = cur_mne->second;
			addr_t div = mne.area.end;
			auto sources = source.equal_range(mne.area.last());
			auto destinations = destination.equal_range(div);
			
			::std::cout << mne.area << ": " << mne << ::std::endl;

			if(next_mne != mnemonics.end() && mne.area.size())
			{
				bool new_bb;

				// if next mnemonic is adjacent
				new_bb = next_mne->first != div;

				// or any following jumps aren't to adjacent mnemonics
				new_bb |= ::std::any_of(sources.first,sources.second,[&](const ::std::pair<addr_t,addr_t> &p) 
				{ 
					return p.second != div; 
				});
				
				// or any jumps pointing to the next that aren't from here
				new_bb |= ::std::any_of(destinations.first,destinations.second,[&](const ::std::pair<addr_t,addr_t> &p) 
				{ 
					return p.second != mne.area.last();
				});
			
				// construct a new basic block
				if(new_bb)
				{
					make_bblock(first_mne,next_mne);
					
					first_mne = next_mne;
					::std::cout << "------ new basic block ------" << ::std::endl;
				}
				else
				{
					while(sources.first != sources.second)
						source.erase(sources.first++);
					while(destinations.first != destinations.second)
						destination.erase(destinations.first++);
				}
			}

			cur_mne = next_mne;
		}
	
		// last bblock
		make_bblock(first_mne,cur_mne);
				
		// connect basic blocks
		for(const ::std::pair<addr_t,addr_t> &p: source)
		{
			if(p.second != naddr)
			{
				auto from = bblocks.find(p.first), to = bblocks.lower_bound(p.second);

				::std::cout << p.first << " to " << p.second << ::std::endl;

				assert(from != bblocks.end());
				assert(to != bblocks.end());
				assert(to->second->area().begin == p.second);
				unconditional_jump(from->second,to->second);
			}
		}

		// pack into bb

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

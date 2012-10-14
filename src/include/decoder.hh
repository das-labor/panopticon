#ifndef DECODER_HH
#define DECODER_HH

#include <functional>
#include <list>
#include <map>
#include <string>
#include <iostream>
#include <cassert>
#include <vector>

#include "architecture.hh"
#include "code_generator.hh"
#include "mnemonic.hh"
#include "basic_block.hh"
#include "procedure.hh"

template<typename Tag>
struct sem_state
{
	typedef typename architecture_traits<Tag>::token_type token;
	typedef typename std::vector<typename architecture_traits<Tag>::token_type>::iterator tokiter;

	sem_state(addr_t a) : address(a), next_address(a) {};

	mne_ptr mnemonic(size_t len, string n, list<value_ptr> ops = list<value_ptr>(), std::function<void(code_generator<Tag>&)> fn = std::function<void(code_generator<Tag>&)>())
	{
		assert(len);

		bblock_ptr new_bb, adj;
		mne_ptr m(new ::mnemonic(area(next_address,next_address + len),n,ops));
		std::list<instr_ptr> instr;
		code_generator<Tag> cg(inserter(instr,instr.end()));

		// generate instr_ptr list
		if(fn) fn(cg);

		cout << "cg: " << instr.size() << endl;
		last = bblock_ptr(new basic_block());
		last->append_mnemonic(m,make_pair(instr.begin(),instr.end()));
		basic_blocks.insert(last);
	
		next_address += len;
		return m;
	};

	mne_ptr mnemonic(size_t len, string n, valproxy a, std::function<void(code_generator<Tag>&)> fn = std::function<void(code_generator<Tag>&)>())
	{
		list<value_ptr> lst({a.value});
		return this->mnemonic(len,n,lst,fn);
	}
	
	mne_ptr mnemonic(size_t len, string n, valproxy a, valproxy b, std::function<void(code_generator<Tag>&)> fn = std::function<void(code_generator<Tag>&)>())
	{
		return mnemonic(len,n,{a.value,b.value},fn);
	}

	void jump(valproxy a, guard_ptr g = guard_ptr(new guard()))
	{
		assert(last && !basic_blocks.empty());
		last->insert_outgoing(g,a.value);
	};

	// in
	addr_t address;
	vector<token> tokens;
	map<string,unsigned int> capture_groups;
	
	// out
	set<bblock_ptr> basic_blocks;
	
private:
	bblock_ptr last;
	addr_t next_address;
};

template<typename Tag>
class rule
{ 
public: 
	typedef typename architecture_traits<Tag>::token_type token;
	typedef typename std::vector<typename architecture_traits<Tag>::token_type>::iterator tokiter;

	// returns pair<is successful?,next token to consume>
	virtual pair<bool,tokiter> match(tokiter begin, tokiter end, sem_state<Tag> &state) const = 0;
};

template<typename Tag>
class action : public rule<Tag>
{
public:
	action(function<void(sem_state<Tag>&)> &f) : semantic_action(f) {};

	// returns pair<is successful?,next token to consume>
	virtual pair<bool,typename rule<Tag>::tokiter> match(typename rule<Tag>::tokiter begin, typename rule<Tag>::tokiter end, sem_state<Tag> &state) const
	{
		if(this->semantic_action)
			semantic_action(state);
		return make_pair(true,begin);
	};

	function<void(sem_state<Tag>&)> semantic_action;
};

template<typename Tag>
class tokpat : public rule<Tag>
{
public: 	
	tokpat(typename rule<Tag>::token m, typename rule<Tag>::token pat, map<string,typename rule<Tag>::token> &cg) : mask(m), pattern(pat), capture_patterns(cg) {};

	virtual pair<bool,typename rule<Tag>::tokiter> match(typename rule<Tag>::tokiter begin, typename rule<Tag>::tokiter end, sem_state<Tag> &state) const
	{
		if(begin == end)
			return make_pair(false,begin);

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
					state.capture_groups.insert(make_pair(cg_iter->first,res));
				++cg_iter;
			}

			state.tokens.push_back(t);

			return make_pair(true,next(begin));
		}
		else
			return make_pair(false,begin);
	};

private:
	typename rule<Tag>::token mask;
	typename rule<Tag>::token pattern;
	map<string,typename rule<Tag>::token> capture_patterns;
};

template<typename Tag>
class disjunction : public rule<Tag>
{
public:
	virtual pair<bool,typename rule<Tag>::tokiter> match(typename rule<Tag>::tokiter begin, typename rule<Tag>::tokiter end, sem_state<Tag> &state) const
	{
		auto i = patterns.cbegin();
		typename rule<Tag>::tokiter j;

		while(i != patterns.cend())
		{
			rule<Tag> &r(**i++);
			pair<bool,typename rule<Tag>::tokiter> ret = r.match(begin,end,state);

			if(ret.first)
				return ret;
		}

		return make_pair(false,begin);
	};
	
	void chain(rule<Tag> *r)
	{
		patterns.push_back(r);
	}
			
private:
	list<rule<Tag> *> patterns;
};

template<typename Tag>
class conjunction : public rule<Tag>
{
public:
	conjunction(rule<Tag> *a, rule<Tag> *b) : first(a), second(b) { assert(a && b);	};

	virtual pair<bool,typename rule<Tag>::tokiter> match(typename rule<Tag>::tokiter begin, typename rule<Tag>::tokiter end, sem_state<Tag> &state) const
	{
		bool cont;
		typename rule<Tag>::tokiter i;

		tie(cont,i) = first->match(begin,end,state);

		if(cont)
			return second->match(i,end,state);
		else
			return make_pair(false,begin);
	};

private:
	rule<Tag> *first, *second;
};

template<typename Tag>
class decoder : public disjunction<Tag>
{
public:
	decoder(void) : current(0), failsafe(0) {};

	decoder &operator=(function<void(sem_state<Tag>&)> f)
	{
		if(current)
		{
			chain(new conjunction<Tag>(current,new action<Tag>(f)));
			current = 0;
		}
		else
		{
			if(failsafe)
				delete failsafe;
			failsafe = new action<Tag>(f);
		}
			
		return *this;
	}
	
	decoder &operator|(typename rule<Tag>::token i)
	{
		map<string,typename rule<Tag>::token> cgs;
		append(new tokpat<Tag>(~((typename rule<Tag>::token)0),i,cgs));
		return *this;
	}

	decoder &operator|(const char *c)
	{
		typename rule<Tag>::token mask = 0, pattern = 0;
		int bit = sizeof(typename rule<Tag>::token) * 8 - 1;
		const char *p = c;
		map<string,typename rule<Tag>::token> cgs;
		typename rule<Tag>::token *cg_mask = 0;
		string cg_name;
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
						cout << "invalid pattern at column " << (int)(p - c) << " '" << c << "'" << endl;
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
							cgs.insert(pair<string,typename rule<Tag>::token>(cg_name,0));
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
						cout << "invalid pattern at column " << (int)(p-c) << " '" << c << "'" << endl;
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
					cout << "invalid pattern at column " << (int)(p-c) << " '" << c << "'" << endl;
					assert(false);
				}
			}
		}
		
		assert(bit == -1);
		append(new tokpat<Tag>(mask,pattern,cgs));
		return *this;
	}

	decoder &operator|(decoder<Tag> &dec)
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
	
	virtual pair<bool,typename rule<Tag>::tokiter> match(typename rule<Tag>::tokiter begin, typename rule<Tag>::tokiter end, sem_state<Tag> &state) const
	{
		pair<bool,typename rule<Tag>::tokiter> ret = disjunction<Tag>::match(begin,end,state);

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
void disassemble_procedure(proc_ptr proc, const decoder<Tag> &main, vector<typename rule<Tag>::token> tokens, addr_t start)
{
	// target, source mnemonic, guard
	set<addr_t> todo;

	todo.insert(start);

	while(!todo.empty())
	{
		addr_t cur_addr = *todo.begin();
		sem_state<Tag> state(cur_addr);
		bool ret;
		typename rule<Tag>::tokiter i = tokens.begin();
	
		todo.erase(todo.begin());

		if(cur_addr >= tokens.size())
			continue;

		advance(i,cur_addr);
		tie(ret,i) = main.match(i,tokens.end(),state);
		
		for_each(state.basic_blocks.begin(),state.basic_blocks.end(),[&](const bblock_ptr &p)
		{
			basic_block::out_iterator i,iend;
			
			if(p->addresses().size())
				extend(proc,p);	
		});

		procedure::iterator j,jend;

		tie(j,jend) = proc->all();
		for_each(j,jend,[&](const bblock_ptr &bb)
		{
			basic_block::out_iterator i,iend;
			
			tie(i,iend) = bb->outgoing();
			for_each(i,iend,[&](const ctrans &ct)
			{ 
				if(!ct.bblock && ct.constant()) 
				{
					cout << "#new target: " << ct.constant()->val << endl;
					todo.insert(ct.constant()->val);
				}
			});
		});
		/*
		j = state.mnemonics.begin();
		jend = state.mnemonics.end();
		

		while(j != jend)
		{			
			pair<mne_ptr,vector<instr_ptr>> &p = *j++;
			list<pair<addr_t,guard_ptr>> ct;
			bool prev_known, nil;
			bblock_ptr cur_bb;
			//bblock_ptr prev_bb = prev_mne ? find_bblock(proc,prev_mne->addresses.last()) : entry;
	

			
	//		cout << "[mne] " << p.first->opcode << " (" << p.first->addresses << "); next: ";
			// XXX
			assert(prev_bb);
			for_each(state.direct_jumps.begin(),state.direct_jumps.end(),[&](pair<const mne_ptr,pair<addr_t,guard_ptr>> q)
			{ 
				if(q.first == p.first)
					ct.push_back(q.second);
			});

			// insert mnemonic into procedure, add (un)conditional control flow edge
			tie(prev_known,cur_bb) = extend_procedure(proc,p.first,prev_mne,prev_bb,prev_guard);
			
			// add unresolved indirect jumps. may cause to `cur_bb' basic block to be split
			for_each(state.indirect_jumps.begin(),state.indirect_jumps.end(),[&](pair<const mne_ptr,value_ptr> q)
			{
				if(q.first == p.first)
					tie(nil,cur_bb) = extend_procedure(proc,p.first,cur_bb,q.second,guard_ptr(new guard()));
			});
				
			// next addresses to disassemble
			if(!prev_known && !ct.empty())
				for_each(ct.begin(),ct.end(),[&](pair<addr_t,guard_ptr> q)
					{ todo.push_back(make_tuple(q.first,p.first,q.second)); });
			
			prev_mne = p.first;
			prev_guard = 
		}*/
	}

	// entry may have been split
	if(!proc->entry)
		proc->entry = *proc->all().first;
	else if(!proc->entry->mnemonics().empty())
		proc->entry = find_bblock(proc,proc->entry->mnemonics().front()->addresses.begin);
	assert(proc->entry);
	cout << "entry " << proc->entry->instructions().size() << endl;
	proc->name = "proc_" + to_string(proc->entry->addresses().begin);
	
	cout << graphviz(proc);
};

#endif

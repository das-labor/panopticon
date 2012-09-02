#ifndef DECODER_HH
#define DECODER_HH

#include <functional>
#include <list>
#include <map>
#include <string>
#include <iostream>
#include <cassert>

#include "procedure.hh"

using namespace std;

template<typename token,typename tokiter>
struct sem_state
{
	sem_state(void) : address(0) {};
	sem_state(addr_t a, tokiter begin, tokiter end) : address(a)
	{
		copy(begin,end,inserter(tokens,tokens.begin()));
	};

	mne_ptr add_mnemonic(area a, string n)
	{
		mne_ptr m(new mnemonic(a,n));
		
		mnemonics.insert(make_pair(a.begin,m));
		return m;
	};
	
	mne_ptr add_mnemonic(area a, string n, valproxy v1)
	{
		mne_ptr m(new mnemonic(a,n,{v1.value}));
		
		mnemonics.insert(make_pair(a.begin,m));
		return m;
	};
	
	mne_ptr add_mnemonic(area a, string n, valproxy v1, valproxy v2)
	{
		mne_ptr m(new mnemonic(a,n,{v1.value,v2.value}));
		
		mnemonics.insert(make_pair(a.begin,m));
		return m;
	};

	void unconditional(mne_ptr m, addr_t a) 
	{ 
		control_transfers.insert(make_pair(m,make_pair(a,guard_ptr(new guard()))));
	};

	// in
	addr_t address;
	vector<token> tokens;
	map<string,unsigned int> capture_groups;
	
	// out
	map<addr_t,mne_ptr> mnemonics;
	multimap<mne_ptr,pair<addr_t,guard_ptr>> control_transfers;
};

template<typename token,typename tokiter>
class rule
{ 
public: 
	// returns pair<is successful?,next token to consume>
	virtual pair<bool,tokiter> match(tokiter begin, tokiter end, sem_state<token,tokiter> &state) const = 0;
};

template<typename token,typename tokiter>
class action : public rule<token,tokiter>
{
public:
	action(function<void(sem_state<token,tokiter>&)> &f) : semantic_action(f) {};

	// returns pair<is successful?,next token to consume>
	virtual pair<bool,tokiter> match(tokiter begin, tokiter end, sem_state<token,tokiter> &state) const
	{
		if(this->semantic_action)
			semantic_action(state);
		return make_pair(true,begin);
	};

	function<void(sem_state<token,tokiter>&)> semantic_action;
};

template<typename token,typename tokiter>
class tokpat : public rule<token,tokiter>
{
public: 	
	tokpat(token m, token pat, map<string,token> &cg) : mask(m), pattern(pat), capture_patterns(cg) {};

	virtual pair<bool,tokiter> match(tokiter begin, tokiter end, sem_state<token,tokiter> &state) const
	{
		if(begin == end)
			return make_pair(false,begin);

		token t = *begin;

		if((t & mask) == pattern)
		{
			auto cg_iter = capture_patterns.cbegin();

			while(cg_iter != capture_patterns.cend())
			{
				token mask = cg_iter->second;
				unsigned int res = 0;
				int bit = sizeof(token) * 8 - 1;

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
	token mask;
	token pattern;
	map<string,token> capture_patterns;
};

template<typename token,typename tokiter>
class disjunction : public rule<token,tokiter>
{
public:
	virtual pair<bool,tokiter> match(tokiter begin, tokiter end, sem_state<token,tokiter> &state) const
	{
		auto i = patterns.cbegin();
		tokiter j;

		while(i != patterns.cend())
		{
			rule<token,tokiter> &r(**i++);
			pair<bool,tokiter> ret = r.match(begin,end,state);

			if(ret.first)
				return ret;
		}

		return make_pair(false,begin);
	};
	
	void chain(rule<token,tokiter> *r)
	{
		patterns.push_back(r);
	}
			
private:
	list<rule<token,tokiter> *> patterns;
};

template<typename token,typename tokiter>
class conjunction : public rule<token,tokiter>
{
public:
	conjunction(rule<token,tokiter> *a, rule<token,tokiter> *b) : first(a), second(b) { assert(a && b);	};

	virtual pair<bool,tokiter> match(tokiter begin, tokiter end, sem_state<token,tokiter> &state) const
	{
		bool cont;
		tokiter i;

		tie(cont,i) = first->match(begin,end,state);

		if(cont)
			return second->match(i,end,state);
		else
			return make_pair(false,begin);
	};

private:
	rule<token,tokiter> *first, *second;
};

template<typename token,typename tokiter>
class decoder : public disjunction<token,tokiter>
{
public:
	decoder(void) : current(0), failsafe(0) {};

	decoder &operator=(function<void(sem_state<token,tokiter>&)> f)
	{
		if(current)
		{
			chain(new conjunction<token,tokiter>(current,new action<token,tokiter>(f)));
			current = 0;
		}
		else
		{
			if(failsafe)
				delete failsafe;
			failsafe = new action<token,tokiter>(f);
		}
			
		return *this;
	}
	
	decoder &operator|(token i)
	{
		map<string,token> cgs;
		append(new tokpat<token,tokiter>(~((token)0),i,cgs));
		return *this;
	}

	decoder &operator|(const char *c)
	{
		token mask = 0, pattern = 0;
		int bit = sizeof(token) * 8 - 1;
		const char *p = c;
		map<string,token> cgs;
		token *cg_mask = 0;
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
							cgs.insert(pair<string,token>(cg_name,0));
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
		append(new tokpat<token,tokiter>(mask,pattern,cgs));
		return *this;
	}

	decoder &operator|(decoder<token,tokiter> &dec)
	{
		append(&dec);
		return *this;
	}

	void append(rule<token,tokiter> *r)
	{
		if(!current)
			current = r;
		else
			current = new conjunction<token,tokiter>(current,r);
	}
	
	virtual pair<bool,tokiter> match(tokiter begin, tokiter end, sem_state<token,tokiter> &state) const
	{
		pair<bool,tokiter> ret = disjunction<token,tokiter>::match(begin,end,state);

		if(!ret.first && failsafe)
		{
			state.tokens.push_back(*begin);
			return failsafe->match(next(begin),end,state);
		}
		else
			return ret;
	}

private:
	rule<token,tokiter> *current;
	action<token,tokiter> *failsafe;
};

template<typename token,typename tokiter>
proc_ptr disassemble_procedure(const decoder<token,tokiter> &main, vector<token> tokens, addr_t offset = 0, bool cf_sensitive = true)
{
	proc_ptr proc(new procedure());
	list<tuple<addr_t,mne_cptr,bblock_ptr>> todo;
	bblock_ptr entry(new basic_block());

	proc->insert_bblock(entry);
	proc->entry = entry;
	todo.emplace_back(make_tuple(0,mne_cptr(0),entry));

	while(!todo.empty())
	{
		sem_state<token,tokiter> state;
		tuple<addr_t,mne_cptr,bblock_ptr> subject = todo.back();
		bool ret;
		tokiter i = tokens.begin();

		todo.pop_back();

		if(get<0>(subject) >= tokens.size())
			continue;

		advance(i,get<0>(subject));

		state.address = get<0>(subject);
		tie(ret,i) = main.match(i,tokens.end(),state);
		
		cout << "disassemble at addr " << hex << get<0>(subject) << dec << endl;
		
		for_each(state.mnemonics.begin(),state.mnemonics.end(),[&](pair<addr_t,mne_ptr> p)
		{
			list<pair<addr_t,guard_ptr>> ct;
			bool prev_known;
			bblock_ptr prev_bb;

			cout << p.second->name << endl;

			for_each(state.control_transfers.begin(),state.control_transfers.end(),[&](pair<const mne_ptr,pair<addr_t,guard_ptr>> q)
			{ 
				if(q.first == p.second)
					ct.push_back(q.second);
			});

			// if mnemonic is right after the previous bb, add it 
			tie(prev_known,prev_bb) = extend_procedure(proc,p.second,get<1>(subject),get<2>(subject));
				
			/* if procedure call, recrusive decode call, remember call for call graph
			if(next.is_call)
			{
				proc->calls.insert(next.call_address);
				typename list<procedure<tok_t>>::iterator f = find_if(cfg.begin(),cfg.end(),[&next](const procedure<tok_t> &p)
					{ return p.entry_point == next.call_address; });
					
				if(f == cfg.end())
					decode_procedure(bytes,next.call_address,cfg);
			}*/

			// next addresses to disassemble
			if(!prev_known && !ct.empty())
				for_each(ct.begin(),ct.end(),[&](pair<addr_t,guard_ptr> q)
					{ todo.push_back(make_tuple(q.first,p.second,prev_bb)); });
		});
	}

	return proc;
};

#endif

#include <functional>
#include <list>
#include <map>
#include <set>
#include <string>
#include <iostream>
#include <cassert>
#include <fstream>
#include <vector>
#include <algorithm>

#include <avr/avr.hh>
#include <code_generator.hh>
#include <mnemonic.hh>
#include <basic_block.hh>
#include <procedure.hh>
#include <flowgraph.hh>

using namespace std;
using namespace po;

void dis(const string &path)
{
	ifstream f(path);
	vector<uint16_t> bytes;

	if (f.bad() || f.fail())
	{
		cerr << path << ": I/O error while reading" << endl;
		return flow_ptr();
	}
	else 
	{
		while(f.good() && !f.eof())
		{
			uint16_t c;
			f.read((char *)&c,sizeof(c));
			bytes.push_back(c);
		}
	}
	
	set<addr_t> todo;
	avr::disassembler main;

	todo.insert(0);

	while(!todo.empty())
	{
		addr_t cur_addr = *todo.begin();
		sem_state<avr_tag> state(cur_addr);
		bool ret;
		typename rule<avr_tag>::tokiter i = tokens.begin();
	
		todo.erase(todo.begin());

		if(cur_addr >= tokens.size())
		{
			::std::cout << "boundary err" << ::std::endl;
			assert(false);
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


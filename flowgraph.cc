#include <sstream>
#include <algorithm>

#include "flowgraph.hh"
#include "basic_block.hh"
#include "mnemonic.hh"

proc_ptr find_procedure(flow_ptr fg, addr_t a)
{
	set<proc_ptr>::iterator i = fg->procedures.begin();
	
	while(i != fg->procedures.end())
		if(find_bblock(*i,a))
			return *i;
		else 
			++i;
	
	return proc_ptr(0);
}

string graphviz(flow_ptr fg)
{
	stringstream ss;

	ss << "digraph G" << endl
		 << "{" << endl
		 << "\tnode [shape=record,style=filled,color=lightgray,fontsize=13,fontname=\"Monospace\"];" << endl;
	
	// procedures
	for_each(fg->procedures.begin(),fg->procedures.end(),[&ss](const proc_ptr proc)
	{
		assert(proc && proc->entry);
		procedure::iterator i,iend;
		string procname(to_string(proc->entry->addresses().begin));

		ss << "\tsubgraph cluster_" << procname << endl
			 << "\t{" << endl
			 << "\t\tlabel = \"procedure at 0x" << hex << procname << dec << "\";" << endl
			 << "\t\tcolor = black;" << endl
			 << "\t\tfontname = \"Monospace\";" << endl;

		// basic blocks
		tie(i,iend) = proc->all();
		for_each(i,iend,[&proc,&ss,&procname](const bblock_ptr bb)
		{
			basic_block::iterator j,jend;

			ss << "\t\tbb_" << procname << "_" << bb->addresses().begin << " [label=\"";
			
			// mnemonics
			tie(j,jend) = bb->mnemonics();
			for_each(j,jend,[&ss](const mne_cptr m)
				{ ss << "0x" << hex << m->addresses.begin << dec << ": " << m->inspect() << "\\l"; });
			ss << "\"];" << endl;

			basic_block::out_iterator k,kend;

			// outgoing edges
			tie(k,kend) = bb->outgoing();
			for_each(k,kend,[&bb,&ss,&procname](const pair<guard_cptr,bblock_ptr> s) 
			{ 
				ss << "\t\tbb_" << procname << "_" << bb->addresses().begin 
					 << " -> bb_" << procname << "_" << s.second->addresses().begin
					 << " [label=\" " << s.first->inspect() << " \"];" << endl; 
			});	
			
			/*basic_block::in_iterator l,lend;

			// incoming edges
			tie(l,lend) = bb->incoming();
			for_each(l,lend,[&bb](const pair<guard_cptr,bblock_ptr> s) 
			{ 
				ss << "\t\tbb_" << procname << "_" << bb->addresses().begin 
					 << " -> bb_" << procname << "_" << s.second->addresses().begin 
					 << " [arrowhead=\"crow\",label=\"" << s.first.get() << "\"];" << endl; 
			});*/

		});
		
		ss << "\t}"  << endl;
	});

	/*
	// callgraph
	ss << " subgraph cluster_calls" << endl
		 << " {" << endl
		 << " node [shape=circle,fontsize=15,fontname=\"Monospace\"];" << endl
		 << "  func_" << proc.entry_point << dec << " [label=\"" << proc.name << "\"];" << endl;
	for_each(proc.calls.begin(),proc.calls.end(),[&proc](const addr_t a) 
		{ ss << "   func_" << proc.entry_point << " -> func_" << a << ";" << endl; });
		ss << " }" << endl;*/
	ss << "}" << endl;
	
	return ss.str();
}	

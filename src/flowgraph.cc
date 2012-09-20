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

bool has_procedure(flow_ptr flow, addr_t entry)
{
	return any_of(flow->procedures.begin(),flow->procedures.end(),[&](const proc_ptr p) 
								{ return p->entry->addresses().includes(entry); });
}

string graphviz(flow_ptr fg)
{
	stringstream ss;

	ss << "digraph G" << endl
		 << "{" << endl
		 << "\tnode [shape=record,style=filled,color=lightgray,fontsize=13,fontname=\"Monospace\"];" << endl;
	
	// procedures
	for_each(fg->procedures.begin(),fg->procedures.end(),[&](const proc_ptr proc)
	{
		assert(proc && proc->entry);
		procedure::iterator i,iend;
		string procname(to_string(proc->entry->addresses().begin));
		shared_ptr<map<bblock_ptr,taint_lattice>> taint_bblock(fg->taint[proc]);
		shared_ptr<map<bblock_ptr,cprop_lattice>> cprop_bblock(fg->cprop[proc]);

		ss << "\tsubgraph cluster_" << procname << endl
			 << "\t{" << endl
			 << "\t\tlabel = \"procedure at " << procname << "\";" << endl
			 << "\t\tcolor = black;" << endl
			 << "\t\tfontname = \"Monospace\";" << endl;

		// basic blocks
		tie(i,iend) = proc->all();
		for_each(i,iend,[&](const bblock_ptr bb)
		{
			size_t sz = bb->mnemonics().size(), pos = 0;
			const mne_cptr *j = bb->mnemonics().data();
			//taint_lattice tl(taint_bblock->at(bb));
			cprop_lattice cp(cprop_bblock->at(bb));

			ss << "\t\tbb_" << procname 
				 << "_" << bb->addresses().begin 
				 << " [label=<<table BORDER=\"0\" CELLBORDER=\"0\" CELLSPACING=\"0\" ALIGN=\"LEFT\">";
		
			// PHI nodes
			{ 
				mnemonic::iterator l = bb->instructions().begin();
				
				if(!bb->instructions().empty() && (*l)->opcode == instr::Phi)
				{
					ss << "<tr ALIGN=\"LEFT\"><td ALIGN=\"LEFT\">0x" 
						 << hex << bb->addresses().begin << dec 
						 << "</td><td ALIGN=\"LEFT\"> </td></tr>"
						 << "<tr><td COLSPAN=\"2\"><table BORDER=\"0\" CELLBORDER=\"0\" CELLSPACING=\"0\" ALIGN=\"LEFT\">";
					
					while(l != bb->instructions().end() && (*l)->opcode == instr::Phi)
					{
						instr_cptr in = *l++;
						ss << "<tr ALIGN=\"LEFT\"><td ALIGN=\"LEFT\">" 
							 << "<font POINT-SIZE=\"11\">" << in->inspect()
							 << "</font></td></tr>";
					}
					ss << "</table></td></tr>";
				}
			}
	
			// mnemonics
			while(pos < sz)
			{
				const mne_cptr m = j[pos++];
				mnemonic::iterator l;

				ss << "<tr ALIGN=\"LEFT\"><td ALIGN=\"LEFT\">0x" 
					 << hex << m->addresses.begin << dec 
					 << "</td><td ALIGN=\"LEFT\">" << m->inspect()
					 << "</td></tr>";
				
				if(!m->instructions.empty())
				{
					ss << "<tr><td COLSPAN=\"2\"><table BORDER=\"0\" CELLBORDER=\"0\" CELLSPACING=\"0\" ALIGN=\"LEFT\">";
					l = m->instructions.begin();
					while(l != m->instructions.end())
					{
						instr_cptr in = *l++;
						ss << "<tr ALIGN=\"LEFT\"><td ALIGN=\"LEFT\">" 
							 << "<font POINT-SIZE=\"11\">" << in->inspect();
						
						/* taint
						if(tl->count(in->assigns->nam))
							ss << accumulate(tl->at(in->assigns->nam).begin(),
															 tl->at(in->assigns->nam).end(),
															 string(" ("),
															 [](const string &acc, const name &s) { return acc + (s.base.front() == 't' ? "" : " " + s.inspect()); })
								 << " )";
						else
							ss << " ( )";*/
						
						// constant prop
						if(cp->has(in->assigns->nam))
							ss << "(" << cp->get(in->assigns->nam) << ")";
						else
							ss << " ( )";

						ss << "</font>"
							 << "</td></tr>";
					}
					ss << "</table></td></tr>";
				}
			}

			ss << "</table>>];" << endl;

			basic_block::out_iterator k,kend;
			basic_block::indir_iterator m,mend;

			// outgoing edges
			tie(k,kend) = bb->outgoing();
			for_each(k,kend,[&bb,&ss,&procname](const pair<guard_ptr,bblock_ptr> s) 
			{ 
				ss << "\t\tbb_" << procname << "_" << bb->addresses().begin 
					 << " -> bb_" << procname << "_" << s.second->addresses().begin
					 << " [label=\" " << s.first->inspect() << " \"];" << endl; 
			});	
			
			// indirect jumps
			tie(m,mend) = bb->indirect();
			for_each(m,mend,[&bb,&ss,&procname](const pair<guard_ptr,value_ptr> s) 
			{ 
				ss << "\t\tbb_" << procname << "_indir" << s.second.get() 
					 << " [label=\"" << s.second->inspect() << "\"];" << endl
					 << "\t\tbb_" << procname << "_" << bb->addresses().begin 
					 << " -> bb_" << procname << "_indir" << s.second.get()
					 << " [label=\"" << s.first->inspect() << "\"];" << endl;
			});	

			/*basic_block::in_iterator l,lend;

			// incoming edges
			tie(l,lend) = bb->incoming();
			for_each(l,lend,[&bb](const pair<guard_ptr,bblock_ptr> s) 
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

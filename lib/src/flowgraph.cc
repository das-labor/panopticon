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
								{ return p->entry->area().includes(entry); });
}

string turtle(flow_ptr fg)
{
	stringstream ss;

	ss << "@prefix : <http://localhost/>." << endl;

	/*if(fg->taint.size())
		ss << ":approx_taint rdf:type po:Approximation;" << endl
			 << "\tpo:title \"Taint analysis\"^^xsd:string." << endl;
	
	if(fg->cprop.size())
		ss << ":approx_cprop rdf:type po:Approximation;" << endl
			 << "\tpo:title \"Constant propagation\"^^xsd:string." << endl;*/
	
	// procedures
	for_each(fg->procedures.begin(),fg->procedures.end(),[&](const proc_ptr proc)
	{
		assert(proc && proc->entry);
		
		procedure::iterator i;
		string procname(to_string(proc->entry->area().begin));
		stringstream ss_bblocks;
		shared_ptr<map<bblock_ptr,taint_lattice>> taint_bblock(fg->taint.count(proc) ? fg->taint[proc] : nullptr);
		shared_ptr<map<bblock_ptr,cprop_lattice>> cprop_bblock(fg->cprop.count(proc) ? fg->cprop[proc] : nullptr);

		ss << ":proc_" 	<< procname << " rdf:type po:Procedure;" << endl
																<< "\tpo:name \"" << proc->name << "\";" << endl
			 													<< "\tpo:entry_point :bblock_" << procname << "_" << to_string(proc->entry->area().begin) << "." << endl;

		for_each(proc->callees.begin(),proc->callees.end(),[&](const proc_ptr c) 
			{ ss << ":proc_" << procname << " po:calls :proc_" << to_string(c->entry->area().begin) << "." << endl; });

		// basic blocks
		i = proc->basic_blocks.begin();
		while(i != proc->basic_blocks.end())
		{
			bblock_ptr bb = *i++;
			assert(bb);
			
			basic_block::succ_iterator j,jend;
			size_t k = 0, kend = bb->mnemonics().size();
			const mne_cptr *kp = bb->mnemonics().data();
			size_t l = 0, lend = bb->instructions().size();
			const instr_ptr *lp = bb->instructions().data();
			string bbname = to_string(bb->area().begin);
			stringstream ss_mne;
			stringstream ss_instr;
			taint_lattice *taint = taint_bblock && taint_bblock->count(bb) ? &taint_bblock->at(bb) : 0;
			cprop_lattice *cprop = cprop_bblock && cprop_bblock->count(bb) ? &cprop_bblock->at(bb) : 0;

			ss << ":bblock_" << procname << "_" << bbname << " rdf:type po:BasicBlock;" << endl
																										<< "\tpo:begin \"" << bb->area().begin << "\"^^xsd:integer;" << endl
																										<< "\tpo:end \"" << bb->area().end << "\"^^xsd:integer." << endl;
			/* mnemonics
			while(k < kend)
			{
				string mnename = to_string(kp[k]->area.begin);
				stringstream ss_ops;

				for_each(kp[k]->operands.begin(),kp[k]->operands.end(),[&](const value_ptr v)
					{ ss_ops << "\"" << v->inspect() << "\"^^xsd:string "; });

				ss << ":mne_" << procname << "_" << bbname << "_" << mnename << " rdf:type po:Mnemonic;" << endl
																																		 << " po:opcode \"" << kp[k]->opcode << "\"^^xsd:string;" << endl
																																		 << " po:operands (" << ss_ops.str() << ")." << endl;
				ss_mne << " :mne_" << procname << "_" << bbname << "_" << mnename;

				++k;
			};
			ss << ":bblock_" << procname << "_" << bbname << " po:mnemonics (" << ss_mne.str() << ")." << endl;
			
			// instructions
			while(l < lend)
			{
				string instrname = to_string(l);
				stringstream ss_args;
				string instr_type = "po:Instruction";
			//	string asname = lp[l]->assigns->nam.base + "_" + (lp[l]->assigns->nam.subscript >= 0 ? to_string(lp[l]->assigns->nam.subscript) : "");

				switch(lp[l]->function)
				{
					case instr::Phi: instr_type = "po:Phi"; break;
					case instr::Not: instr_type = "po:Not"; break;
					case instr::And: instr_type = "po:And"; break;
					case instr::Or: instr_type = "po:Or"; break;
					case instr::Xor: instr_type = "po:Xor"; break;
					case instr::Assign: instr_type = "po:Assign"; break;
					case instr::ULeq: instr_type = "po:ULeq"; break;
					case instr::SLeq: instr_type = "po:SLeq"; break;
					case instr::UShr: instr_type = "po:UShr"; break;
					case instr::UShl: instr_type = "po:UShl"; break;
					case instr::SShr: instr_type = "po:SShr"; break;
					case instr::SShl: instr_type = "po:SShl"; break;
					case instr::SExt: instr_type = "po:SExt"; break;
					case instr::UExt: instr_type = "po:UExt"; break;
					case instr::Slice: instr_type = "po:Slice"; break;
					case instr::Concat: instr_type = "po:Concat"; break;
					case instr::Add: instr_type = "po:Add"; break;
					case instr::Sub: instr_type = "po:Sub"; break;
					case instr::Mul: instr_type = "po:Mul"; break;
					case instr::SDiv: instr_type = "po:SDiv"; break;
					case instr::UDiv: instr_type = "po:UDiv"; break;
					case instr::SMod: instr_type = "po:SMod"; break;
					case instr::UMod: instr_type = "po:UMod"; break;
					case instr::Call: instr_type = "po:Call"; break;
					default: ;
				}
				
				for_each(lp[l]->arguments.begin(),lp[l]->arguments.end(),[&](value_ptr v)
				{ 
					var_ptr w;
					shared_ptr<constant> c;

					if((w = dynamic_pointer_cast<variable>(v)))
					{
						string argname = procname + "_" + w->nam.base + "_" + (w->nam.subscript >= 0 ? to_string(w->nam.subscript) : "");
												
						ss << ":var_" << argname << " rdf:type po:Variable;" << endl 
							 << "\tpo:width \"0\";" << endl 
							 << "\tpo:subscript \"" << w->nam.subscript << "\";" << endl
							 << "\tpo:base \"" << w->nam.base << "\"." << endl;
						ss_args << ":var_" << argname << " ";
					}
					else if((c = dynamic_pointer_cast<constant>(v)))
					{
						ss << ":val_" << procname << "_" << c->val << " rdf:type po:Constant;" << endl 
							 << "\tpo:width \"0\";" << endl 
							 << "\tpo:value \"" << c->val << "\"." << endl;
						ss_args << ":val_" << procname << "_" << c->val << " ";
					}
					else
					{
						ss << ":null rdf:type po:Value; po:width \"0\".";
						ss_args << ":null ";
					}
				});
				
	/*			ss << ":var_" << procname << "_" << asname << " rdf:type po:Variable;" << endl	 
					 << "\tpo:width \"0\";" << endl 
					 << "\tpo:subscript \"" << lp[l]->assigns->nam.subscript << "\";" << endl
					 << "\tpo:base \"" << lp[l]->assigns->nam.base << "\"." << endl;

				/*ss << ":instr_" << procname << "_" << bbname << "_" << instrname << " rdf:type " << instr_type << ";" << endl
																																		 		 << " po:arguments (" << ss_args.str() << ");" << endl
																																				 << " po:assigns " << ":var_" << procname << "_" << asname << "." << endl;
				if(taint)
					ss << ":var_" << procname << "_" << asname << " po:approx " << "\"" << (*taint)->get(lp[l]->assigns->nam) << "\"" << "." << endl;
				if(cprop)
				{
					ss << ":approx_cprop po:defines :cporp_" << (*cprop)->get(lp[l]->assigns->nam) << "." << endl
						 << ":cprop_" << (*cprop)->get(lp[l]->assigns->nam) << " po:approximates :var_" << procname << "_" << asname << ";" << endl
						 << "\tpo:display \"" << (*cprop)->get(lp[l]->assigns->nam) << "\"^^xsd:string." << endl;
				}

				ss_instr << " :instr_" << procname << "_" << bbname << "_" << instrname;

				++l;
			}*/
			//ss << ":bblock_" << procname << "_" << bbname << " po:instructions (" << ss_instr.str() << ")." << endl;
			
			// successors
			tie(j,jend) = bb->successors();
			for_each(j,jend,[&](const bblock_ptr s)
			{ 
				ss << ":bblock_" << procname << "_" << bbname 
					 << " po:precedes :bblock_" << procname << "_" << to_string(s->area().begin) << "." << endl; 
			});

			
			ss_bblocks << ":bblock_" << procname << "_" << bbname << " ";
			if(i != proc->basic_blocks.end())
				ss_bblocks << ", ";
		}

		ss << ":proc_" 	<< procname << " po:contains " << ss_bblocks.str() << "." << endl;
	});

		/* basic blocks
		tie(i,iend) = proc->all();
		for_each(i,iend,[&](const bblock_ptr bb)
		{
			size_t sz = bb->mnemonics().size(), pos = 0;
			const mne_cptr *j = bb->mnemonics().data();
			//taint_lattice tl(taint_bblock->at(bb));
			cprop_lattice cp(cprop_bblock->at(bb));

			ss << "\t\tbb_" << procname 
				 << "_" << bb->area().begin 
				 << " [label=<<table BORDER=\"0\" CELLBORDER=\"0\" CELLSPACING=\"0\" ALIGN=\"LEFT\">";
		
			* PHI nodes
			{ 
				mnemonic::iterator l = bb->instructions().begin();
				
				if(!bb->instructions().empty() && (*l)->opcode == instr::Phi)
				{
					ss << "<tr ALIGN=\"LEFT\"><td ALIGN=\"LEFT\">0x" 
						 << hex << bb->area().begin << dec 
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
			}*
	
			* mnemonics
			while(pos < sz)
			{
				const mne_cptr m = j[pos++];
				mnemonic::iterator l;

				ss << "<tr ALIGN=\"LEFT\"><td ALIGN=\"LEFT\">0x" 
					 << hex << m->area.begin << dec 
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
						
						* taint
						if(tl->count(in->assigns->nam))
							ss << accumulate(tl->at(in->assigns->nam).begin(),
															 tl->at(in->assigns->nam).end(),
															 string(" ("),
															 [](const string &acc, const name &s) { return acc + (s.base.front() == 't' ? "" : " " + s.inspect()); })
								 << " )";
						else
							ss << " ( )";*
						
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
				ss << "\t\tbb_" << procname << "_" << bb->area().begin 
					 << " -> bb_" << procname << "_" << s.second->area().begin
					 << " [label=\" " << s.first->inspect() << " \"];" << endl; 
			});	
			
			// indirect jumps
			tie(m,mend) = bb->indirect();
			for_each(m,mend,[&bb,&ss,&procname](const pair<guard_ptr,value_ptr> s) 
			{ 
				ss << "\t\tbb_" << procname << "_indir" << s.second.get() 
					 << " [label=\"" << s.second->inspect() << "\"];" << endl
					 << "\t\tbb_" << procname << "_" << bb->area().begin 
					 << " -> bb_" << procname << "_indir" << s.second.get()
					 << " [label=\"" << s.first->inspect() << "\"];" << endl;
			});	

			*basic_block::in_iterator l,lend;

			// incoming edges
			tie(l,lend) = bb->incoming();
			for_each(l,lend,[&bb](const pair<guard_ptr,bblock_ptr> s) 
			{ 
				ss << "\t\tbb_" << procname << "_" << bb->area().begin 
					 << " -> bb_" << procname << "_" << s.second->area().begin 
					 << " [arrowhead=\"crow\",label=\"" << s.first.get() << "\"];" << endl; 
			});*

		});*
		
		ss << "\t}"  << endl;
	});*/

	
	return ss.str();
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
		string procname(to_string(proc->entry->area().begin));
		shared_ptr<map<bblock_ptr,taint_lattice>> taint_bblock(fg->taint[proc]);
		shared_ptr<map<bblock_ptr,cprop_lattice>> cprop_bblock(fg->cprop[proc]);

		ss << "\tsubgraph cluster_" << procname << endl
			 << "\t{" << endl
			 << "\t\tlabel = \"procedure at " << procname << "\";" << endl
			 << "\t\tcolor = black;" << endl
			 << "\t\tfontname = \"Monospace\";" << endl;

		// basic blocks
		for_each(proc->basic_blocks.begin(),proc->basic_blocks.end(),[&](const bblock_ptr bb)
		{
			size_t sz = bb->mnemonics().size(), pos = 0;
			const mne_cptr *j = bb->mnemonics().data();
			taint_lattice *tl = taint_bblock && taint_bblock->count(bb) ? &taint_bblock->at(bb) : 0;
			cprop_lattice *cp = cprop_bblock && cprop_bblock->count(bb) ? &cprop_bblock->at(bb) : 0;

			ss << "\t\tbb_" << procname 
				 << "_" << bb->area().begin;

			if(pos < sz)
				ss << " [label=<<table BORDER=\"0\" CELLBORDER=\"0\" CELLSPACING=\"0\" ALIGN=\"LEFT\">";
			else
				ss << " [label=\"" << bb->area().begin << "\"];" << endl;
		
			/* PHI nodes
			{ 
				mnemonic::iterator l = bb->instructions().begin();
				
				if(!bb->instructions().empty() && (*l)->opcode == instr::Phi)
				{
					ss << "<tr ALIGN=\"LEFT\"><td ALIGN=\"LEFT\">0x" 
						 << hex << bb->area().begin << dec 
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
			}*/
	
			// mnemonics
			while(pos < sz)
			{
				const mne_cptr m = j[pos++];
				mnemonic::iterator l;
				instr_citerator n,nend;

				tie(n,nend) = bb->instructions(m);
				ss << "<tr ALIGN=\"LEFT\"><td ALIGN=\"LEFT\">0x" 
					 << hex << m->area.begin << dec 
					 << "</td><td ALIGN=\"LEFT\">" << m->inspect()
					 << "</td></tr>";
				
				if(n != nend)
				{
					ss << "<tr><td COLSPAN=\"2\"><table BORDER=\"0\" CELLBORDER=\"0\" CELLSPACING=\"0\" ALIGN=\"LEFT\">";
					while(n != nend)
					{
						instr_cptr in = *n++;
						var_ptr v = dynamic_pointer_cast<variable>(in->assigns);

						ss << "<tr ALIGN=\"LEFT\"><td ALIGN=\"LEFT\">" 
							 << "<font POINT-SIZE=\"11\">" << in->inspect()
							 << "</font></td><td><font POINT-SIZE=\"11\">";

						// taint
						if(v && tl && (*tl)->has(v->nam))
							ss << accumulate((*tl)->get(v->nam).begin(),
															 (*tl)->get(v->nam).end(),
															 string(" ("),
															 [](const string &acc, const name &s) { return acc + (s.base.front() == 't' ? "" : " " + s.inspect()); })
								 << " )";
						else
							ss << " ( )";
						
						ss << "</font></td><td><font POINT-SIZE=\"11\">";
						
						// constant prop
						if(v && cp && (*cp)->has(v->nam))
							ss << "(" << (*cp)->get(v->nam) << ")";
						else
							ss << " ( )";

						ss << "</font></td></tr>";
					}
					ss << "</table></td></tr>";
				}
			}

			if(pos)
				ss << "</table>>];" << endl;

			basic_block::out_iterator k,kend;

			// outgoing edges
			tie(k,kend) = bb->outgoing();
			for_each(k,kend,[&bb,&ss,&procname](const ctrans &ct) 
			{ 
				if(ct.bblock)
				{
					ss << "\t\tbb_" << procname << "_" << bb->area().begin 
						 << " -> bb_" << procname << "_" << ct.bblock->area().begin
					 	 << " [label=\" " << ct.guard->inspect() << " \"];" << endl; 
				}
				else
				{
					ss << "\t\tbb_" << procname << "_indir" << ct.value.get() 
					 << " [shape = circle, label=\"" << ct.value->inspect() << "\"];" << endl
					 << "\t\tbb_" << procname << "_" << bb->area().begin 
					 << " -> bb_" << procname << "_indir" << ct.value.get()
					 << " [label=\" " << ct.guard->inspect() << " \"];" << endl;
				}		
			});	
			
			/*basic_block::in_iterator l,lend;

			// incoming edges
			tie(l,lend) = bb->incoming();
			for_each(l,lend,[&bb](const pair<guard_ptr,bblock_ptr> s) 
			{ 
				ss << "\t\tbb_" << procname << "_" << bb->area().begin 
					 << " -> bb_" << procname << "_" << s.second->area().begin 
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

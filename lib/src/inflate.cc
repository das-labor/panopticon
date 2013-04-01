#include <sstream>
#include <algorithm>

#include <inflate.hh>
#include <basic_block.hh>
#include <mnemonic.hh>
#include <flowgraph.hh>
#include <procedure.hh>

using namespace po;
using namespace std;

odotstream::odotstream(void)
: ostringstream(), calls(true), body(true), subgraph(false), instrs(false)
{}

odotstream &po::operator<<(odotstream &os, odotstream &(*func)(odotstream &os))
{
	return func(os);
}

odotstream &po::calls(odotstream &os) { os.calls = true; return os; }
odotstream &po::nocalls(odotstream &os) { os.calls = false; return os; }
odotstream &po::body(odotstream &os) { os.body = true; return os; }
odotstream &po::nobody(odotstream &os) { os.body = false; return os; }
odotstream &po::subgraph(odotstream &os) { os.subgraph = true; return os; }
odotstream &po::nosubgraph(odotstream &os) { os.subgraph = false; return os; }
odotstream &po::instrs(odotstream &os) { os.instrs = true; return os; }
odotstream &po::noinstrs(odotstream &os) { os.instrs = false; return os; }

oturtlestream::oturtlestream(void)
: ostringstream(), m_blank(0)
{
	*this << "@prefix : <http://localhost/>." << endl;
	*this << "@prefix po: <http://panopticum.io/>." << endl;
	*this << "@prefix xsd: <http://www.w3.org/2001/XMLSchema#>." << endl;
	*this << "@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>." << endl;
}

string oturtlestream::blank(void)
{
	return "_:z" + to_string(m_blank++);
}

oturtlestream &po::operator<<(oturtlestream &os, ostream& (*func)(ostream&))
{
	func(os);
	return os;
}

string po::turtle(flow_ptr fg)
{
	stringstream ss;
	unsigned long next_blank = 0;
	auto blank = [&](void) { return "z" + to_string(next_blank++); };

	ss << "@prefix : <http://localhost/>." << endl;

	/*if(fg->taint.size())
		ss << ":approx_taint rdf:type po:Approximation;" << endl
			 << "\tpo:title \"Taint analysis\"^^xsd:string." << endl;
	
	if(fg->cprop.size())
		ss << ":approx_cprop rdf:type po:Approximation;" << endl
			 << "\tpo:title \"Constant propagation\"^^xsd:string." << endl;*/
	
	// procedures
	for(const proc_ptr &proc: fg->procedures)
	{
		assert(proc && proc->entry);
		
		string procname(to_string(proc->entry->area().begin));
		stringstream ss_bblocks;
		//shared_ptr<map<bblock_ptr,taint_lattice>> taint_bblock(fg->taint.count(proc) ? fg->taint[proc] : nullptr);
		//shared_ptr<map<rvalue,sscp_lattice>> sscp(fg->simple_sparse_constprop.count(proc) ? fg->simple_sparse_constprop[proc] : nullptr);

		ss << ":proc_" 	<< procname << " rdf:type po:Procedure;" << endl
																<< "\tpo:name \"" << proc->name << "\";" << endl
																<< "\tpo:entry_point :bblock_" << procname << "_" << to_string(proc->entry->area().begin) << "." << endl;

		//for(const proc_ptr &c: proc->callees) 
		//	ss << ":proc_" << procname << " po:calls :proc_" << to_string(c->entry->area().begin) << "." << endl;

		// basic blocks
		for(const bblock_ptr &bb: proc->basic_blocks)
		{
			assert(bb);
			
			basic_block::succ_iterator j,jend;
			string bbname = to_string(bb->area().begin);
			stringstream ss_mnes;

			
			// mnemonics
			for(const mnemonic &mne: bb->mnemonics())
			{
				string mnename = blank();
				stringstream ss_ops;
				function<string(const rvalue &)> inflate_value;
				inflate_value = [&](const rvalue &v) -> string
				{
					string opname = blank();
					
					if(v.is_variable())
					{
						ss << ":" << opname << " rdf:type po:Variable;" << endl
												 				<< "\tpo:base \"" << v.to_variable().name() << "\"^^xsd:string;" << endl
												 				<< "\tpo:subscript \"" << v.to_variable().subscript() << "\"^^xsd:decimal." << endl;
					}
					else if(v.is_undefined())
					{
						ss << ":" << opname << " rdf:type po:Undefined." << endl;
					}
					else if(v.is_constant())
					{
						ss << ":" << opname << " rdf:type po:Constant;" << endl
												 				<< "\tpo:value \"" << v.to_constant().content() << "\"^^xsd:decimal." << endl;
					}
					else if(v.is_memory())
					{	
						string offname = inflate_value(v.to_memory().offset());
						ss << ":" << opname << " rdf:type po:Memory;" << endl
																<< "\tpo:offset :" << offname << ";" << endl
																<< "\tpo:bytes \"" << v.to_memory().bytes() << "\"^^xsd:decimal;" << endl
																<< "\tpo:endianess \"" << (int)v.to_memory().endianess() << "\"^^xsd:decimal." << endl;
					}
					else
						assert(false);

					return opname;
				};


				for(const rvalue &v: mne.operands)
					ss_ops << ":" << inflate_value(v) << " ";

				ss << ":" << mnename << " rdf:type po:Mnemonic;" << endl
														 << "\tpo:opcode \"" << mne.opcode << "\"^^xsd:string;" << endl
														 << "\tpo:begin \"" << mne.area.begin << "\"^^xsd:integer;" << endl
														 << "\tpo:end \"" << mne.area.end << "\"^^xsd:integer;" << endl
														 << "\tpo:operands (" << ss_ops.str() << ");" << endl
														 << "\tpo:format \"";

				for(const mnemonic::token &tok: mne.format)
					if(!tok.is_literal)
						ss << "{" << tok.width << ":" << (tok.has_sign ? "-" : "") << ":" << tok.alias << "}";
					else
						ss << tok.alias;

				ss << "\"^^xsd:string." << endl;
				
				ss_mnes << ":" << mnename;
				if(mne.area != bb->mnemonics().back().area)
					ss_mnes << ", ";

			}

			ss << ":bblock_" << procname << "_" << bbname << " rdf:type po:BasicBlock;" << endl
																										<< " po:executes " << ss_mnes.str() << "." << endl;
			
			/* instructions
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
				
	*			ss << ":var_" << procname << "_" << asname << " rdf:type po:Variable;" << endl	 
					 << "\tpo:width \"0\";" << endl 
					 << "\tpo:subscript \"" << lp[l]->assigns->nam.subscript << "\";" << endl
					 << "\tpo:base \"" << lp[l]->assigns->nam.base << "\"." << endl;

				*ss << ":instr_" << procname << "_" << bbname << "_" << instrname << " rdf:type " << instr_type << ";" << endl
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

			
			ss_bblocks << ":bblock_" << procname << "_" << bbname;
			if(bb != *next(proc->basic_blocks.end(),-1))
				ss_bblocks << ", ";
		}

		ss << ":proc_" 	<< procname << " po:contains " << ss_bblocks.str() << "." << endl;
	}

		/* basic blocks
		tie(i,iend) = proc->all();
		for_each(i,iend,[&](const bblock_ptr bb)
		{
			size_t sz = bb->mnemonics().size(), pos = 0;
			const mne_ptr *j = bb->mnemonics().data();
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
						instr_ptr in = *l++;
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
				const mne_ptr m = j[pos++];
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
						instr_ptr in = *l++;

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

string po::graphviz(flow_ptr fg)
{
	stringstream ss;

	ss << "digraph G" << endl
		 << "{" << endl
		 << "\tnode [shape=record,style=filled,color=lightgray,fontsize=13,fontname=\"Monospace\"];" << endl;
	
	// procedures
	for(const proc_ptr &proc: fg->procedures)
	{
		assert(proc && proc->entry);
		string procname(to_string(proc->entry->area().begin));
		//shared_ptr<map<bblock_ptr,taint_lattice>> taint_bblock(fg->taint[proc]);
		//shared_ptr<map<bblock_ptr,cprop_lattice>> cprop_bblock(fg->cprop[proc]);

		ss << "\tsubgraph cluster_" << procname << endl
			 << "\t{" << endl
			 << "\t\tlabel = \"procedure at " << procname << "\";" << endl
			 << "\t\tcolor = black;" << endl
			 << "\t\tfontname = \"Monospace\";" << endl;

		// basic blocks
		for(const bblock_ptr &bb: proc->basic_blocks)
		{
			size_t mnemonic_sz = bb->mnemonics().size(), mnemonic_pos = 0;
			const mnemonic *mnemonic_vec = bb->mnemonics().data();
			//taint_lattice *tl = taint_bblock && taint_bblock->count(bb) ? &taint_bblock->at(bb) : 0;
			//cprop_lattice *cp = cprop_bblock && cprop_bblock->count(bb) ? &cprop_bblock->at(bb) : 0;

			ss << "\t\tbb_" << procname 
				 << "_" << bb->area().begin;

			if(mnemonic_sz)
				ss << " [label=<<table BORDER=\"0\" CELLBORDER=\"0\" CELLSPACING=\"0\" ALIGN=\"LEFT\">";
			else
				ss << " [label=\"" << bb->area().begin << "\"];" << endl;
		
			// mnemonics
			while(mnemonic_pos < mnemonic_sz)
			{
				const mnemonic &m = mnemonic_vec[mnemonic_pos++];
				size_t instr_sz = m.instructions.size(), instr_pos = 0;
				const instr *instr_vec = m.instructions.data();

				ss << "<tr ALIGN=\"LEFT\"><td ALIGN=\"LEFT\">0x" 
					 << hex << m.area.begin << dec 
					 << "</td><td ALIGN=\"LEFT\">" << m
					 << "</td></tr>";
				
				if(instr_sz)
				{
					ss << "<tr><td COLSPAN=\"2\"><table BORDER=\"0\" CELLBORDER=\"0\" CELLSPACING=\"0\" ALIGN=\"LEFT\">";
					while(instr_pos < instr_sz)
					{
						const instr &in = instr_vec[instr_pos++];

						ss << "<tr ALIGN=\"LEFT\"><td ALIGN=\"LEFT\">" 
							 << "<font POINT-SIZE=\"11\">" << in
							 << "</font></td>";//<td><font POINT-SIZE=\"11\">";

						/* taint
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
							ss << " ( )";*/

						ss << /*"</font></td>*/"</tr>";
					}
					ss << "</table></td></tr>";
				}
			}

			if(mnemonic_sz)
				ss << "</table>>];" << endl;

			// outgoing edges
			for_each(bb->outgoing().begin(),bb->outgoing().end(),[&bb,&ss,&procname](const ctrans &ct) 
			{ 
				if(ct.bblock.lock())
				{
					ss << "\t\tbb_" << procname << "_" << bb->area().begin 
						 << " -> bb_" << procname << "_" << ct.bblock.lock()->area().begin
						 << " [label=\" " << ct.guard << " \"];" << endl; 
				}
				else
				{
					ss << "\t\tbb_" << procname << "_indir" << ct.value
					 << " [shape = circle, label=\"" << ct.value << "\"];" << endl
					 << "\t\tbb_" << procname << "_" << bb->area().begin 
					 << " -> bb_" << procname << "_indir" << ct.value
					 << " [label=\" " << ct.guard << " \"];" << endl;
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

		}
		
		ss << "\t}"  << endl;
	}

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

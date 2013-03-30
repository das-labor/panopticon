#include <sstream>
#include <algorithm>

#include <inflate.hh>
#include <basic_block.hh>
#include <mnemonic.hh>

using namespace po;
using namespace std;

odotstream::odotstream(void) : ostringstream() {}

odotstream &po::operator<<(odotstream &os, const flowgraph &f)
{
	os << "digraph G" << endl
		 << "{" << endl
		 << "\tgraph [label=\"" << f.name << "\"];" << endl;

	for(proc_cptr p: f.procedures)
		os << *p << endl;

	os << "}" << endl;
	return os;
}

odotstream &po::operator<<(odotstream &os, const procedure &p)
{
	os << "\tsubgraph cluster_" << p.name << endl
		 << "\t{" << endl
		 << "\t\tgraph [label=\"" << p.name << "\"];" << endl;

//	for(const mnemonic &m)
//		os << m << endl;

	os << "\t}" << endl;
	return os;
}
	
//odotstream &operator<<(odotstream &os, const mnemonic &m);
//odotstream &operator<<(odotstream &os, const instr &i);
//odotstream &operator<<(odotstream &os, rvalue v);

std::string po::turtle(flow_ptr fg)
{
	std::stringstream ss;
	unsigned long next_blank = 0;
	auto blank = [&](void) { return "z" + std::to_string(next_blank++); };

	ss << "@prefix : <http://localhost/>." << std::endl;

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
		
		std::string procname(std::to_string(proc->entry->area().begin));
		std::stringstream ss_bblocks;
		//shared_ptr<map<bblock_ptr,taint_lattice>> taint_bblock(fg->taint.count(proc) ? fg->taint[proc] : nullptr);
		//std::shared_ptr<std::map<rvalue,sscp_lattice>> sscp(fg->simple_sparse_constprop.count(proc) ? fg->simple_sparse_constprop[proc] : nullptr);

		ss << ":proc_" 	<< procname << " rdf:type po:Procedure;" << std::endl
																<< "\tpo:name \"" << proc->name << "\";" << std::endl
																<< "\tpo:entry_point :bblock_" << procname << "_" << std::to_string(proc->entry->area().begin) << "." << std::endl;

		//for(const proc_ptr &c: proc->callees) 
		//	ss << ":proc_" << procname << " po:calls :proc_" << std::to_string(c->entry->area().begin) << "." << std::endl;

		// basic blocks
		for(const bblock_ptr &bb: proc->basic_blocks)
		{
			assert(bb);
			
			basic_block::succ_iterator j,jend;
			std::string bbname = std::to_string(bb->area().begin);
			std::stringstream ss_mnes;

			
			// mnemonics
			for(const mnemonic &mne: bb->mnemonics())
			{
				std::string mnename = blank();
				std::stringstream ss_ops;
				std::function<std::string(const rvalue &)> inflate_value;
				inflate_value = [&](const rvalue &v) -> std::string
				{
					std::string opname = blank();
					
					if(v.is_variable())
					{
						ss << ":" << opname << " rdf:type po:Variable;" << std::endl
												 				<< "\tpo:base \"" << v.to_variable().name() << "\"^^xsd:string;" << std::endl
												 				<< "\tpo:subscript \"" << v.to_variable().subscript() << "\"^^xsd:decimal." << std::endl;
					}
					else if(v.is_undefined())
					{
						ss << ":" << opname << " rdf:type po:Undefined." << std::endl;
					}
					else if(v.is_constant())
					{
						ss << ":" << opname << " rdf:type po:Constant;" << std::endl
												 				<< "\tpo:value \"" << v.to_constant().content() << "\"^^xsd:decimal." << std::endl;
					}
					else if(v.is_memory())
					{	
						std::string offname = inflate_value(v.to_memory().offset());
						ss << ":" << opname << " rdf:type po:Memory;" << std::endl
																<< "\tpo:offset :" << offname << ";" << std::endl
																<< "\tpo:bytes \"" << v.to_memory().bytes() << "\"^^xsd:decimal;" << std::endl
																<< "\tpo:endianess \"" << (int)v.to_memory().endianess() << "\"^^xsd:decimal." << std::endl;
					}
					else
						assert(false);

					return opname;
				};


				for(const rvalue &v: mne.operands)
					ss_ops << ":" << inflate_value(v) << " ";

				ss << ":" << mnename << " rdf:type po:Mnemonic;" << std::endl
														 << "\tpo:opcode \"" << mne.opcode << "\"^^xsd:string;" << std::endl
														 << "\tpo:begin \"" << mne.area.begin << "\"^^xsd:integer;" << std::endl
														 << "\tpo:end \"" << mne.area.end << "\"^^xsd:integer;" << std::endl
														 << "\tpo:operands (" << ss_ops.str() << ");" << std::endl
														 << "\tpo:format \"";

				for(const mnemonic::token &tok: mne.format)
					if(!tok.is_literal)
						ss << "{" << tok.width << ":" << (tok.has_sign ? "-" : "") << ":" << tok.alias << "}";
					else
						ss << tok.alias;

				ss << "\"^^xsd:string." << std::endl;
				
				ss_mnes << ":" << mnename;
				if(mne.area != bb->mnemonics().back().area)
					ss_mnes << ", ";

			}

			ss << ":bblock_" << procname << "_" << bbname << " rdf:type po:BasicBlock;" << std::endl
																										<< " po:executes " << ss_mnes.str() << "." << std::endl;
			
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
					 << " po:precedes :bblock_" << procname << "_" << std::to_string(s->area().begin) << "." << std::endl; 
			});

			
			ss_bblocks << ":bblock_" << procname << "_" << bbname;
			if(bb != *next(proc->basic_blocks.end(),-1))
				ss_bblocks << ", ";
		}

		ss << ":proc_" 	<< procname << " po:contains " << ss_bblocks.str() << "." << std::endl;
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

std::string po::graphviz(flow_ptr fg)
{
	std::stringstream ss;

	ss << "digraph G" << std::endl
		 << "{" << std::endl
		 << "\tnode [shape=record,style=filled,color=lightgray,fontsize=13,fontname=\"Monospace\"];" << std::endl;
	
	// procedures
	for(const proc_ptr &proc: fg->procedures)
	{
		assert(proc && proc->entry);
		std::string procname(std::to_string(proc->entry->area().begin));
		//shared_ptr<map<bblock_ptr,taint_lattice>> taint_bblock(fg->taint[proc]);
		//shared_ptr<map<bblock_ptr,cprop_lattice>> cprop_bblock(fg->cprop[proc]);

		ss << "\tsubgraph cluster_" << procname << std::endl
			 << "\t{" << std::endl
			 << "\t\tlabel = \"procedure at " << procname << "\";" << std::endl
			 << "\t\tcolor = black;" << std::endl
			 << "\t\tfontname = \"Monospace\";" << std::endl;

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
				ss << " [label=\"" << bb->area().begin << "\"];" << std::endl;
		
			// mnemonics
			while(mnemonic_pos < mnemonic_sz)
			{
				const mnemonic &m = mnemonic_vec[mnemonic_pos++];
				size_t instr_sz = m.instructions.size(), instr_pos = 0;
				const instr *instr_vec = m.instructions.data();

				ss << "<tr ALIGN=\"LEFT\"><td ALIGN=\"LEFT\">0x" 
					 << std::hex << m.area.begin << std::dec 
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
				ss << "</table>>];" << std::endl;

			// outgoing edges
			for_each(bb->outgoing().begin(),bb->outgoing().end(),[&bb,&ss,&procname](const ctrans &ct) 
			{ 
				if(ct.bblock.lock())
				{
					ss << "\t\tbb_" << procname << "_" << bb->area().begin 
						 << " -> bb_" << procname << "_" << ct.bblock.lock()->area().begin
						 << " [label=\" " << ct.guard << " \"];" << std::endl; 
				}
				else
				{
					ss << "\t\tbb_" << procname << "_indir" << ct.value
					 << " [shape = circle, label=\"" << ct.value << "\"];" << std::endl
					 << "\t\tbb_" << procname << "_" << bb->area().begin 
					 << " -> bb_" << procname << "_indir" << ct.value
					 << " [label=\" " << ct.guard << " \"];" << std::endl;
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
		
		ss << "\t}"  << std::endl;
	}

	/*
	// callgraph
	ss << " subgraph cluster_calls" << std::endl
		 << " {" << std::endl
		 << " node [shape=circle,fontsize=15,fontname=\"Monospace\"];" << endl
		 << "  func_" << proc.entry_point << dec << " [label=\"" << proc.name << "\"];" << endl;
	for_each(proc.calls.begin(),proc.calls.end(),[&proc](const addr_t a) 
		{ ss << "   func_" << proc.entry_point << " -> func_" << a << ";" << endl; });
		ss << " }" << endl;*/
	ss << "}" << std::endl;
	
	return ss.str();
}

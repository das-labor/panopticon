#include <algorithm>

#include <flowgraph.hh>
#include <basic_block.hh>

using namespace po;
using namespace std;

flowgraph::flowgraph(const string &n) : name(n) {}

odotstream &po::operator<<(odotstream &os, const flowgraph &f)
{
	os << "digraph G" << endl
		 << "{" << endl
		 << "\tgraph [compound=true,label=\"" << f.name << "\"];" << endl;

	cout << f.procedures.size() << " procs" << endl;

	for(proc_cptr p: f.procedures)
	{
		assert(p);

		if(os.body)
			os << subgraph << *p << nosubgraph << endl;
		else
			os << *p << endl;

		if(os.calls)
		{
			for(proc_cwptr q: p->callees)
			{
				auto qq = q.lock();

				if(qq && os.body)
				{
					cout << qq->entry << " " << p->entry << endl;
					if(qq->entry && p->entry)
					{
						os << "\t"
							 << unique_name(*p->entry) 
							 << " -> "
							 << unique_name(*qq->entry)
							 << " [lhead=cluster_" << unique_name(*qq)
							 << ",ltail=cluster_" << unique_name(*p)
							 << "];" << endl;
					}
				}
				else if(qq)
				{
					os << "\t" 
						 << unique_name(*p)
						 << " -> " 
						 << unique_name(*qq)
						 << ";" << endl;
				}
			}
		}
	}

	os << "}" << endl;
	return os;
}

string po::unique_name(const flowgraph &f)
{
	return "flow_" + to_string((uintptr_t)&f);
}

oturtlestream& po::operator<<(oturtlestream &os, const flowgraph &f)
{
	string n = unique_name(f);

	os << ":" << n << " po:name \"" << f.name << "\"^^xsd:string." << endl;
	os << ":" << n << " rdf:type po:Flowgraph." << endl;
	
	for(proc_cptr p: f.procedures)
	{
		os << *p;
		os << ":" << n << " po:include :" << unique_name(*p) << "." << endl;
	}

	return os;
}

iturtlestream &po::operator>>(iturtlestream &is, flowgraph *&flow)
{
	flow = new flowgraph();
	return is;
}

proc_ptr po::find_procedure(flow_ptr fg, addr_t a)
{
	std::set<proc_ptr>::iterator i = fg->procedures.begin();
	
	while(i != fg->procedures.end())
		if(find_bblock(*i,a))
			return *i;
		else 
			++i;
	
	return proc_ptr(0);
}

bool po::has_procedure(flow_ptr flow, addr_t entry)
{
	return any_of(flow->procedures.begin(),flow->procedures.end(),[&](const proc_ptr p) 
								{ return p && p->entry && p->entry->area().includes(entry); });
}

set<addr_t> po::collect_calls(proc_cptr proc)
{
	set<addr_t> ret;

	execute(proc,[&](const lvalue &left, instr::Function fn, const std::vector<rvalue> &right)
	{
		if(fn == instr::Call)
		{
			assert(right.size() == 1);

			if(right[0].is_constant())
			{
				const constant &c = right[0].to_constant();
				ret.insert(c.content());
			}
		}
	});

	return ret;
}

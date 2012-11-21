#include <algorithm>

#include <flowgraph.hh>
#include <basic_block.hh>

using namespace po;

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
								{ return p->entry->area().includes(entry); });
}

#include <iostream>
#include <fstream>
#include <vector>
#include <algorithm>

#include "avr.hh"
#include "procedure.hh"

using namespace std;

/*
 * TODO
 * - finish disassemble
 * - cfg reconstr
 * main | <skip> | <2tok instr> = add to control_trans and instr
 * main | <skip> = 1 tok instr
 * avr.cc/hh instr_ptr/guard_ptr generieren lassen
 * cfg von instr_ptr/guard_ptr
 * call instr
 */

void decode(vector<uint16_t> &bytes)
{
	proc_ptr proc = avr_decode(bytes,6310);
	procedure::iterator i,iend;
	
	tie(i,iend) = proc->all();

	cout << "digraph G {" << endl << "node [shape=record,style=filled,color=lightgray,fontsize=13,fontname=\"-artwiz-smoothansi-medium-*-*-*-*-*-*-*-*-*-*-*\"];" << endl;
/*	for_each(cfg.begin(),cfg.end(),[](const procedure<uint16_t> &proc)
	{
		cout << " subgraph cluster_" << proc.name << endl;
		cout << " {" << endl;
		cout << "  label = \"" << proc.name << "\";" << endl;*/
		cout << "  color = black;" << endl;
		cout << "  fontname = \"-artwiz-smoothansi-medium-*-*-*-*-*-*-*-*-*-*-*\";" << endl;

		for_each(i,iend,[&proc](const bblock_ptr bb)
		{
			basic_block::iterator j,jend;

			tie(j,jend) = bb->mnemonics();

			cout << "  bb_" << /*proc.name << "_" <<*/ bb->addresses().begin << " [label=\"";
			
			for_each(j,jend,[](const mne_cptr m)
			{ 
				cout << "0x" << hex << m->addresses.begin << dec << ": " << m->name 
		//				 << " " << (ops.empty() ? "" : ops.substr(2,string::npos))
						 << "\\l"; 
			});

			cout << "\"];" << endl;

			basic_block::succ_iterator k,kend;
			tie(k,kend) = bb->successors();
			for_each(k,kend,[&bb](const bblock_ptr s) 
			{ 
				cout << "  bb_" << /*proc.name << "_" <<*/ bb->addresses().begin << 
								" -> bb_" << /*proc.name << "_" <<*/ s->addresses().begin << ";" << endl; 
			});
		});
/*		cout << " }" << endl;

		cout << " subgraph cluster_calls" << endl
				 << " {" << endl
				 << " node [shape=circle,fontsize=15,fontname=\"-artwiz-smoothansi-medium-*-*-*-*-*-*-*-*-*-*-*\"];" << endl
				 << "  func_" << proc.entry_point << dec << " [label=\"" << proc.name << "\"];" << endl;
		for_each(proc.calls.begin(),proc.calls.end(),[&proc](const addr_t a) 
			{ cout << "   func_" << proc.entry_point << " -> func_" << a << ";" << endl; });
		cout << " }" << endl;*/
//	});
	cout << "}" << endl;

}

int main(int argc, char *argv[])
{
	if(argc <= 1)
	{
		printf("AVR disasembler\n%s <files>\n",argv[0]);
		return 1;
	}

	int fn = 1;
	while(fn < argc)
	{
		ifstream f(argv[fn]);
		vector<uint16_t> bytes;

		if (f.bad())
        cout << "I/O error while reading" << endl;
    else if (f.fail())
        cout << "Non-integer data encountered" << endl;
		else 
		{
			while(f.good() && !f.eof())
			{
				uint16_t c;
				f.read((char *)&c,sizeof(c));
				bytes.push_back(c);
			}
			decode(bytes);
		}

		++fn;
	}

	return 0;
}

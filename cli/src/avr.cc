#include <fstream>
#include <vector>
#include <algorithm>

#include <avr/avr.hh>
#include <flowgraph.hh>

using namespace std;
using namespace po;

flow_ptr in_avr(const string &path)
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
		
		return avr::disassemble(bytes,0);
	}
}

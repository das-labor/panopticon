#include <iostream>
#include <output.hh>

using namespace std;
using namespace po;

void out_gv(const flow_ptr f, const string &path)
{
	/*ofstream o(path);
	vector<uint16_t> bytes;

	if (o.bad() || o.fail())
		cerr << path << ": I/O error while writing" << endl;
	else 
		o << graphviz(f);	*/
	
	odotstream os;

	os << *f;
	cout << os.str() << endl;

}

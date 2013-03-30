#include <input.hh>
#include <inflate.hh>
#include <deflate.hh>

using namespace po;
using namespace std;

flow_ptr in_turtle(const string &path)
{
	deflate df(path);

	cerr << "Turtle: " << path << endl;
	return df.flowgraph();
}

void out_turtle(const flow_ptr f, const string &path)
{
	/*ofstream o(path);
	vector<uint16_t> bytes;

	if (o.bad() || o.fail())
		cerr << path << ": I/O error while writing" << endl;
	else 
		o << turtle(f);*/

	odotstream os;

	os << *f;
	cout << os.str() << endl;
}

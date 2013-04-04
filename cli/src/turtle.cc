#include <input.hh>
#include <marshal.hh>
#include <flowgraph.hh>

using namespace po;
using namespace std;

flow_ptr in_turtle(const string &path)
{
	iturtlestream is(path);
	cerr << "Turtle: " << path << endl;

	flowgraph *flow = nullptr;
	is >> flow;

	cout << flow->name << endl;

	return flow_ptr(flow);
}

void out_turtle(const flow_ptr f, const string &path)
{
	/*ofstream o(path);
	vector<uint16_t> bytes;

	if (o.bad() || o.fail())
		cerr << path << ": I/O error while writing" << endl;
	else 
		o << turtle(f);*/

	oturtlestream os;

	os << *f;
	cout << os.str() << endl;
}

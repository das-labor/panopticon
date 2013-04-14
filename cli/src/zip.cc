#include <input.hh>
#include <output.hh>
#include <marshal.hh>

using namespace po;
using namespace std;

flow_ptr in_zip(const string &path)
{
	rdf::storage store = rdf::storage::from_archive(path);
	flow_ptr flow = flowgraph::unmarshal(store.first(nullptr,"rdf:type","po:Flowgraph").subject(),store);

	cout << "flow: " << flow << endl;
	return flow;
}

void out_zip(const flow_ptr f, const string &path)
{
	oturtlestream os;
	os << "po:world po:include " << *f << ".";
	cout << os.str() << endl;

	try
	{
		rdf::storage store = rdf::storage::from_stream(os);
		store.snapshot(path);
	}
	catch(marshal_exception &e)
	{
		cerr << e.what() << endl;
	}
}

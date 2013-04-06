#include <input.hh>
#include <marshal.hh>
#include <flowgraph.hh>

using namespace po;
using namespace std;

flow_ptr in_turtle(const string &path)
{
	rdf::storage store(path);
	cerr << "Turtle: " << path << endl;

	rdf::stream s = store.select(nullptr,"rdf:type","po:Flowgraph");

	if(!s.eof())
	{
		rdf::statement st;

		s >> st;
		return flowgraph::unmarshal(st.subject(),store);
	}
	else
		return flow_ptr(0);
}

void out_turtle(const flow_ptr f, const string &path)
{
	oturtlestream os;

	os << *f;
	cout << os.str() << endl;
}

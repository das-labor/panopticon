#include <input.hh>
#include <output.hh>
#include <marshal.hh>

using namespace po;
using namespace std;

flow_ptr in_zip(const string &path)
{
	try
	{
		rdf::storage store = rdf::storage::from_archive(path);
	}
	catch(marshal_exception &e)
	{
		cerr << e.what() << endl;
	}

	return flow_ptr(0);
}

void out_zip(const flow_ptr f, const string &path)
{
	oturtlestream os;
	os << *f;
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

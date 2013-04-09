#include <input.hh>
#include <output.hh>
#include <marshal.hh>

using namespace po;
using namespace std;

flow_ptr in_zip(const string &path)
{
	return flow_ptr(0);
}

void out_zip(const flow_ptr f, const string &path)
{
	oturtlestream os;
	os << *f;
	cout << os.str() << endl;

	rdf::storage store = rdf::storage::stream(os);
	store.save(path);
}

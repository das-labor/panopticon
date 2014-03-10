#include <iostream>
#include "rdf.hh"

using namespace std;

storage::storage(const string& base)
: _sp(), _op(), _so(), _po()
{
	_sp.open(base + "-sp.kct",PolyDB::OWRITER | PolyDB::OCREATE);
	_op.open(base + "-op.kct",PolyDB::OWRITER | PolyDB::OCREATE);
	_so.open(base + "-so.kct",PolyDB::OWRITER | PolyDB::OCREATE);
	_po.open(base + "-po.kct",PolyDB::OWRITER | PolyDB::OCREATE);
}

storage::~storage(void)
{
	_sp.close();
	_op.close();
	_so.close();
	_po.close();
}

bool storage::has(const string& s, const string& p, const string& o) const
{
	return find(s,p,o);
}

boost::optional<tuple<string,string,string>> storage::find(const boost::optional<string> &s,const boost::optional<string> &p,const boost::optional<string> &o) const
{
	if(s && p && o)
		return find_exact(s,p,o,1,2,3,_sp);
	else if(s && p && !o)
		return find_full(s,p,1,2,3,_sp);
	else if(s && !p && o)
		return find_full(s,o,1,3,2,_so);
	else if(s && !p && !o)
		return find_partial(s,1,2,3,_sp);
	else if(!s && p && o)
		return find_full(p,o,2,3,1);
	else if(!s && p && !o)
		return find_partial(p,2,3,1,_po);
	else if(!s && !p && o)
		return find_partial(o,3,2,1,_op);
	else
		return find_all(_sp);
}

bool storage::insert(const string& s, const string& p, const string& o)
{
	_sp.set(encode_key(s,p),o);
	_op.set(encode_key(o,p),s);
	_so.set(encode_key(s,o),p);
	_po.set(encode_key(p,o),s);

	return true;
}

string storage::encode_key(const string& a, const string& b)
{
	return varint(a.size()) + a + varint(b.size()) + b;
}

string storage::varint(size_t sz)
{
	string ret;

	while(sz)
	{
		ret.push_back((sz & 0x7f) | (sz > 0x7f ? 0x80 : 0));
		sz >>= 7;
	}

	return ret;
}


int main(int argc, char *argv[])
{
	storage st("test");

	st.insert("Hello",", ","World");
	st.insert("Hello",", ","You");
	st.insert("Bye",", ","You");
	st.insert("Bye",", ","World");
	st.insert("Hello",".","Bye");

	return 0;
}

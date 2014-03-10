#include <iostream>
#include <list>
#include <tuple>

#include "rdf.hh"

using namespace std;

storage::storage(const string& base)
: _meta()
{
	_meta.open(base + "meta.kct",PolyDB::OWRITER | PolyDB::OCREATE);
}

storage::~storage(void)
{
	_meta.close();
}

bool storage::has(const string& s, const string& p, const string& o) const
{
	return false;
}

list<tuple<string,string,string>> storage::find(const string &s, const string &p) const
{
	return list<tuple<string,string,string>>();
}

list<tuple<string,string,string>> storage::find(const string &s) const
{
	return list<tuple<string,string,string>>();
}

bool storage::insert(const string& s, const string& p, const string& o)
{
	_meta.set(encode_key(s,p,o),"");

	return true;
}


string storage::encode_key(const string& s, const string& p, const string& o)
{
	return encode_varint(s.size()) + s + encode_varint(p.size()) + p + encode_varint(o.size()) + o;
}

tuple<string,string,string> storage::decode_key(const std::string& k)
{
	return make_tuple("","","");
}

string storage::encode_varint(size_t sz)
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

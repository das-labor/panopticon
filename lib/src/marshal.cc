#include <sstream>
#include <algorithm>

#include <marshal.hh>

using namespace po;
using namespace std;

odotstream::odotstream(void)
: ostringstream(), calls(true), body(true), subgraph(false), instrs(false)
{}

odotstream &po::operator<<(odotstream &os, odotstream &(*func)(odotstream &os))
{
	return func(os);
}

odotstream &po::calls(odotstream &os) { os.calls = true; return os; }
odotstream &po::nocalls(odotstream &os) { os.calls = false; return os; }
odotstream &po::body(odotstream &os) { os.body = true; return os; }
odotstream &po::nobody(odotstream &os) { os.body = false; return os; }
odotstream &po::subgraph(odotstream &os) { os.subgraph = true; return os; }
odotstream &po::nosubgraph(odotstream &os) { os.subgraph = false; return os; }
odotstream &po::instrs(odotstream &os) { os.instrs = true; return os; }
odotstream &po::noinstrs(odotstream &os) { os.instrs = false; return os; }

oturtlestream::oturtlestream(void)
: ostringstream(), embed(false), m_blank(0)
{
	*this << "@prefix : <http://localhost/>." << endl;
	*this << "@prefix po: <http://panopticum.io/>." << endl;
	*this << "@prefix xsd: <http://www.w3.org/2001/XMLSchema#>." << endl;
	*this << "@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>." << endl;
}

string oturtlestream::blank(void)
{
	return "_:z" + to_string(m_blank++);
}

oturtlestream &po::embed(oturtlestream &os) { os.embed = true; return os; }
oturtlestream &po::noembed(oturtlestream &os) { os.embed = false; return os; }

oturtlestream &po::operator<<(oturtlestream &os, oturtlestream &(*func)(oturtlestream &os))
{
	return func(os);
}

oturtlestream &po::operator<<(oturtlestream &os, ostream& (*func)(ostream&))
{
	func(os);
	return os;
}

librdf_world *iturtlestream::s_rdf_world = 0;
raptor_world *iturtlestream::s_rap_world = 0;
unsigned int iturtlestream::s_usage = 0;
mutex iturtlestream::s_mutex;
unordered_map<string,librdf_node *> iturtlestream::s_nodes;

iturtlestream::iturtlestream(const string &path)
{
	lock_guard<mutex> g(s_mutex);

	if(!s_usage++)
	{
		assert(!s_rdf_world && !s_rap_world);
		
		s_rdf_world = librdf_new_world();
		librdf_world_open(s_rdf_world);
		s_rap_world = librdf_world_get_raptor(s_rdf_world);
	}

	assert(m_storage = librdf_new_storage(s_rdf_world,"memory",NULL,NULL));
	assert(m_model = librdf_new_model(s_rdf_world,m_storage,NULL));

	librdf_parser *parser;
	librdf_uri *uri;
	
	assert(parser = librdf_new_parser(s_rdf_world,"turtle",NULL,NULL));
	assert(uri = librdf_new_uri_from_filename(s_rdf_world,path.c_str()));
	assert(!librdf_parser_parse_into_model(parser,uri,uri,m_model));

	cout << librdf_model_size(m_model) << " triples in " << path << endl;	
	
	librdf_free_uri(uri);
	librdf_free_parser(parser);
}

iturtlestream::~iturtlestream(void)
{
	librdf_free_model(m_model);
	librdf_free_storage(m_storage);

	lock_guard<mutex> g(s_mutex);
	if(!--s_usage)
	{
		for(const pair<string,librdf_node *> &p: s_nodes)
			librdf_free_node(p.second);
		s_nodes.clear();

		librdf_free_world(s_rdf_world);
		s_rdf_world = 0;
		s_rap_world = 0;
	}
}

const librdf_node *iturtlestream::node(const string &s)
{
	lock_guard<mutex> g(s_mutex);
	auto i = s_nodes.find(s);

	if(i == s_nodes.end())
		i = s_nodes.insert(make_pair(s,librdf_new_node_from_uri_string(s_rdf_world,(const unsigned char *)s.c_str()))).first;

	return i->second;
}

const librdf_node *iturtlestream::po(const string &s)
{
	return node("http://panopticum.io/rdf/v1/rdf#" + s);
}

const librdf_node *iturtlestream::rdf(const string &s)
{
	return node("http://www.w3.org/1999/02/22-rdf-syntax-ns#" + s);
}

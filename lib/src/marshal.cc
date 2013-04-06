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
	*this << "@prefix : <" << LOCAL << ">." << endl;
	*this << "@prefix po: <" << PO << ">." << endl;
	*this << "@prefix xsd: <" << XSD << ">." << endl;
	*this << "@prefix rdf: <" << RDF << ">." << endl;
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

marshal_exception::marshal_exception(const string &w)
: runtime_error(w)
{}

rdf::storage::proxy::proxy(nullptr_t)
: is_literal(false), is_node(false)
{}

rdf::storage::proxy::proxy(const string &s)
: is_literal(true), is_node(false)
{
	if(s.find_first_of(":") == 0)
		literal = LOCAL + s.substr(1);
	else if(s.find_first_of("po:") == 0)
		literal = PO + s.substr(3);
	else if(s.find_first_of("xsd:") == 0)
		literal = XSD + s.substr(4);
	else if(s.find_first_of("rdf:") == 0)
		literal = RDF + s.substr(4);
	else
		literal = s;
}

rdf::storage::proxy::proxy(const char *s)
: proxy(string(s)) 
{}

rdf::storage::proxy::proxy(const rdf::node &n)
: is_literal(false), is_node(true), literal(""), node(n.inner() ? librdf_new_node_from_node(n.inner()) : 0)
{}

librdf_world *rdf::storage::s_rdf_world = 0;
raptor_world *rdf::storage::s_rap_world = 0;
unsigned int rdf::storage::s_usage = 0;
mutex rdf::storage::s_mutex;
unordered_map<string,librdf_node *> rdf::storage::s_nodes;

rdf::storage::storage(const string &path)
: m_storage(0), m_model(0)
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

rdf::storage::~storage(void)
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

rdf::stream rdf::storage::select(rdf::storage::proxy s, rdf::storage::proxy p, rdf::storage::proxy o) const
{
	lock_guard<mutex> g(s_mutex);
	function<librdf_node *(const proxy &p)> fn = [&](const proxy &p) -> librdf_node*
	{
		if(p.is_node)
			return p.node;
		else if(p.is_literal)
			return librdf_new_node_from_uri_string(s_rdf_world,reinterpret_cast<const unsigned char *>(p.literal.c_str()));
		else
			return NULL;
	};

	librdf_statement *partial = librdf_new_statement_from_nodes(s_rdf_world,fn(s),fn(p),fn(o));
	stream st(librdf_model_find_statements(m_model,partial));

	librdf_free_statement(partial);

	return st;
}

rdf::statement rdf::storage::first(rdf::storage::proxy s, rdf::storage::proxy p, rdf::storage::proxy o) const 
{
	stream st = select(s,p,o);

	if(st.eof())
		throw marshal_exception();

	statement ret;
	st >> ret;

	return ret;
}

rdf::node::node(librdf_node *n)
: m_node(n)
{}

rdf::node::node(const rdf::node &n)
: m_node(librdf_new_node_from_node(n.m_node))
{}

rdf::node::node(rdf::node &&n)
: m_node(n.m_node)
{
	n.m_node = 0;
}

rdf::node::~node(void)
{
	if(m_node)
		librdf_free_node(m_node);
}

rdf::node &rdf::node::operator=(const rdf::node &n)
{
	if(m_node)
		librdf_free_node(m_node);
	m_node = n.m_node ? librdf_new_node_from_node(n.m_node) : 0;

	return *this;
}

rdf::node &rdf::node::operator=(rdf::node &&n)
{
	m_node = n.m_node;
	n.m_node = 0;
	
	return *this;
}

string rdf::node::to_string(void) const
{
	if(!m_node)
		throw marshal_exception();

	if(librdf_node_is_literal(m_node))
		return string((char *)librdf_node_get_literal_value(m_node));
	else if(librdf_node_is_resource(m_node))
		return string(reinterpret_cast<const char *>(librdf_uri_as_string(librdf_node_get_uri(m_node))));
	else if(librdf_node_is_resource(m_node))
		return string(reinterpret_cast<const char *>(librdf_node_get_blank_identifier(m_node)));
	else
		throw marshal_exception("unknown node type");
}

librdf_node *rdf::node::inner(void) const
{
	return m_node;
}

rdf::statement::statement(librdf_statement *n)
: m_statement(n)
{}

rdf::statement::statement(const rdf::statement &n)
: m_statement(librdf_new_statement_from_statement(n.m_statement))
{}

rdf::statement::statement(rdf::statement &&n)
: m_statement(n.m_statement)
{
	n.m_statement = 0;
}

rdf::statement::~statement(void)
{
	if(m_statement)
		librdf_free_statement(m_statement);
}

rdf::statement &rdf::statement::operator=(const rdf::statement &n)
{
	if(m_statement)
		librdf_free_statement(m_statement);
	m_statement = n.m_statement ? librdf_new_statement_from_statement(n.m_statement) : 0;
	
	return *this;
}

rdf::statement &rdf::statement::operator=(rdf::statement &&n)
{
	m_statement = n.m_statement;
	n.m_statement = 0;
	
	return *this;
}

rdf::node rdf::statement::subject(void) const
{
	if(!m_statement || !librdf_statement_get_subject(m_statement))
		throw marshal_exception();

	return node(librdf_new_node_from_node(librdf_statement_get_subject(m_statement)));
}
	
rdf::node rdf::statement::predicate(void) const
{
	if(!m_statement || !librdf_statement_get_predicate(m_statement))
		throw marshal_exception();

	return node(librdf_new_node_from_node(librdf_statement_get_predicate(m_statement)));
}

rdf::node rdf::statement::object(void) const
{
	if(!m_statement || !librdf_statement_get_object(m_statement))
		throw marshal_exception();

	return node(librdf_new_node_from_node(librdf_statement_get_object(m_statement)));
}

rdf::stream::stream(librdf_stream *n)
: m_stream(n)
{}

rdf::stream::stream(rdf::stream &&n)
: m_stream(n.m_stream)
{
	n.m_stream = 0;
}

rdf::stream::~stream(void)
{
	if(m_stream)
		librdf_free_stream(m_stream);
}

rdf::stream &rdf::stream::operator>>(rdf::statement &st)
{
	if(eof())
		throw marshal_exception("stream at eof");

	st = statement(librdf_new_statement_from_statement(librdf_stream_get_object(m_stream)));
	librdf_stream_next(m_stream);

	return *this;
}

bool rdf::stream::eof(void) const
{
	return m_stream && librdf_stream_end(m_stream) != 0;
}

/*
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

const librdf_node *iturtlestream::axis(void) const
{
	return m_axis;
}

iturtlestream::axis_wrap po::setaxis(librdf_node *n)
{
	iturtlestream::axis_wrap a;
	a.node = n;

	return a;
}

iturtlestream &po::operator>>(iturtlestream &is, iturtlestream::axis_wrap &a)
{
	is.m_axis = a.node;
	return is;
}*/

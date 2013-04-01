#include <sstream>
#include <algorithm>

#include <inflate.hh>
#include <basic_block.hh>
#include <mnemonic.hh>
#include <flowgraph.hh>
#include <procedure.hh>

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

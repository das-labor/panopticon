#include <input.hh>

using namespace po;
using namespace std;

flow_ptr in_turtle(const string &path)
{
	cerr << "Turtle: " << path << endl;
	return flow_ptr(0);
}

void out_turtle(const flow_ptr f, const string &path)
{
	cerr << "Turtle-out: " << path << endl;
}

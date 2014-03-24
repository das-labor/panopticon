#include <panopticon/structure.hh>

using namespace po;
using namespace std;

template<>
rdf::statements po::marshal(const structure*, const uuid&)
{
	return rdf::statements();
}

template<>
structure* po::unmarshal(const uuid&, const rdf::storage&)
{
	return nullptr;
}

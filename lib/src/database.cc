#include <panopticon/database.hh>

using namespace po;
using namespace std;

template<>
rdf::statements po::marshal(const database*, const uuid&)
{
	return rdf::statements();
}

template<>
database* po::unmarshal(const uuid&, const rdf::storage&)
{
	return nullptr;
}

#include <panopticon/loc.hh>

using namespace po;

std::unordered_map<uuid,std::pair<marshal_poly,marshal_poly>> po::dirty_locations;

#ifdef __MINGW32__
boost::mutex po::dirty_locations_mutex;
#else
std::mutex po::dirty_locations_mutex;
#endif

void po::save_point(rdf::storage &store)
{
#ifdef __MINGW32__
	std::lock_guard<mutex::mutex> guard(dirty_locations_mutex);
#else
	std::lock_guard<std::mutex> guard(dirty_locations_mutex);
#endif

	for(const std::pair<uuid,std::pair<marshal_poly,marshal_poly>> &p: dirty_locations)
	{
		rdf::statements to_del = p.second.first();
		std::for_each(to_del.cbegin(),to_del.cend(),std::bind((bool(rdf::storage::*)(const rdf::statement&))&rdf::storage::remove,&store,std::placeholders::_1));

		rdf::statements to_new = p.second.second();
		std::for_each(to_new.cbegin(),to_new.cend(),std::bind((bool(rdf::storage::*)(const rdf::statement&))&rdf::storage::insert,&store,std::placeholders::_1));
	}

	dirty_locations.clear();
}

#include <panopticon/loc.hh>

using namespace std;
using namespace boost;
using namespace boost::uuids;

std::unordered_map<uuid,function<void(void)>> po::dirty_locations;
std::mutex po::dirty_locations_mutex;

void po::save_point(void)
{
	lock_guard<mutex> g(dirty_locations_mutex);

	for(const pair<uuid,function<void(void)>> &p: dirty_locations)
		p.second();
	dirty_locations.clear();
}

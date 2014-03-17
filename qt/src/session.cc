#include "session.hh"

Session::Session(QObject *p)
: QObject(p)
{
	po::region_loc base_as = po::region::undefined("base",128);
	po::region_loc xor_as = po::region::undefined("xor",64);
	po::region_loc add_as = po::region::wrap("add",std::initializer_list<po::byte>({0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26}));
//	po::address_space zlib_as("zlib",po::rrange(0,1280),std::function<bytes(const bytes&)>());
//	po::address_space aes_as("aes",po::rrange(0,320),std::function<bytes(const bytes&)>());

	auto base_desc = _regions.insert_node(base_as);
	auto xor_desc = _regions.insert_node(xor_as);
	auto add_desc = _regions.insert_node(add_as);
//	m_graph.insert_node(zlib_as);
//	m_graph.insert_node(aes_as);

	_regions.insert_edge(po::bound(10,74),xor_desc,base_desc);
	_regions.insert_edge(po::bound(80,107),add_desc,base_desc);
}

po::regions &Session::graph(void)
{
	return _regions;
}

Session::~Session(void)
{}

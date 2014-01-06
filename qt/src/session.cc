#include "session.hh"

Session::Session(QObject *p)
: QObject(p)
{
	using bytes = std::vector<uint8_t>;
	using vertex_descriptor = typename boost::graph_traits<po::regions>::vertex_descriptor;
	using edge_descriptor = typename boost::graph_traits<po::regions>::edge_descriptor;

	po::region_loc base_as(new po::region("base",128));
	po::region_loc xor_as(new po::region("xor",64));
	po::region_loc add_as(new po::region("add",27));
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

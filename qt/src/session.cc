#include <session.hh>

Session::Session(QObject *p)
: QObject(p)
{
	using bytes = std::vector<uint8_t>;
	using vertex_descriptor = typename boost::graph_traits<po::graph<po::address_space,po::rrange>>::vertex_descriptor;
	using edge_descriptor = typename boost::graph_traits<po::graph<po::address_space,po::rrange>>::edge_descriptor;

	po::address_space base_as("base",po::rrange(0,1280),std::function<bytes(const bytes&)>());
	po::address_space xor_as("xor",po::rrange(0,640),std::function<bytes(const bytes&)>());
	po::address_space add_as("add",po::rrange(0,270),std::function<bytes(const bytes&)>());
//	po::address_space zlib_as("zlib",po::rrange(0,1280),std::function<bytes(const bytes&)>());
//	po::address_space aes_as("aes",po::rrange(0,320),std::function<bytes(const bytes&)>());

	auto base_desc = m_graph.insert_node(base_as);
	auto xor_desc = m_graph.insert_node(xor_as);
	auto add_desc = m_graph.insert_node(add_as);
//	m_graph.insert_node(zlib_as);
//	m_graph.insert_node(aes_as);

	m_graph.insert_edge(po::rrange(100,740),xor_desc,base_desc);
	m_graph.insert_edge(po::rrange(800,1070),add_desc,base_desc);
}

po::graph<po::address_space,po::rrange> &Session::graph(void)
{
	return m_graph;
}

Session::~Session(void)
{}

#include <cassert>
#include <linearscene.hh>

LinearScene::LinearScene(QDeclarativeItem *parent)
: QDeclarativeItem(parent), m_nodes()
{
	using bytes = std::vector<uint8_t>;
	using vertex_descriptor = typename boost::graph_traits<po::graph<po::address_space,po::rrange>>::vertex_descriptor;
	using edge_descriptor = typename boost::graph_traits<po::graph<po::address_space,po::rrange>>::edge_descriptor;

	po::graph<po::address_space,po::rrange> m_graph;

	po::address_space base_as("base",po::rrange(0,128),std::function<bytes(const bytes&)>());
	po::address_space xor_as("xor",po::rrange(0,64),std::function<bytes(const bytes&)>());
	po::address_space add_as("add",po::rrange(0,27),std::function<bytes(const bytes&)>());
	po::address_space zlib_as("zlib",po::rrange(0,128),std::function<bytes(const bytes&)>());
	po::address_space aes_as("aes",po::rrange(0,32),std::function<bytes(const bytes&)>());

	auto base_vx = m_graph.insert_node(base_as);
	auto xor_vx = m_graph.insert_node(xor_as);
	auto add_vx = m_graph.insert_node(add_as);
	auto zlib_vx = m_graph.insert_node(zlib_as);
	auto aes_vx = m_graph.insert_node(aes_as);

	m_graph.insert_edge(po::rrange(0,64),xor_vx,base_vx);
	m_graph.insert_edge(po::rrange(64,72),add_vx,base_vx);
	m_graph.insert_edge(po::rrange(45,64),add_vx,xor_vx);
	m_graph.insert_edge(po::rrange(80,128),zlib_vx,base_vx);
	m_graph.insert_edge(po::rrange(32,64),aes_vx,zlib_vx);

	graphChanged(m_graph);
}

LinearScene::~LinearScene(void)
{}

QDeclarativeListProperty<Section> LinearScene::nodes(void)
{
	return QDeclarativeListProperty<Section>(this,this,&appendCallback<Section>,&countCallback<Section>,&atCallback<Section>,&clearCallback<Section>);
}

const QList<Section*> &LinearScene::nodeList(void) const
{
	return m_nodes;
}

void LinearScene::graphChanged(const po::graph<po::address_space,po::rrange> &graph)
{
	auto proj = po::projection(graph.get_node(po::root(graph)),graph);

	m_nodes.clear();
	for(auto i: proj)
		m_nodes.append(new Section(QString::fromStdString(i.second.name),boost::icl::length(i.first)));
	nodesChanged();
}

template<>
void LinearScene::appendCallback(QDeclarativeListProperty<Section> *property, Section *value)
{
	LinearScene *graph = reinterpret_cast<LinearScene*>(property->data);

	if(graph)
	{
		assert(!graph->m_nodes.contains(value));

		graph->m_nodes.append(value);
		graph->nodesChanged();
	}
}

template<>
int LinearScene::countCallback(QDeclarativeListProperty<Section> *property)
{
	LinearScene *graph = reinterpret_cast<LinearScene*>(property->data);
	return graph ? graph->nodeList().count() : -1;
}

template<>
Section *LinearScene::atCallback(QDeclarativeListProperty<Section> *property, int idx)
{
	LinearScene *graph = reinterpret_cast<LinearScene*>(property->data);
	return graph ? graph->nodeList().value(idx) : 0;
}

template<>
void LinearScene::clearCallback(QDeclarativeListProperty<Section> *property)
{
	LinearScene *graph = reinterpret_cast<LinearScene*>(property->data);
	if(graph)
	{
		graph->m_nodes.clear();
		graph->nodesChanged();
	}
}

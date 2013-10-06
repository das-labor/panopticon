#include <filterwidget.hh>

/*
FilterWidgetItem::FilterWidgetItem(po::proc_ptr p, Field f)
: QTableWidgetItem((int)f), m_procedure(p)
{
	assert(p);
	setFlags(Qt::ItemIsSelectable | Qt::ItemIsEnabled);

	switch(f)
	{
	case EntryPoint:
		setText(p->entry ? QString("%1").arg(p->entry->area().begin) : "(no entry)");
		break;
	case Name:
		setText(p->name.size() ? QString::fromStdString(p->name) : "(unnamed)");
		break;
	default:
		setText("(unknown field type)");
	}
}

po::proc_ptr FilterWidgetItem::procedure(void) const
{
	assert(m_procedure);
	return m_procedure;
}

FilterWidgetItem::Field FilterWidgetItem::field(void) const
{
	return (Field)type();
}

bool FilterWidgetItem::operator<(const QTableWidgetItem &i) const
{
	const FilterWidgetItem *p = dynamic_cast<const FilterWidgetItem *>(&i);
	assert(p && p->field() == field());

	switch(field())
	{
	case EntryPoint:
		return procedure()->entry->area().begin < p->procedure()->entry->area().begin;
	default:
		return text() < i.text();
	}
}

QTableWidgetItem *FilterWidgetItem::clone(void) const
{
	return new FilterWidgetItem(procedure(),field());
}*/

FilterWidget::FilterWidget(QWidget *parent)
: QDockWidget("Filters",parent), m_graph()
{
	using bytes = std::vector<uint8_t>;
	using vertex_descriptor = typename boost::graph_traits<po::graph<po::address_space,po::rrange>>::vertex_descriptor;
	using edge_descriptor = typename boost::graph_traits<po::graph<po::address_space,po::rrange>>::edge_descriptor;

	std::unordered_map<edge_descriptor,int> w_map;

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

	w_map.insert(std::make_pair(m_graph.insert_edge(po::rrange(0,64),xor_vx,base_vx),1));
	w_map.insert(std::make_pair(m_graph.insert_edge(po::rrange(64,72),add_vx,base_vx),1));
	w_map.insert(std::make_pair(m_graph.insert_edge(po::rrange(45,64),add_vx,xor_vx),1));
	w_map.insert(std::make_pair(m_graph.insert_edge(po::rrange(80,128),zlib_vx,base_vx),1));
	w_map.insert(std::make_pair(m_graph.insert_edge(po::rrange(32,64),aes_vx,zlib_vx),1));

	boost::associative_property_map<std::unordered_map<edge_descriptor,int>> weight_adaptor(w_map);

	auto proj = po::projection(base_as,m_graph);

	auto common_parent = [&](vertex_descriptor v, vertex_descriptor u)
	{
		auto find_path = [&](vertex_descriptor x)
		{
			std::unordered_map<vertex_descriptor,int> d_map;
			boost::associative_property_map<std::unordered_map<vertex_descriptor,int>> distance_adaptor(d_map);

			std::unordered_map<vertex_descriptor,vertex_descriptor> p_map;
			boost::associative_property_map<std::unordered_map<vertex_descriptor,vertex_descriptor>> pred_adaptor(p_map);

			auto es = m_graph.edges();
			std::for_each(es.first,es.second,[&](const edge_descriptor e) { w_map[e] = 1; });
			boost::dijkstra_shortest_paths(m_graph,x,boost::weight_map(weight_adaptor).distance_map(distance_adaptor).predecessor_map(pred_adaptor));

			auto i = base_vx;
			std::list<vertex_descriptor> path({i});
			while(i != p_map[i])
			{
				i = p_map[i];
				path.push_back(i);
			}
			return path;
		};

		auto l1 =	find_path(v);
		auto l2 = find_path(u);

		return *std::find_first_of(l1.begin(),l1.end(),l2.begin(),l2.end());
	};


	std::cout << m_graph.get_node(common_parent(add_vx,xor_vx)).name << std::endl;

	/*m_list.horizontalHeader()->hide();
	m_list.horizontalHeader()->setStretchLastSection(true);
	m_list.setShowGrid(false);
	m_list.verticalHeader()->hide();
	m_list.setSelectionBehavior(QAbstractItemView::SelectRows);
	m_list.setSelectionMode(QAbstractItemView::SingleSelection);*/
	m_view.setColumnCount(3);
	setWidget(&m_view);

	/*connect(&m_list,SIGNAL(itemActivated(QTableWidgetItem *)),this,SLOT(activateItem(QTableWidgetItem*)));
	connect(m_list.selectionModel(),SIGNAL(currentChanged(const QModelIndex&,const QModelIndex &)),this,SLOT(currentChanged(const QModelIndex&,const QModelIndex &)));*/
	m_view.clear();
	m_view.insertTopLevelItem(0,new QTreeWidgetItem(QStringList({"Test","0","44"})));
	m_view.insertTopLevelItem(0,new QTreeWidgetItem(QStringList({"Test2","5","54"})));
	m_view.insertTopLevelItem(0,new QTreeWidgetItem(QStringList({"Test3","0","0"})));
}

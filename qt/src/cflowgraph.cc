#include <QDebug>
#include <cflowgraph.hh>

extern "C" {
#include <gvc.h>
}

CFlowgraph::CFlowgraph(QAbstractItemModel *m, QModelIndex i, QWidget *parent)
: Graph(m,i,parent) 
{
	populate();
};

void CFlowgraph::populate(void)
{
	QModelIndex bblocks = m_root.sibling(m_root.row(),Model::BasicBlocksColumn);
	int row = 0;
	GVC_t *gvc = gvContext();
	Agraph_t *graph = agopen((char *)std::string("g").c_str(),AGDIGRAPH);
	std::list<Agnode_t *> nodes;

	qDebug() << "pop";
	m_scene.clear();

	while(row < m_model->rowCount(bblocks))
	{
		QModelIndex i = bblocks.child(row,Model::AreaColumn);
		string name("a_" + to_string(row)) ;
		Agnode_t *n = agnode(graph,(char *)name.c_str());

		nodes.push_back(n);
		++row;
	}

	gvLayout(gvc,graph,"dot");
	gvRender(gvc,graph,"dot",NULL);

	std::for_each(nodes.begin(),nodes.end(),[&](Agnode_t *n)
	{
		QString pos(agget(n,(char *)std::string("pos").c_str()));
		QStringList coords = pos.split(",");

		assert(coords.size() == 2);
		unsigned long x = coords.at(0).toULongLong(), y = coords.at(1).toULongLong();

		m_scene.insert(new Node("A",QPoint(x,y)));
	});

	gvFreeLayout(gvc,graph);
	agclose(graph);
	gvFreeContext(gvc);
}

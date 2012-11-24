#include <procedurewidget.hh>

#include <QMap>
#include <basicblockwidget.hh>

ProcedureWidget::ProcedureWidget(QAbstractItemModel *m, QModelIndex i, QWidget *parent)
: GraphWidget(m,i,parent)
{
	populate();
}

void ProcedureWidget::populate(void)
{
	QModelIndex bblocks = m_root.sibling(m_root.row(),Model::BasicBlocksColumn);
	int row = 0;
	QMap<ptrdiff_t,QGraphicsObject *> nodes;

	m_scene.clear();

	// nodes
	while(row < m_model->rowCount(bblocks))
	{
		auto m = new BasicBlockWidget(bblocks.child(row,Model::AreaColumn),0);
		m_scene.insert(m);
		nodes.insert(bblocks.child(row,Model::UniqueIdColumn).data().toULongLong(),m);
		++row;
	}
	
	// edges
	row = 0;
	while(row < m_model->rowCount(bblocks))
	{
		QModelIndex succ = bblocks.child(row,Model::SuccessorsColumn);
		ptrdiff_t uid = bblocks.child(row,Model::UniqueIdColumn).data().toULongLong();
		QGraphicsObject *from = nodes[uid];
		int s = 0;
		
		while(s < m_model->rowCount(succ))
		{
			uid = succ.child(s,Model::UniqueIdColumn).data().toULongLong();
			QGraphicsObject *to = nodes[uid];

			m_scene.connect(from,to);
			++s;
		}

		++row;
	}

	m_scene.graphLayout("dot");
}

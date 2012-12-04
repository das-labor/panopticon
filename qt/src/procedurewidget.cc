#include <procedurewidget.hh>

#include <QMap>
#include <basicblockwidget.hh>
#include <controltransferwidget.hh>

ProcedureWidget::ProcedureWidget(QAbstractItemModel *m, QModelIndex i, QWidget *parent)
: GraphWidget(m,i,parent)
{
	setRootIndex(i);
}

QPointF ProcedureWidget::populate(void)
{
	const QModelIndex bblocks = m_root.sibling(m_root.row(),Model::BasicBlocksColumn);
	const QModelIndex entry = m_root.sibling(m_root.row(),Model::EntryPointColumn);
	int row = 0;
	QMap<ptrdiff_t,BasicBlockWidget *> nodes;

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
		const QModelIndex succ = bblocks.child(row,Model::SuccessorsColumn);
		const QModelIndex guard = bblocks.child(row,Model::SuccessorGuardsColumn);

		BasicBlockWidget *from = nodes[bblocks.child(row,Model::UniqueIdColumn).data().toULongLong()];
		int s = 0;
		
		while(s < m_model->rowCount(succ))
		{
			BasicBlockWidget *to = nodes[succ.child(s,Model::UniqueIdColumn).data().toULongLong()];
	
			qDebug() << "connect" << bblocks.child(row,Model::UniqueIdColumn).data().toULongLong() << "to" << succ.child(s,Model::UniqueIdColumn).data().toULongLong();
			m_scene.connect(new ControlTransferWidget(guard.child(s,Model::ValuesColumn),from,to));
			++s;
		}

		++row;
	}

	m_scene.layoutHierarchically();
	BasicBlockWidget *e = nodes[entry.child(0,Model::UniqueIdColumn).data().toULongLong()];

	assert(e);
	return e->pos() + e->boundingRect().center();
}

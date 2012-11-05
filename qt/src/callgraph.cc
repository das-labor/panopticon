#include <QTimer>
#include <callgraph.hh>

Callgraph::Callgraph(QAbstractItemModel *m, QModelIndex i, QItemSelectionModel *s, QWidget *parent)
: Graph(m,i,parent), m_selection(s)
{
	populate();

	connect(&m_scene,SIGNAL(selectionChanged()),this,SLOT(sceneSelectionChanged()));
	connect(m_selection,SIGNAL(selectionChanged(const QItemSelection&,const QItemSelection&)),this,SLOT(modelSelectionChanged(const QItemSelection&,const QItemSelection&)));
	connect(m_model,SIGNAL(dataChanged(const QModelIndex&,const QModelIndex&)),this,SLOT(dataChanged(const QModelIndex&,const QModelIndex&)));
}

Callgraph::~Callgraph(void)
{
	disconnect(&m_scene,SIGNAL(sceneRectChanged(const QRectF&)),this,SLOT(sceneRectChanged(const QRectF&)));
	disconnect(&m_scene,SIGNAL(selectionChanged()),this,SLOT(sceneSelectionChanged()));
	disconnect(m_selection,SIGNAL(selectionChanged(const QItemSelection&,const QItemSelection&)),this,SLOT(modelSelectionChanged(const QItemSelection&,const QItemSelection&)));
}

void Callgraph::populate(void)
{
	int row = 0;
	QModelIndex procs = m_root.sibling(m_root.row(),Model::ProceduresColumn);

	m_scene.clear();
	// TODO: edges, signals when updated
	while(row < m_model->rowCount(procs))
	{
		QModelIndex name = procs.child(row,Model::NameColumn);
		QModelIndex addr = procs.child(row,Model::EntryPointColumn);
		QModelIndex uid = procs.child(row,Model::UniqueIdColumn);
		Node *n = new Node(QString("%1: %2").arg(m_model->data(addr,Qt::DisplayRole).toString()).arg(m_model->data(name,Qt::DisplayRole).toString()));
		ptrdiff_t u = m_model->data(uid,Qt::DisplayRole).toULongLong();

		m_scene.insert(n);
		m_uid2procedure.insert(make_pair(u,n));
		m_procedure2row.insert(make_pair(n,row));
		++row;
	}

	row = 0;
	while(row < m_model->rowCount(procs))
	{
		QModelIndex callees = procs.child(row,Model::CalleesColumn);
		QModelIndex uid = procs.child(row,Model::UniqueIdColumn);
		ptrdiff_t u = m_model->data(uid,Qt::DisplayRole).toULongLong();
		int sow = 0;
		Node *from;
		auto i = m_uid2procedure.find(u);
		
		assert(i != m_uid2procedure.end());
		from = i->second;

		while(sow < m_model->rowCount(callees))
		{
			Node *to;
			u = m_model->data(callees.child(sow,Model::UniqueIdColumn),Qt::DisplayRole).toULongLong();
			auto j = m_uid2procedure.find(u);
			
			assert(j != m_uid2procedure.end());
			to = j->second;

			m_scene.connect(from,to);
			++sow;
		}

		++row;
	}

	QTimer::singleShot(1,&m_scene,SLOT(graphLayout()));
//	setSceneRect(r);
}

void Callgraph::sceneSelectionChanged(void)
{
	disconnect(&m_scene,SIGNAL(selectionChanged()),this,SLOT(sceneSelectionChanged()));
	disconnect(m_selection,SIGNAL(selectionChanged(const QItemSelection&,const QItemSelection&)),this,SLOT(modelSelectionChanged(const QItemSelection&,const QItemSelection&)));
	
	QListIterator<QModelIndex> j(m_selection->selectedRows(Model::UniqueIdColumn));
	while(j.hasNext())
	{
		QModelIndex idx = j.next();
		ptrdiff_t u = m_model->data(idx,Qt::DisplayRole).toULongLong();
		auto k = m_uid2procedure.find(u);
		assert(k != m_uid2procedure.end());
		Node *n = k->second;
		
		if(n)
		{
			auto e = m_scene.out_edges(n);
			for_each(e.first,e.second,[&](Arrow *a) { a->setHighlighted(false); });
		}
	}

	m_selection->clearSelection();
	QListIterator<QGraphicsItem *> i(m_scene.selectedItems());
	QModelIndex procs = m_root.sibling(m_root.row(),Model::ProceduresColumn);

	while(i.hasNext())
	{
		Node *n = dynamic_cast<Node *>(i.next());
		
		if(n)
		{
			auto j = m_procedure2row.find(n);
			auto e = m_scene.out_edges(n);

			assert(j != m_procedure2row.end());
			m_selection->select(procs.child(j->second,0),QItemSelectionModel::Select | QItemSelectionModel::Rows);

			for_each(e.first,e.second,[&](Arrow *a) { a->setHighlighted(true); });
		}
	}
	
	connect(&m_scene,SIGNAL(selectionChanged()),this,SLOT(sceneSelectionChanged()));
	connect(m_selection,SIGNAL(selectionChanged(const QItemSelection&,const QItemSelection&)),this,SLOT(modelSelectionChanged(const QItemSelection&,const QItemSelection&)));
}

void Callgraph::modelSelectionChanged(const QItemSelection &selected, const QItemSelection &deselected)
{	
	disconnect(&m_scene,SIGNAL(selectionChanged()),this,SLOT(sceneSelectionChanged()));
	disconnect(m_selection,SIGNAL(selectionChanged(const QItemSelection&,const QItemSelection&)),this,SLOT(modelSelectionChanged(const QItemSelection&,const QItemSelection&)));

	QListIterator<QModelIndex> i(selected.indexes());
	while(i.hasNext())
	{
		QModelIndex idx = i.next();
		ptrdiff_t u = m_model->data(idx.sibling(idx.row(),Model::UniqueIdColumn),Qt::DisplayRole).toULongLong();
		auto j = m_uid2procedure.find(u);
		assert(j != m_uid2procedure.end());
		auto e = m_scene.out_edges(j->second);
		
		j->second->setSelected(true);
		for_each(e.first,e.second,[&](Arrow *a) { a->setHighlighted(true); });
	}
	
	i = QListIterator<QModelIndex>(deselected.indexes());
	while(i.hasNext())
	{
		QModelIndex idx = i.next();
		ptrdiff_t u = m_model->data(idx.sibling(idx.row(),Model::UniqueIdColumn),Qt::DisplayRole).toULongLong();
		auto j = m_uid2procedure.find(u);
		assert(j != m_uid2procedure.end());
		auto e = m_scene.out_edges(j->second);

		j->second->setSelected(false);
		for_each(e.first,e.second,[&](Arrow *a) { a->setHighlighted(false); });
	}

	connect(&m_scene,SIGNAL(selectionChanged()),this,SLOT(sceneSelectionChanged()));
	connect(m_selection,SIGNAL(selectionChanged(const QItemSelection&,const QItemSelection&)),this,SLOT(modelSelectionChanged(const QItemSelection&,const QItemSelection&)));
}

void Callgraph::mouseDoubleClickEvent(QMouseEvent *event)
{
	QListIterator<QGraphicsItem *> i(items(event->pos()));
	while(i.hasNext())
	{
		Node *n = dynamic_cast<Node *>(i.next());
		if(n)
		{
			auto i = m_procedure2row.find(n);
			QModelIndex procs = m_root.sibling(m_root.row(),Model::ProceduresColumn);

			assert(i != m_procedure2row.end());
			emit activated(procs.child(i->second,0));
		}
	}
}

void Callgraph::dataChanged(const QModelIndex &topLeft, const QModelIndex &bottomRight)
{
	int row = topLeft.row();

	while(row <= bottomRight.row())
	{
		QModelIndex name = topLeft.sibling(row,Model::NameColumn);
		QModelIndex addr = topLeft.sibling(row,Model::EntryPointColumn);
		QModelIndex uid = topLeft.sibling(row,Model::UniqueIdColumn);
		ptrdiff_t u = m_model->data(uid,Qt::DisplayRole).toULongLong();
		auto i = m_uid2procedure.find(u);
		Node *n;

		if(i != m_uid2procedure.end())
		{
			n = i->second;
			n->setTitle(QString("%1: %2").arg(m_model->data(addr,Qt::DisplayRole).toString()).arg(m_model->data(name,Qt::DisplayRole).toString()));
		}
		++row;
	}
}

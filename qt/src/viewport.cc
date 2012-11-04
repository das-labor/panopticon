#include <QDebug>
#include <QCoreApplication>
#include <QTimer>
#include <QScrollBar>

#include <viewport.hh>

Viewport::Viewport(QAbstractItemModel *m, QModelIndex i, QItemSelectionModel *s, QWidget *parent)
: QGraphicsView(parent), m_model(m), m_selection(s), m_root(i)
{
	setScene(&m_graph);
	setRenderHints(QPainter::Antialiasing | QPainter::SmoothPixmapTransform | QPainter::TextAntialiasing | QPainter::HighQualityAntialiasing);
	setDragMode(QGraphicsView::RubberBandDrag);//QGraphicsView::ScrollHandDrag);

	populate();
	connect(&m_graph,SIGNAL(sceneRectChanged(const QRectF&)),this,SLOT(sceneRectChanged(const QRectF&)));
	connect(&m_graph,SIGNAL(selectionChanged()),this,SLOT(sceneSelectionChanged()));
	connect(m_selection,SIGNAL(selectionChanged(const QItemSelection&,const QItemSelection&)),this,SLOT(modelSelectionChanged(const QItemSelection&,const QItemSelection&)));
	connect(m_model,SIGNAL(dataChanged(const QModelIndex&,const QModelIndex&)),this,SLOT(dataChanged(const QModelIndex&,const QModelIndex&)));
}

Viewport::~Viewport(void)
{
	disconnect(&m_graph,SIGNAL(sceneRectChanged(const QRectF&)),this,SLOT(sceneRectChanged(const QRectF&)));
	disconnect(&m_graph,SIGNAL(selectionChanged()),this,SLOT(sceneSelectionChanged()));
	disconnect(m_selection,SIGNAL(selectionChanged(const QItemSelection&,const QItemSelection&)),this,SLOT(modelSelectionChanged(const QItemSelection&,const QItemSelection&)));
}

void Viewport::populate(void)
{
	int row = 0;
	QModelIndex procs = m_root.sibling(m_root.row(),Model::ProceduresColumn);

	m_graph.clear();
	// TODO: edges, signals when updated
	while(row < m_model->rowCount(procs))
	{
		QModelIndex name = procs.child(row,Model::NameColumn);
		QModelIndex addr = procs.child(row,Model::EntryPointColumn);
		QModelIndex uid = procs.child(row,Model::UniqueIdColumn);
		Node *n = new Node(QString("%1: %2").arg(m_model->data(addr,Qt::DisplayRole).toString()).arg(m_model->data(name,Qt::DisplayRole).toString()));
		ptrdiff_t u = m_model->data(uid,Qt::DisplayRole).toULongLong();

		m_graph.insert(n);
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

			m_graph.connect(from,to);
			++sow;
		}

		++row;
	}

	QTimer::singleShot(1,&m_graph,SLOT(graphLayout()));
//	setSceneRect(r);
}

void Viewport::sceneRectChanged(const QRectF &r)
{
	fitInView(r,Qt::KeepAspectRatio);
}

void Viewport::sceneSelectionChanged(void)
{
	disconnect(&m_graph,SIGNAL(selectionChanged()),this,SLOT(sceneSelectionChanged()));
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
			auto e = m_graph.out_edges(n);
			for_each(e.first,e.second,[&](Arrow *a) { a->setHighlighted(false); });
		}
	}

	m_selection->clearSelection();
	QListIterator<QGraphicsItem *> i(m_graph.selectedItems());
	QModelIndex procs = m_root.sibling(m_root.row(),Model::ProceduresColumn);

	while(i.hasNext())
	{
		Node *n = dynamic_cast<Node *>(i.next());
		
		if(n)
		{
			auto j = m_procedure2row.find(n);
			auto e = m_graph.out_edges(n);

			assert(j != m_procedure2row.end());
			m_selection->select(procs.child(j->second,0),QItemSelectionModel::Select | QItemSelectionModel::Rows);

			for_each(e.first,e.second,[&](Arrow *a) { a->setHighlighted(true); });
		}
	}
	
	connect(&m_graph,SIGNAL(selectionChanged()),this,SLOT(sceneSelectionChanged()));
	connect(m_selection,SIGNAL(selectionChanged(const QItemSelection&,const QItemSelection&)),this,SLOT(modelSelectionChanged(const QItemSelection&,const QItemSelection&)));
}

void Viewport::modelSelectionChanged(const QItemSelection &selected, const QItemSelection &deselected)
{	
	disconnect(&m_graph,SIGNAL(selectionChanged()),this,SLOT(sceneSelectionChanged()));
	disconnect(m_selection,SIGNAL(selectionChanged(const QItemSelection&,const QItemSelection&)),this,SLOT(modelSelectionChanged(const QItemSelection&,const QItemSelection&)));

	QListIterator<QModelIndex> i(selected.indexes());
	while(i.hasNext())
	{
		QModelIndex idx = i.next();
		ptrdiff_t u = m_model->data(idx.sibling(idx.row(),Model::UniqueIdColumn),Qt::DisplayRole).toULongLong();
		auto j = m_uid2procedure.find(u);
		assert(j != m_uid2procedure.end());
		auto e = m_graph.out_edges(j->second);
		
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
		auto e = m_graph.out_edges(j->second);

		j->second->setSelected(false);
		for_each(e.first,e.second,[&](Arrow *a) { a->setHighlighted(false); });
	}

	connect(&m_graph,SIGNAL(selectionChanged()),this,SLOT(sceneSelectionChanged()));
	connect(m_selection,SIGNAL(selectionChanged(const QItemSelection&,const QItemSelection&)),this,SLOT(modelSelectionChanged(const QItemSelection&,const QItemSelection&)));
}

void Viewport::wheelEvent(QWheelEvent *event)
{
	double fac = (double)(event->delta()) / 150.f;
	fac = fac > 0.0f ? 1 / fac : -fac;
	scale(fac,fac);
}

void Viewport::mouseDoubleClickEvent(QMouseEvent *event)
{
	QListIterator<QGraphicsItem *> i(items(event->pos()));
	while(i.hasNext())
	{
		Node *n = dynamic_cast<Node *>(i.next());
		if(n)
			qDebug() << n << "activated";
	}
}

void Viewport::mouseMoveEvent(QMouseEvent *event)
{
	if(event->buttons() & Qt::MiddleButton)
	{
		QPointF p = m_lastDragPos - event->pos();

		horizontalScrollBar()->setValue(horizontalScrollBar()->value() + p.x());
		verticalScrollBar()->setValue(verticalScrollBar()->value() + p.y());
		m_lastDragPos = event->pos();
	}
	else
		QGraphicsView::mouseMoveEvent(event);
}

void Viewport::mousePressEvent(QMouseEvent *event)
{
	if(event->button() == Qt::MiddleButton)
	{
		m_lastDragPos = event->pos();
		viewport()->setCursor(Qt::ClosedHandCursor);
	}
	else
		QGraphicsView::mousePressEvent(event);
}

void Viewport::mouseReleaseEvent(QMouseEvent *event)
{	
	if(event->button() == Qt::MiddleButton)
		viewport()->setCursor(Qt::ArrowCursor);
	else
		QGraphicsView::mouseReleaseEvent(event);
}

void Viewport::dataChanged(const QModelIndex &topLeft, const QModelIndex &bottomRight)
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

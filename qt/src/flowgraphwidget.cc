#include <QTimer>
#include <flowgraphwidget.hh>

FlowgraphWidget::FlowgraphWidget(po::flow_ptr f, QItemSelectionModel *s, QWidget *parent)
: GraphWidget(parent), m_flowgraph(f), m_selection(s)
{
	snapshot();

	//connect(&m_scene,SIGNAL(sceneRectChanged(const QRectF&)),this,SLOT(sceneRectChanged(const QRectF&)));
	//connect(&m_scene,SIGNAL(selectionChanged()),this,SLOT(sceneSelectionChanged()));
	//connect(m_selection,SIGNAL(selectionChanged(const QItemSelection&,const QItemSelection&)),this,SLOT(modelSelectionChanged(const QItemSelection&,const QItemSelection&)));
	//connect(m_model,SIGNAL(dataChanged(const QModelIndex&,const QModelIndex&)),this,SLOT(dataChanged(const QModelIndex&,const QModelIndex&)));
}

FlowgraphWidget::~FlowgraphWidget(void)
{
	//disconnect(&m_scene,SIGNAL(sceneRectChanged(const QRectF&)),this,SLOT(sceneRectChanged(const QRectF&)));
	//disconnect(&m_scene,SIGNAL(selectionChanged()),this,SLOT(sceneSelectionChanged()));
	//disconnect(m_selection,SIGNAL(selectionChanged(const QItemSelection&,const QItemSelection&)),this,SLOT(modelSelectionChanged(const QItemSelection&,const QItemSelection&)));
}

void FlowgraphWidget::snapshot(void)
{
	std::lock_guard<std::mutex> guard(m_flowgraph->mutex);

	if(m_flowgraph->procedures.size() == m_procedureToNode.size())
		return;
	
	qDebug() << "snapshot";
	std::set<po::proc_ptr> new_procs;

	for(po::proc_ptr p: m_flowgraph->procedures)
	{
		if(!p || m_procedureToNode.count(p))
			continue;

		FlowgraphNode *n = new FlowgraphNode(QString("%1: %2").arg(p->entry->area().begin).arg(QString::fromStdString(p->name)));

		m_scene.insert(n);
		m_procedureToNode.insert(std::make_pair(p,n));
		new_procs.insert(p);
	}

	for(po::proc_ptr p: m_flowgraph->procedures)
	{
		if(!p) continue;

		FlowgraphNode *from;
		auto i = m_procedureToNode.find(p);
		
		if(i != m_procedureToNode.end())
		{
			from = i->second;
	
			for(po::proc_ptr q: p->callees)
			{
				FlowgraphNode *to;
				auto j = m_procedureToNode.find(q);
				
				if(j != m_procedureToNode.end())
				{
					to = j->second;
					if(new_procs.count(p) || new_procs.count(q))
						m_scene.connect(new ProcedureCallWidget(from,to));
				}
			}
		}
	}
	
	m_scene.layoutCustom("circo");
	fitInView(m_scene.sceneRect(),Qt::KeepAspectRatio);
	qDebug() << "snapshot done";
}

void FlowgraphWidget::sceneRectChanged(const QRectF &r)
{
	fitInView(r,Qt::KeepAspectRatio);
}

void FlowgraphWidget::sceneSelectionChanged(void)
{
	/*disconnect(&m_scene,SIGNAL(selectionChanged()),this,SLOT(sceneSelectionChanged()));
	disconnect(m_selection,SIGNAL(selectionChanged(const QItemSelection&,const QItemSelection&)),this,SLOT(modelSelectionChanged(const QItemSelection&,const QItemSelection&)));
	
	QListIterator<QModelIndex> j(m_selection->selectedRows(Model::UniqueIdColumn));
	while(j.hasNext())
	{
		QModelIndex idx = j.next();
		qulonglong u = m_model->data(idx,Qt::DisplayRole).toULongLong();
		auto k = m_uid2procedure.find(u);
		assert(k != m_uid2procedure.end());
		FlowgraphNode *n = k->second;
		
		if(n)
		{
			auto e = m_scene.out_edges(n);
			std::for_each(e.first,e.second,[&](Arrow *a) { dynamic_cast<ProcedureCallWidget *>(a)->setHighlighted(false); });
		}
	}

	m_selection->clearSelection();
	QListIterator<QGraphicsItem *> i(m_scene.selectedItems());
	QModelIndex procs = m_root.sibling(m_root.row(),Model::ProceduresColumn);

	while(i.hasNext())
	{
		FlowgraphNode *n = dynamic_cast<FlowgraphNode *>(i.next());
		
		if(n)
		{
			auto j = m_procedure2row.find(n);
			auto e = m_scene.out_edges(n);

			assert(j != m_procedure2row.end());
			m_selection->select(procs.child(j->second,0),QItemSelectionModel::Select | QItemSelectionModel::Rows);

			std::for_each(e.first,e.second,[&](Arrow *a) { dynamic_cast<ProcedureCallWidget *>(a)->setHighlighted(true); });
		}
	}
	
	connect(&m_scene,SIGNAL(selectionChanged()),this,SLOT(sceneSelectionChanged()));
	connect(m_selection,SIGNAL(selectionChanged(const QItemSelection&,const QItemSelection&)),this,SLOT(modelSelectionChanged(const QItemSelection&,const QItemSelection&)));*/
}

void FlowgraphWidget::modelSelectionChanged(const QItemSelection &selected, const QItemSelection &deselected)
{	
	/*disconnect(&m_scene,SIGNAL(selectionChanged()),this,SLOT(sceneSelectionChanged()));
	disconnect(m_selection,SIGNAL(selectionChanged(const QItemSelection&,const QItemSelection&)),this,SLOT(modelSelectionChanged(const QItemSelection&,const QItemSelection&)));

	QListIterator<QModelIndex> i(selected.indexes());
	while(i.hasNext())
	{
		QModelIndex idx = i.next();
		qulonglong u = m_model->data(idx.sibling(idx.row(),Model::UniqueIdColumn),Qt::DisplayRole).toULongLong();
		auto j = m_uid2procedure.find(u);
		assert(j != m_uid2procedure.end());
		auto e = m_scene.out_edges(j->second);
		
		j->second->setSelected(true);
		std::for_each(e.first,e.second,[&](Arrow *a) { dynamic_cast<ProcedureCallWidget *>(a)->setHighlighted(true); });
	}
	
	i = QListIterator<QModelIndex>(deselected.indexes());
	while(i.hasNext())
	{
		QModelIndex idx = i.next();
		qulonglong u = m_model->data(idx.sibling(idx.row(),Model::UniqueIdColumn),Qt::DisplayRole).toULongLong();
		auto j = m_uid2procedure.find(u);
		assert(j != m_uid2procedure.end());
		auto e = m_scene.out_edges(j->second);

		j->second->setSelected(false);
		std::for_each(e.first,e.second,[&](Arrow *a) { dynamic_cast<ProcedureCallWidget *>(a)->setHighlighted(false); });
	}

	connect(&m_scene,SIGNAL(selectionChanged()),this,SLOT(sceneSelectionChanged()));
	connect(m_selection,SIGNAL(selectionChanged(const QItemSelection&,const QItemSelection&)),this,SLOT(modelSelectionChanged(const QItemSelection&,const QItemSelection&)));*/
}

void FlowgraphWidget::mouseDoubleClickEvent(QMouseEvent *event)
{
/*	QListIterator<QGraphicsItem *> i(items(event->pos()));
	while(i.hasNext())
	{
		FlowgraphNode *n = dynamic_cast<FlowgraphNode *>(i.next());
		if(n)
		{
			auto i = m_procedure2row.find(n);
			QModelIndex procs = m_root.sibling(m_root.row(),Model::ProceduresColumn);

			assert(i != m_procedure2row.end());
			emit activated(procs.child(i->second,0));
		}
	}*/
}

FlowgraphNode::FlowgraphNode(QString name, QPoint ptn)
: m_text(name,this), m_rect(m_text.boundingRect().adjusted(0,0,10,10),this)
{
	m_rect.setPen(QPen(QBrush(Qt::black),2,Qt::SolidLine,Qt::RoundCap,Qt::RoundJoin));
	m_text.setZValue(1);

	setPos(ptn);
	setFlag(QGraphicsItem::ItemIsSelectable);

	m_text.setPos(5,5);

	itemChange(QGraphicsItem::ItemSelectedHasChanged,QVariant(false));
}

QRectF FlowgraphNode::boundingRect(void) const
{
	return m_rect.boundingRect();
}

void FlowgraphNode::paint(QPainter *painter, const QStyleOptionGraphicsItem *option, QWidget *widget)
{
	return;
}

QVariant FlowgraphNode::itemChange(GraphicsItemChange change, const QVariant &value)
{
	switch(change)
	{
	case QGraphicsItem::ItemSelectedHasChanged:
		m_rect.setBrush(QBrush(value.toBool() ? QColor(200,11,11) : QColor(11,200,11)));
	default:
		return value;
	}
}

void FlowgraphNode::setTitle(QString str)
{
	m_text.setPlainText(str);
}

ProcedureCallWidget::ProcedureCallWidget(FlowgraphNode *from, FlowgraphNode *to, QGraphicsItem *parent)
: QGraphicsItem(parent), m_from(from), m_to(to), m_path(QPainterPath(),this), m_highlighted(false)
{
	setPath(QPainterPath());
}

void ProcedureCallWidget::setPath(QPainterPath pp)
{
	m_path.setPath(pp);
	m_path.setPos(0,0);
}

QGraphicsObject *ProcedureCallWidget::from(void)
{
	return m_from;
}

QGraphicsObject *ProcedureCallWidget::to(void)
{
	return m_to;
}

QPainterPath ProcedureCallWidget::path(void) const
{
	return m_path.path();
}

void ProcedureCallWidget::setHighlighted(bool b)
{
	m_highlighted = b;
	m_path.setPen(QPen(b ? Qt::red : Qt::green,2));
}

QRectF ProcedureCallWidget::boundingRect(void) const
{
	return QRectF();
}

void ProcedureCallWidget::paint(QPainter *painter, const QStyleOptionGraphicsItem *option, QWidget *widget)
{
	return;
}

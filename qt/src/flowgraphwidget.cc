#include <QTimer>
#include <flowgraphwidget.hh>

FlowgraphWidget::FlowgraphWidget(po::flow_ptr f, QWidget *parent)
: GraphWidget(parent), m_flowgraph(f), m_selection(0)
{
	snapshot();

	connect(&m_scene,SIGNAL(sceneRectChanged(const QRectF&)),this,SLOT(sceneRectChanged(const QRectF&)));
	connect(&m_scene,SIGNAL(selectionChanged()),this,SLOT(sceneSelectionChanged()));
}

void FlowgraphWidget::snapshot(void)
{
	std::lock_guard<std::mutex> guard(m_flowgraph->mutex);
	std::set<po::proc_ptr> new_procs;

	if(m_flowgraph->procedures.size() == m_procedureToNode.size())
		return;

	for(po::proc_ptr p: m_flowgraph->procedures)
	{
		if(!p || m_procedureToNode.count(p))
			continue;

		FlowgraphNode *n = new FlowgraphNode(QString("%1: %2").arg(p->entry->area().begin).arg(QString::fromStdString(p->name)));

		m_scene.insert(n);
		m_procedureToNode.insert(std::make_pair(p,n));
		m_nodeToProcedure.insert(std::make_pair(n,p));
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
	
			for(po::proc_wptr qq: p->callees)
			{
				po::proc_ptr q = qq.lock();
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
}

void FlowgraphWidget::sceneRectChanged(const QRectF &r)
{
	fitInView(r,Qt::KeepAspectRatio);
}

void FlowgraphWidget::sceneSelectionChanged(void)
{
	select(m_scene.selectedItems().size() > 0 ? m_nodeToProcedure[dynamic_cast<FlowgraphNode *>(m_scene.selectedItems().first())] : nullptr);
}

void FlowgraphWidget::select(po::proc_ptr proc)
{
	if(m_selection == proc)
		return;

	if(m_selection)
	{
		FlowgraphNode *n = m_procedureToNode[m_selection];
		auto e = m_scene.out_edges(n);
		
		n->setSelected(false);
		std::for_each(e.first,e.second,[&](Arrow *a) { dynamic_cast<ProcedureCallWidget *>(a)->setHighlighted(false); });
	}
	

	m_selection = proc;
	emit selected(m_selection);
	
	if(m_selection)
	{
		FlowgraphNode *n = m_procedureToNode[m_selection];	
		auto e = m_scene.out_edges(n);
		
		n->setSelected(true);
		std::for_each(e.first,e.second,[&](Arrow *a) { dynamic_cast<ProcedureCallWidget *>(a)->setHighlighted(true); });
	}
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
	setHighlighted(false);
	setZValue(-1);
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
	m_path.setPen(QPen(b ? Qt::red : Qt::black,0));
}

QRectF ProcedureCallWidget::boundingRect(void) const
{
	return QRectF();
}

void ProcedureCallWidget::paint(QPainter *painter, const QStyleOptionGraphicsItem *option, QWidget *widget)
{
	return;
}

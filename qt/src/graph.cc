#include <QDebug>

#include <boost/graph/random_layout.hpp>
#include <boost/graph/fruchterman_reingold.hpp>
#include <boost/graph/circle_layout.hpp>
#include <boost/property_map/property_map.hpp>

#include <numeric>
#include <map>
#include <cassert>

#include <graph.hh>
#include <bgl.hh>

Node::Node(QString name, QPoint ptn)
: m_text(name,this), m_rect(m_text.boundingRect().adjusted(-5,-5,5,5),this), m_animation(0)
{
	m_rect.setBrush(QBrush(QColor(44,77,22)));
	m_rect.setPen(QPen(QBrush(Qt::black),2,Qt::SolidLine,Qt::RoundCap,Qt::RoundJoin));
	
	m_text.setZValue(1);
	setPos(ptn);
}

QRectF Node::boundingRect(void) const
{
	QListIterator<QGraphicsItem *> i(childItems());
	QRectF ret;

	while(i.hasNext())
	{
		QGraphicsItem *itm = i.next();
		ret = ret.united(itm->boundingRect());
	}
	
	return ret;
}

void Node::paint(QPainter *painter, const QStyleOptionGraphicsItem *option, QWidget *widget)
{
	return;
}

void Node::smoothSetPos(QPointF ptn)
{
	if(!m_animation)
	{
		m_animation = new Animation([this](const QVariant &v) { setPos(v.toPointF()); },this);
		m_animation->setStartValue(pos());
		m_animation->setDuration(3000);
		m_animation->setEasingCurve(QEasingCurve::OutCubic);
	}
	else if(m_animation->state() == QAbstractAnimation::Stopped)
		m_animation->setStartValue(pos());
	m_animation->setEndValue(ptn);

	if(m_animation->state() == QAbstractAnimation::Stopped)
		m_animation->start();
}

Arrow::Arrow(Node *f, Node *t)
: m_from(f), m_to(t)
{
	connect(m_from,SIGNAL(xChanged()),this,SLOT(refresh()));
	connect(m_from,SIGNAL(yChanged()),this,SLOT(refresh()));
	connect(m_to,SIGNAL(xChanged()),this,SLOT(refresh()));
	connect(m_to,SIGNAL(yChanged()),this,SLOT(refresh()));

	m_head << QPointF(0,0) << QPointF(3*-1.3,3*3) << QPointF(0,3*2.5) << QPointF(3*1.3,3*3) << QPointF(0,0);
}

QRectF Arrow::boundingRect(void) const
{
	QRectF a = mapFromItem(m_from,m_from->boundingRect().adjusted(-2,-2,2,2)).boundingRect();
	QRectF b = mapFromItem(m_to,m_to->boundingRect().adjusted(-2,-2,2,2)).boundingRect();

	return a | b;
}

void Arrow::paint(QPainter *painter, const QStyleOptionGraphicsItem *option, QWidget *widget)
{	
	QPolygonF con_a = mapFromItem(m_from,m_from->boundingRect().adjusted(-2,-2,2,2));
	QPolygonF con_b = mapFromItem(m_to,m_to->boundingRect().adjusted(-2,-2,2,2));
	QPointF cent_a = con_a.boundingRect().center();
	QPointF cent_b = con_b.boundingRect().center();
	QLineF los(cent_a,cent_b);
	std::function<QPointF(QPolygonF &, QPointF &)> collide = [&los](QPolygonF &contour, QPointF &backup) -> QPointF
	{
		if(contour.size() < 2) return backup;
		int idx = 1;
		QPointF prev = contour[0];

		while(idx < contour.size())
		{
			QLineF cand(prev,contour[idx]);
			QPointF inters;

			if(los.intersect(cand,&inters) == QLineF::BoundedIntersection)
				return inters;
			prev = contour[idx++];
		}

		return backup;
	};
	
	QLineF body(collide(con_a,cent_a),collide(con_b,cent_b));
	
	painter->save();
	painter->setPen(QPen(Qt::red,2));
	painter->setRenderHint(QPainter::Antialiasing);
	painter->setBrush(QBrush(Qt::red));
	painter->drawLine(body);
	painter->translate(body.p1());
	painter->rotate(270 - body.angle());
	painter->drawConvexPolygon(m_head);
	painter->restore();
}

Node *Arrow::from(void)
{
	return m_from;
}

Node *Arrow::to(void)
{
	return m_to;
}

void Arrow::refresh(void)
{
	prepareGeometryChange();
}

Animation::Animation(std::function<void(const QVariant &)> func, QObject *parent)
: QVariantAnimation(parent), m_function(func)
{
	return;
}

void Animation::updateCurrentValue(const QVariant &value)
{
	m_function(value);
}

Graph::Graph(void)
{
	return;
}

QList<Node *> &Graph::nodes(void)
{
	return m_nodes;
}

QList<Arrow *> &Graph::edges(void)
{
	return m_edges;
}

std::pair<QMultiMap<Node *,Arrow *>::iterator,QMultiMap<Node *,Arrow *>::iterator> Graph::out_edges(Node *n) 
{
	return std::make_pair(m_incidence.lowerBound(n),m_incidence.upperBound(n));
}

void Graph::graphLayout(void)
{
	boost::square_topology<> topo;
	std::map<Node *,typename boost::square_topology<>::point_type> pos_map;
	std::map<Node *,int> idx_map;
	boost::associative_property_map<std::map<Node *,typename boost::square_topology<>::point_type>> pos_adapter(pos_map);
	boost::associative_property_map<std::map<Node *,int>> idx_adapter(idx_map);

	unsigned int i = 0;
	QListIterator<Node *> j(nodes());
	while(j.hasNext())
		idx_map.insert(std::make_pair(j.next(),i++));

	//boost::random_graph_layout(this,adapter,topo);
	//boost::circle_graph_layout(this,adapter,0.5f);
	boost::fruchterman_reingold_force_directed_layout(this,pos_adapter,topo,boost::vertex_index_map(idx_adapter));

	QListIterator<Node *> k(nodes());
	while(k.hasNext())
	{
		Node *n = k.next();
		n->smoothSetPos(QPointF(pos_map[n][0] * 350.0,pos_map[n][1] * 350.0));
	}
}

void Graph::insert(Node *n)
{
	assert(!m_nodes.contains(n));
	
	addItem(n);
	m_nodes.append(n);
}

void Graph::connect(Node *a, Node *b)
{
	assert(m_nodes.contains(a) && m_nodes.contains(b));
	
	Arrow *e = new Arrow(a,b);

	addItem(e);
	m_edges.append(e);
	m_incidence.insert(a,e);
}

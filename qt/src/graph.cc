#include <QDebug>
#include <QApplication>
#include <QDesktopWidget>

#include <string>
#include <cassert>

#include <graph.hh>

extern "C" {
#include <gvc.h>
}

Node::Node(QString name, QPoint ptn)
: m_text(name,this), m_rect(m_text.boundingRect().adjusted(-5,-5,5,5),this), m_animation(0)
{
	m_rect.setPen(QPen(QBrush(Qt::black),2,Qt::SolidLine,Qt::RoundCap,Qt::RoundJoin));
	m_text.setZValue(1);

	setPos(ptn);
	setFlag(QGraphicsItem::ItemIsSelectable);

	itemChange(QGraphicsItem::ItemSelectedHasChanged,QVariant(false));
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

QVariant Node::itemChange(GraphicsItemChange change, const QVariant &value)
{
	switch(change)
	{
	case QGraphicsItem::ItemSelectedHasChanged:
		m_rect.setBrush(QBrush(value.toBool() ? QColor(200,11,11) : QColor(11,200,11)));
	default:
		return value;
	}
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

void Node::setTitle(QString s)
{
	m_text.setPlainText(s);
	m_rect.setRect(m_text.boundingRect().adjusted(-5,-5,5,5));
}

Arrow::Arrow(QGraphicsObject *f, QGraphicsObject *t)
: m_from(f), m_to(t), m_highlighted(false)
{
	connect(m_from,SIGNAL(xChanged()),this,SLOT(updated()));
	connect(m_from,SIGNAL(yChanged()),this,SLOT(updated()));
	connect(m_to,SIGNAL(xChanged()),this,SLOT(updated()));
	connect(m_to,SIGNAL(yChanged()),this,SLOT(updated()));

	setZValue(-1);
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
	painter->setPen(QPen(m_highlighted ? Qt::blue : Qt::red,2));
	painter->setRenderHint(QPainter::Antialiasing);
	painter->setBrush(QBrush(m_highlighted ? Qt::blue : Qt::red));
	painter->drawLine(body);
	painter->translate(body.p2());
	painter->rotate(90 - body.angle());
	painter->drawConvexPolygon(m_head);
	painter->restore();
}

QGraphicsObject *Arrow::from(void)
{
	return m_from;
}

QGraphicsObject *Arrow::to(void)
{
	return m_to;
}

void Arrow::setHighlighted(bool t)
{
	m_highlighted = t;
	updated();
}

void Arrow::updated(void)
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

QList<QGraphicsObject *> &Graph::nodes(void)
{
	return m_nodes;
}

QList<Arrow *> &Graph::edges(void)
{
	return m_edges;
}

std::pair<QMultiMap<QGraphicsObject *,Arrow *>::iterator,QMultiMap<QGraphicsObject *,Arrow *>::iterator> Graph::out_edges(QGraphicsObject *n) 
{
	return std::make_pair(m_incidence.lowerBound(n),m_incidence.upperBound(n));
}

QRectF Graph::graphLayout(QString algorithm)
{
	if(nodes().empty())
		return QRectF();

	GVC_t *gvc = gvContext();
	Agraph_t *graph = agopen((char *)std::string("g").c_str(),AGDIGRAPH);
	QMap<QGraphicsObject *,Agnode_t *> proxies;
	const QDesktopWidget *desk = QApplication::desktop();

	// allocate Graphviz nodes shadowing the Nodes in the scene
	QListIterator<QGraphicsObject *> i(nodes());
	while(i.hasNext())
	{
		QGraphicsObject *n = i.next();
		std::string name = std::to_string((ptrdiff_t)n);
		QRectF bb = n->boundingRect();
		Agnode_t *p = agnode(graph,(char *)name.c_str());
	
		agsafeset(p,(char *)std::string("width").c_str(),(char *)std::to_string(ceil(bb.width()/96.0)).c_str(),(char *)std::string("1").c_str());
		agsafeset(p,(char *)std::string("height").c_str(),(char *)std::to_string(ceil(bb.height()/96.0)).c_str(),(char *)std::string("1").c_str());

		proxies.insert(n,p);
	}

	// add Graphviz edges 
	QMapIterator<QGraphicsObject *, Arrow *> j(m_incidence);
	while(j.hasNext())
	{
		j.next();
		Arrow *a = j.value();
		Agnode_t *from = proxies[a->from()], *to = proxies[a->to()];

		agedge(graph,to,from);
	}

	gvLayout(gvc,graph,(char *)algorithm.toStdString().c_str());
	gvRender(gvc,graph,"dot",NULL);

	// move Nodes accoring to Graphviz proxies
	QMapIterator<QGraphicsObject *,Agnode_t *> k(proxies);
	while(k.hasNext())
	{
		k.next();
		QGraphicsObject *n = k.key();
		Agnode_t *p = k.value();
		QRectF bb = n->boundingRect();
		unsigned long x = ND_coord(p).x - (bb.width() / 2.0), y = ND_coord(p).y - (bb.height() / 2.0);

		//n->smoothSetPos(QPoint(x,y));
		n->setPos(QPoint(x,y));
	}

	agclose(graph);
	gvFreeContext(gvc);
	
	return QRectF();
}

void Graph::insert(QGraphicsObject *n)
{
	assert(!m_nodes.contains(n));
	
	addItem(n);
	m_nodes.append(n);
}

void Graph::connect(QGraphicsObject *from, QGraphicsObject *to)
{
	assert(m_nodes.contains(from) && m_nodes.contains(to));
	
	Arrow *e = new Arrow(from,to);

	addItem(e);
	m_edges.append(e);
	m_incidence.insert(from,e);
}

void Graph::clear(void)
{
	QGraphicsScene::clear();
	m_incidence.clear();
	m_edges.clear();
	m_nodes.clear();
}


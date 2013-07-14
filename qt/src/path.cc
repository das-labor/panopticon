#include <cassert>
#include "path.hh"

Path::Path(QDeclarativeItem *from, QDeclarativeItem *to,QDeclarativeItem *parent)
: QDeclarativeItem(parent), m_from(0), m_to(0), m_head(0), m_tail(0), m_pen(), m_direct(false), m_boundingRect()
{
	setZValue(1);
	setFlag(QGraphicsItem::ItemHasNoContents,false);

	setFrom(from);
	setTo(to);

	connect(&m_pen,SIGNAL(changed()),this,SLOT(update()));
	setAcceptsHoverEvents(true);
}

QRectF Path::boundingRect(void) const
{
	return m_boundingRect;
}

void Path::updateGeometry(void)
{
	prepareGeometryChange();

	if(m_from)
	{
		m_fromCenter = QGraphicsItem::mapFromItem(m_from,m_from->boundingRect()).boundingRect().center();

		if(m_to)
		{	
			m_toCenter = QGraphicsItem::mapFromItem(m_to,m_to->boundingRect()).boundingRect().center();
			m_boundingRect = QGraphicsItem::mapFromItem(m_from,m_from->boundingRect()).boundingRect() |
											 QGraphicsItem::mapFromItem(m_to,m_to->boundingRect()).boundingRect() |
											 m_path.boundingRect();	
		}
	}
}

void Path::update(void)
{
	QGraphicsItem::update();
}

void Path::setPath(const QPainterPath &pp)
{
	m_path = pp;
	updateGeometry();
}

void Path::paint(QPainter *painter, const QStyleOptionGraphicsItem *option, QWidget *widget)
{
	painter->save();
	painter->setPen(m_pen);
	painter->setRenderHints(QPainter::Antialiasing | QPainter::HighQualityAntialiasing);

	if(isDirect())
	{
		if(from() && to())
			painter->drawLine(QLineF(m_fromCenter,m_toCenter));
	}
	else
	{
		painter->drawPath(m_path);
	}

	painter->restore();
}

QDeclarativeItem *Path::from(void) const
{
	return m_from;
}

QDeclarativeItem *Path::to(void) const
{
	return m_to;
}

QDeclarativeItem *Path::head(void) const
{
	return m_head;
}

QDeclarativeItem *Path::tail(void) const
{
	return m_tail;
}

Pen *Path::pen(void)
{
	return &m_pen;
}

bool Path::isDirect(void) const
{
	return m_direct;
}

void Path::setFrom(QDeclarativeItem *n)
{
	if(m_from)
		disconnect(m_from);

	m_from = n;

	if(m_from)
	{
		connect(m_from,SIGNAL(xChanged()),this,SLOT(updateGeometry()));
		connect(m_from,SIGNAL(yChanged()),this,SLOT(updateGeometry()));
	}

	m_path = QPainterPath();
	positionEnds();
	updateGeometry();
	emit nodesChanged();
}

void Path::setTo(QDeclarativeItem *n)
{
	if(m_to)
		disconnect(m_to);

	m_to = n;

	if(m_to)
	{
		connect(m_to,SIGNAL(xChanged()),this,SLOT(updateGeometry()));
		connect(m_to,SIGNAL(yChanged()),this,SLOT(updateGeometry()));
	}

	m_path = QPainterPath();
	positionEnds();
	updateGeometry();
	emit nodesChanged();
}

void Path::setHead(QDeclarativeItem *n)
{
	m_head = n;
	positionEnds();
	updateGeometry();
}

void Path::setTail(QDeclarativeItem *n)
{
	m_tail = n;
	positionEnds();
	updateGeometry();
}

void Path::setDirect(bool b)
{
	m_direct = b;
	positionEnds();
	updateGeometry();
}

void Path::positionEnds(void)
{
	if(!m_path.elementCount())
		return;

	if(m_head && m_to)
	{
		QRectF bb = m_head->boundingRect();
		QLineF vec = contactVector(m_to);
		m_head->setPos(vec.p1() - QPointF(bb.width() / 2,bb.height() / 2));
		m_head->setRotation(90 - vec.angle());
	}

	if(m_tail && m_from)
	{
		QRectF bb = m_tail->boundingRect();
		QLineF vec = contactVector(m_from);
		m_tail->setPos(vec.p1() - QPointF(bb.width() / 2,bb.height() / 2));
		m_tail->setRotation(90 - vec.angle());
	}
}

QLineF Path::contactVector(QDeclarativeItem *itm) const
{
	QRectF bb = QGraphicsItem::mapFromItem(itm,itm->boundingRect()).boundingRect();
	std::function<std::pair<QLineF,qreal>(const QLineF&)> func = [&](const QLineF &ln)
	{
		qreal dist = std::numeric_limits<qreal>::max();
		std::function<qreal(qreal,qreal)> iter;
		iter = [&](qreal from, qreal to)
		{
			qreal len = to - from;
			qreal mid = from + len / 2.0;
			
			if(len < 0.001f)
				return mid;

			std::function<qreal(qreal)> measure = [&](qreal p)
			{
				QPointF b_ptn = m_path.pointAtPercent(p);
				QLineF normal = ln.normalVector().translated(b_ptn - ln.p1());
				QPointF cut_ptn;

				assert(ln.intersect(normal,&cut_ptn) != QLineF::NoIntersection);
				if(!bb.contains(cut_ptn))
					return QLineF(normal.p1(),ln.p1()).length();
				else
					return QLineF(normal.p1(),cut_ptn).length();
			};

			qreal left = measure(from + len / 4);
			qreal right = measure(to - len / 4);

			dist = std::min(std::min(dist,left),right);

			if(left < right)
				return iter(from,mid);
			else
				return iter(mid,to);
		};

		qreal t = iter(0,1);
		QLineF vec = QLineF::fromPolar(1,m_path.angleAtPercent(t));
		QPointF p1 = m_path.pointAtPercent(t);

		return std::make_pair(vec.translated(p1),dist);
	};

	QList<QLineF> es;
	qreal best_dist = std::numeric_limits<qreal>::max();
	QLineF ret;

	es.append(QLineF(bb.topLeft(),bb.topRight()));
	es.append(QLineF(bb.topRight(),bb.bottomRight()));
	es.append(QLineF(bb.bottomRight(),bb.bottomLeft()));
	es.append(QLineF(bb.bottomLeft(),bb.topLeft()));

	QListIterator<QLineF> iter(es);
	while(iter.hasNext())
	{
		QLineF p;
		qreal d;

		std::tie(p,d) = func(iter.next());
		if(d < best_dist)
		{
			best_dist = d;
			ret = p;
		}
	}

	return ret;
}

qreal Path::approximateDistance(const QPointF &pnt) const
{
	qreal dist = std::numeric_limits<qreal>::max();
	std::function<qreal(qreal,qreal)> iter;
	iter = [&](qreal from, qreal to)
	{
		qreal len = to - from;
		qreal mid = from + len / 2.0;
		
		if(len < 0.001f)
			return mid;

		qreal left = QLineF(pnt,m_path.pointAtPercent(from + len / 4)).length();
		qreal right = QLineF(pnt,m_path.pointAtPercent(to - len / 4)).length();

		dist = std::min(std::min(dist,left),right);

		if(left < right)
			return iter(from,mid);
		else
			return iter(mid,to);
	};

	iter(0,1);
	return dist;
}

void Path::hoverMoveEvent(QGraphicsSceneHoverEvent *event)
{
	//QPointF ptn = mapFromScene(event->scenePos());
	//qreal d = approximateDistance(ptn);
}

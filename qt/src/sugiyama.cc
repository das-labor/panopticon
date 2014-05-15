#include "sugiyama.hh"
/*
GraphScenePath::GraphScenePath(QQuickItem *from, QQuickItem *to,QQuickItem *parent)
: QQuickPaintedItem(parent), m_from(0), m_to(0), m_head(0), m_tail(0), m_pen(), m_direct(false), m_boundingRect()
{
	setZ(1);

	setFrom(from);
	setTo(to);

	connect(&m_pen,SIGNAL(changed()),this,SLOT(update()));
	setAcceptHoverEvents(true);
}

GraphScenePath::~GraphScenePath(void)
{}

QRectF GraphScenePath::contentsBoundingRect(void) const
{
	return m_boundingRect;
}

void GraphScenePath::updateGeometry(void)
{
	if(m_from)
	{
		m_fromCenter = mapFromItem(m_from,m_from->boundingRect().center());

		if(m_to)
		{
			m_toCenter = mapFromItem(m_to,m_to->boundingRect().center());
			m_boundingRect = QRectF(mapFromItem(m_from,m_from->boundingRect().topLeft()),QSizeF(m_from->width(),m_from->height())) |
											 QRectF(mapFromItem(m_to,m_to->boundingRect().topLeft()),QSizeF(m_to->width(),m_to->height())) |
											 m_path.boundingRect();
		}
	}
}

void GraphScenePath::update(void)
{
	QQuickPaintedItem::update();
}

void GraphScenePath::setPath(const QPainterPath &pp)
{
	m_path = pp;
	updateGeometry();
}

void GraphScenePath::paint(QPainter *painter)
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

QQuickItem *GraphScenePath::from(void) const
{
	return m_from;
}

QQuickItem *GraphScenePath::to(void) const
{
	return m_to;
}

QQuickItem *GraphScenePath::head(void) const
{
	return m_head;
}

QQuickItem *GraphScenePath::tail(void) const
{
	return m_tail;
}

GraphScenePen *GraphScenePath::pen(void)
{
	return &m_pen;
}

bool GraphScenePath::isDirect(void) const
{
	return m_direct;
}

void GraphScenePath::setFrom(QQuickItem *n)
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

void GraphScenePath::setTo(QQuickItem *n)
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

void GraphScenePath::setHead(QQuickItem *n)
{
	m_head = n;
	positionEnds();
	updateGeometry();
}

void GraphScenePath::setTail(QQuickItem *n)
{
	m_tail = n;
	positionEnds();
	updateGeometry();
}

void GraphScenePath::setDirect(bool b)
{
	m_direct = b;
	positionEnds();
	updateGeometry();
}

void GraphScenePath::positionEnds(void)
{
	if(!m_path.elementCount())
		return;

	if(m_head && m_to)
	{
		QRectF bb = m_head->boundingRect();
		QLineF vec = contactVector(m_to);
		QPointF pos(vec.p1() - QPointF(bb.width() / 2,bb.height() / 2));

		m_head->setX(pos.x());
		m_head->setY(pos.y());
		m_head->setRotation(90 - vec.angle());
	}

	if(m_tail && m_from)
	{
		QRectF bb = m_tail->boundingRect();
		QLineF vec = contactVector(m_from);
		QPointF pos(vec.p1() - QPointF(bb.width() / 2,bb.height() / 2));

		m_tail->setX(pos.x());
		m_tail->setY(pos.y());
		m_tail->setRotation(90 - vec.angle());
	}
}

QLineF GraphScenePath::contactVector(QQuickItem *itm) const
{
	QRectF bb(QQuickPaintedItem::mapFromItem(itm,itm->boundingRect().topLeft()),QSizeF(itm->width(),itm->height()));
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

qreal GraphScenePath::approximateDistance(const QPointF &pnt) const
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
}*/

Sugiyama::Sugiyama(QQuickItem *parent)
: QQuickPaintedItem(parent), _graph(), _delegate(nullptr), _vertices(), _edges()
{}

Sugiyama::~Sugiyama(void)
{}

void Sugiyama::route(void)
{
	if(po::num_edges(graph()))
	{
		SugiyamaInterface iface{this};

		emit routingStart();
		dot::astar<SugiyamaInterface>(iface);
		emit routingDone();
	}
}

void Sugiyama::layout(void)
{
	if(po::num_edges(graph()))
	{
		SugiyamaInterface iface{this};

		emit layoutStart();
		dot::layout<SugiyamaInterface>(iface,100,100);
		emit layoutDone();
	}
}

void Sugiyama::clear(void)
{
	if(_graph)
	{
		for(auto vx: iters(po::vertices(*_graph)))
		{
			QQuickItem *q = get_vertex(vx,*_graph).second;

			if(q)
			{
				q->setVisible(false);
				q->setParentItem(0);
				q->deleteLater();
			}
		}

		_graph = boost::none;
	}
}

po::digraph<std::pair<QVariant,QQuickItem*>,std::pair<QVariant,QPainterPath>>& Sugiyama::graph(void)
{
	if(!_graph)
	{
		_graph = po::digraph<std::pair<QVariant,QQuickItem*>,std::pair<QVariant,QPainterPath>>();


		QListIterator<QVariant> i(_vertices);
		while(i.hasNext())
		{
			QVariant var = i.next();
			QQuickItem *itm = nullptr;

			if(_delegate)
			{
				itm = qobject_cast<QQuickItem*>(_delegate->create(QQmlEngine::contextForObject(this)));
				itm->setParentItem(this);
			}

			insert_vertex(std::make_pair(var,itm),*_graph);
		}

		QListIterator<QVariant> j(_edges);
		while(j.hasNext())
		{
			using vx_desc = boost::graph_traits<po::digraph<std::pair<QVariant,QQuickItem*>,std::pair<QVariant,QPainterPath>>>::vertex_descriptor;
			QVariant var = j.next();
			QObject *obj = var.value<QObject*>();

			if(obj)
			{
				QQmlProperty from(obj,"from");
				QQmlProperty to(obj,"to");
				auto p = po::vertices(*_graph);

				auto a = std::find_if(p.first,p.second,[&](vx_desc v) { return get_vertex(v,*_graph).first == from.read(); });
				auto b = std::find_if(p.first,p.second,[&](vx_desc v) { return get_vertex(v,*_graph).first == to.read(); });

				if(a != p.second && b != p.second)
					insert_edge(std::make_pair(var,QPainterPath()),*a,*b,*_graph);
				else
					qWarning() << "Edge between unknown nodes";
			}
			else
				qWarning() << "Edge" << var << "has no attribute";
		}
	}

	return *_graph;
}

void Sugiyama::paint(QPainter *p)
{
	assert(p);

	for(auto e: iters(po::edges(graph())))
	{
		auto t = get_edge(e,graph());
		p->drawPath(t.second);
	}
	QRectF bb = contentsBoundingRect();
	QRectF bb2(mapToScene(QPointF(bb.x(),bb.y())),QSizeF(bb.width(),bb.height()));
	p->drawRect(QRectF(bb.x(),bb.y(),bb.width() - 1,bb.height() - 1));
}

/*
template<>
std::pair<QList<GraphScenePath*>::const_iterator,QList<GraphScenePath*>::const_iterator> dot::out_edges<SugiyamaInterface>(QQuickItem* n, SugiyamaInterface t)
{
	const QList<GraphScenePath*> &p = t.graph->outEdges(n);
	return std::make_pair(p.begin(),p.end());
}*/

/*template<>
std::pair<QList<Path*>::const_iterator,QList<Path*>::const_iterator> in_paths<SugiyamaInterface>(uint64_t n, SugiyamaInterface t)
{
	return t.paths_by_head.equal_range(n);
}*/

template<>
std::pair<dot::graph_traits<SugiyamaInterface>::node_iterator,dot::graph_traits<SugiyamaInterface>::node_iterator> dot::nodes<SugiyamaInterface>(SugiyamaInterface t)
{
	return vertices((*t)->graph());
}

template<>
std::pair<dot::graph_traits<SugiyamaInterface>::edge_iterator,dot::graph_traits<SugiyamaInterface>::edge_iterator> dot::edges<SugiyamaInterface>(SugiyamaInterface t)
{
	return edges((*t)->graph());
}

template<>
std::pair<dot::graph_traits<SugiyamaInterface>::out_edge_iterator,dot::graph_traits<SugiyamaInterface>::out_edge_iterator>
dot::out_edges<SugiyamaInterface>(dot::graph_traits<SugiyamaInterface>::node_type n, SugiyamaInterface t)
{
	return out_edges(n,(*t)->graph());
}

//template<>
//std::pair<QList<QQuickItem*>::const_iterator,QList<QQuickItem*>::const_iterator> in_paths<SugiyamaInterface>(uint64_t n, SugiyamaInterface t)

template<>
dot::graph_traits<SugiyamaInterface>::node_type dot::source<SugiyamaInterface>(dot::graph_traits<SugiyamaInterface>::edge_type e, SugiyamaInterface t)
{
	return po::source(e,(*t)->graph());
}

template<>
dot::graph_traits<SugiyamaInterface>::node_type dot::sink<SugiyamaInterface>(dot::graph_traits<SugiyamaInterface>::edge_type e, SugiyamaInterface t)
{
	return po::target(e,(*t)->graph());
}

template<>
unsigned int dot::weight<SugiyamaInterface>(dot::graph_traits<SugiyamaInterface>::edge_type e, SugiyamaInterface)
{
	return 1;
}

template<>
std::pair<unsigned int,unsigned int> dot::dimensions<SugiyamaInterface>(dot::graph_traits<SugiyamaInterface>::node_type n, SugiyamaInterface t)
{
	QQuickItem *q = get_vertex(n,(*t)->graph()).second;
	const QRectF &bb = q->boundingRect();
	return std::make_pair(bb.width(),bb.height());
}

template<>
bool dot::has_entry<SugiyamaInterface>(SugiyamaInterface g)
{
	return false;//g.root;
}

template<>
dot::graph_traits<SugiyamaInterface>::node_type dot::entry<SugiyamaInterface>(SugiyamaInterface g)
{
	assert(false);
}

template<>
void dot::set_position(dot::graph_traits<SugiyamaInterface>::node_type n, const dot::coord &pos, SugiyamaInterface t)
{
	QQuickItem *q = get_vertex(n,(*t)->graph()).second;
	q->setX(pos.first);
	q->setY(pos.second);
}

template<>
dot::coord dot::position(dot::graph_traits<SugiyamaInterface>::node_type n, SugiyamaInterface t)
{
	QQuickItem *q = get_vertex(n,(*t)->graph()).second;
	QPointF ptn(QPointF(q->x(),q->y()));
	return std::make_pair(ptn.x(),ptn.y());
}

//template<>
//bool is_free(float x, float y, unsigned int w, unsigned int h, QQuickItem *e, SugiyamaInterface g);

template<>
void dot::set_segments(dot::graph_traits<SugiyamaInterface>::edge_type e, const std::list<dot::coord> &segs, SugiyamaInterface graph)
{
	QPainterPath pp;

	// draw segments with bezier curves
	if(segs.size() > 2)
	{
		std::list<qreal> angles;
		auto d = std::next(segs.begin());
		QPointF f1(segs.front().first,segs.front().second);
		QPointF f2(std::next(segs.begin())->first,std::next(segs.begin())->second);

		angles.push_back(QLineF(f1,f2).angle());
		while(d != std::prev(segs.end()))
		{
			QPointF a(std::prev(d)->first,std::prev(d)->second);
			QPointF b(d->first,d->second);
			QPointF c(std::next(d)->first,std::next(d)->second);

			QLineF ln(a,b);
			angles.push_back(ln.angle() + ln.angleTo(QLineF(b,c)) / 2.0);
			++d;
		}

		QPointF x(std::prev(segs.end(),2)->first,std::prev(segs.end(),2)->second);
		QPointF y(std::prev(segs.end())->first,std::prev(segs.end())->second);
		angles.push_back(QLineF(x,y).angle());

		assert(angles.size() == segs.size());

		size_t idx = 0;
		while(idx < segs.size() - 1)
		{
			QPointF ptn1(std::next(segs.begin(),idx)->first,std::next(segs.begin(),idx)->second);
			qreal alpha1 = *std::next(angles.begin(),idx);

			QPointF ptn2(std::next(segs.begin(),idx + 1)->first,std::next(segs.begin(),idx + 1)->second);
			qreal alpha2 = *std::next(angles.begin(),idx + 1);

			qreal omega = std::min(QLineF(ptn1,ptn2).length() / 5.0,100.0);
			QPointF c1(QLineF::fromPolar(omega,alpha2).translated(ptn2).p2()), c2(QLineF::fromPolar(omega,alpha2 - 180.0).translated(ptn2).p2());
			QPointF e1(QLineF::fromPolar(omega,alpha1).translated(ptn1).p2()), e2(QLineF::fromPolar(omega,alpha1 - 180.0).translated(ptn1).p2());
			QPointF a,b;

			if(QLineF(ptn1,c1).length() > QLineF(ptn1,c2).length())
				b = c2;
			else
				b = c1;

			if(QLineF(ptn2,e1).length() > QLineF(ptn2,e2).length())
				a = e2;
			else
				a = e1;

			pp.moveTo(ptn1);
			pp.cubicTo(a,b,ptn2);
			++idx;
		}
	}
	else if(segs.size() == 2)
	{
		QPointF p1(segs.front().first,segs.front().second), p2(segs.back().first,segs.back().second);

		pp.moveTo(p1);
		pp.lineTo(p2);
	}

	get_edge(e,(*graph)->graph()).second = pp;
	(*graph)->update();
}

/*template<>
bool dot::is_free(float x, float y, unsigned int w, unsigned int h, Path *e, SugiyamaInterface g)
{
	QRectF bb(QPointF(x,y),QSizeF(w,h));
	QList<QQuickItem*> itms = g.scene->items(bb);

	bool ret = g.scene->sceneRect().contains(bb) &&
						 std::none_of(itms.begin(),itms.end(),[&](QQuickItem *i)
						 {
							 return i != e->from() &&
											i != e->to() &&
											g.graph->nodeList().contains(dynamic_cast<QQuickItem*>(i));
						 });
	return ret;
}*/

template<>
bool dot::is_free(const dot::vis_node<SugiyamaInterface> &a, const dot::vis_node<SugiyamaInterface> &b, SugiyamaInterface graph)
{
	QList<QQuickItem*> items = (*graph)->childItems();

	if(a.node.is_node())
		items.removeAll(get_vertex(a.node.node(),(*graph)->graph()).second);
	if(b.node.is_node())
		items.removeAll(get_vertex(b.node.node(),(*graph)->graph()).second);

	// collision?
	QLineF l(QPointF(a.position.first,a.position.second),QPointF(b.position.first,b.position.second));
	QListIterator<QQuickItem*> iter(items);

	while(iter.hasNext())
	{
		QQuickItem *i = iter.next();
		QPointF pos(i->x(),i->y());
		QRectF bb(pos,QSizeF(i->width(),i->height()));
		auto p = vertices((*graph)->graph());
		auto j = std::find_if(p.first,p.second,[&](dot::graph_traits<SugiyamaInterface>::node_type n) { return get_vertex(n,(*graph)->graph()).second == i; });
		QPointF c;

		if(j != p.second && (
			 l.intersect(QLineF(bb.topLeft(),bb.topRight()),&c) == QLineF::BoundedIntersection ||
			 l.intersect(QLineF(bb.topRight(),bb.bottomRight()),&c) == QLineF::BoundedIntersection ||
			 l.intersect(QLineF(bb.bottomRight(),bb.bottomLeft()),&c) == QLineF::BoundedIntersection ||
			 l.intersect(QLineF(bb.bottomLeft(),bb.topLeft()),&c) == QLineF::BoundedIntersection))
		{
			return false;
		}
	}
	return true;
}

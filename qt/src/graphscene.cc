#include "graphscene.hh"

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
}

GraphSceneItem::GraphSceneItem(QQuickItem *parent)
: QQuickItem(parent), m_nodes(), m_paths(), m_stateMachine(), m_rootState(new QState()), m_grabbedNodes()
{
	new GraphState(&m_stateMachine,m_rootState);

	m_rootState->setChildMode(QState::ParallelStates);

	m_stateMachine.addState(m_rootState);
	m_stateMachine.setInitialState(m_rootState);
	m_stateMachine.start();

	setAcceptedMouseButtons(Qt::LeftButton);
	connect(&m_stateMachine,SIGNAL(sent(const Event &)),this,SLOT(sent(const Event &)));
}

GraphSceneItem::~GraphSceneItem(void)
{}

void GraphSceneItem::mousePressEvent(QMouseEvent *event)
{
	QList<QQuickItem*> items = childItems();
	QListIterator<QQuickItem*> iter(items);

	while(iter.hasNext())
	{
		QQuickItem *item = dynamic_cast<QQuickItem*>(iter.next());
		if(m_nodes.contains(item) && item->contains(item->mapFromScene(event->pos())))
			m_stateMachine.send("press.node",QPair<QString,QVariant>("node",QVariant::fromValue(item)));
	}
	event->accept();
}

void GraphSceneItem::mouseReleaseEvent(QMouseEvent *event)
{
	QList<QQuickItem*> items = childItems();
	QListIterator<QQuickItem*> iter(items);

	while(iter.hasNext())
	{
		QQuickItem *item = dynamic_cast<QQuickItem*>(iter.next());
		if(m_nodes.contains(item) && item->contains(item->mapFromScene(event->pos())))
			m_stateMachine.send("release.node",QPair<QString,QVariant>("node",QVariant::fromValue(item)));
	}
	event->accept();
}

void GraphSceneItem::mouseMoveEvent(QMouseEvent *event)
{
	/// XXX
	QPointF delta = event->pos();// - event->lastGraphSceneItemPos();
	QSetIterator<QQuickItem*> iter(m_grabbedNodes);

	while(iter.hasNext())
	{
		QQuickItem *i = iter.next();
		QPointF pos(QPointF(i->x(),i->y()) + delta);

		i->setX(pos.x());
		i->setY(pos.y());
	}
}

bool GraphSceneItem::eventFilter(QObject *obj, QEvent *event)
{
	switch(event->type())
	{
		case QEvent::MouseButtonPress:
			mousePressEvent(dynamic_cast<QMouseEvent*>(event));
			return true;
		case QEvent::MouseButtonRelease:
			mouseReleaseEvent(dynamic_cast<QMouseEvent*>(event));
			return true;
		case QEvent::MouseMove:
			mouseMoveEvent(dynamic_cast<QMouseEvent*>(event));
			return true;
		default:
			return false;
	}
}

void GraphSceneItem::sent(const Event &n)
{
	//qDebug() << "sent" << n.name();
	m_stateMachine.postEvent(new Event(n));

	if(n.name() == "start.layout")
	{
		layoutNodes();
	}
	else if(n.name() == "start.route")
	{
		routeGraphScenePaths();
	}
	else if(n.name().endsWith(".node"))
	{
		assert(n.has("node"));
		QQuickItem *node = n["node"].value<QQuickItem*>();
		assert(m_nodes.contains(node));

		if(n.name() == "transition.node")
		{
			assert(n.has("state"));
			QString state = n["state"].value<QString>();

			node->setProperty("state",state);
		}
		else if(n.name() == "grab.node")
		{
			m_grabbedNodes.insert(node);
		}
		else if(n.name() == "drop.node")
		{
			m_grabbedNodes.remove(node);
		}
	}
	else if(n.name().endsWith(".path"))
	{
		assert(n.has("path"));
		GraphScenePath *path = reinterpret_cast<GraphScenePath*>(n["path"].value<void*>());
		assert(m_paths.contains(path));

		if(n.name() == "transition.path")
		{
			assert(n.has("state"));
			QString state = n["state"].value<QString>();

			path->setProperty("state",state);
		}
	}
}

QQmlListProperty<QQuickItem> GraphSceneItem::nodes(void)
{
	return QQmlListProperty<QQuickItem>(this,this,&appendCallback<QQuickItem>,&countCallback<QQuickItem>,&atCallback<QQuickItem>,&clearCallback<QQuickItem>);
}

QQmlListProperty<GraphScenePath> GraphSceneItem::paths(void)
{
	return QQmlListProperty<GraphScenePath>(this,this,&appendCallback<GraphScenePath>,&countCallback<GraphScenePath>,&atCallback<GraphScenePath>,&clearCallback<GraphScenePath>);
}

const QList<QQuickItem*> &GraphSceneItem::nodeList(void) const
{
	return m_nodes;
}

const QList<GraphScenePath*> &GraphSceneItem::pathList(void) const
{
	return m_paths;
}

const QList<GraphScenePath*> &GraphSceneItem::outEdges(QQuickItem *i) const
{
	QMap<QQuickItem*,QList<GraphScenePath*>>::const_iterator j = m_outEdges.constFind(i);

	assert(j != m_outEdges.constEnd());
	return *j;
}

void GraphSceneItem::updateOutEdges(void)
{
	QMutableMapIterator<QQuickItem*,QList<GraphScenePath*>> iter(m_outEdges);
	GraphScenePath *path = qobject_cast<GraphScenePath*>(sender());

	assert(path);
	while(iter.hasNext())
	{
		iter.next();
		iter.value().removeAll(path);
	}

	if(path->from())
		m_outEdges[path->from()].append(path);
}

void GraphSceneItem::routeGraphScenePaths(void)
{
	GraphSceneInterface iface(0,this);
	dot::astar<GraphSceneInterface>(iface);

	m_stateMachine.postEvent(new Event("done.route"));
}

void GraphSceneItem::layoutNodes(void)
{
	GraphSceneInterface iface(0,this);
	dot::layout<GraphSceneInterface>(iface,100,100);

	m_stateMachine.postEvent(new Event("done.layout"));
}

template<>
void GraphSceneItem::appendCallback(QQmlListProperty<QQuickItem> *property, QQuickItem *value)
{
	GraphSceneItem *graph = reinterpret_cast<GraphSceneItem*>(property->data);

	if(graph)
	{
		assert(!graph->m_nodes.contains(value));

		graph->m_nodes.append(value);
		value->installEventFilter(graph);
		value->setAcceptedMouseButtons(Qt::LeftButton);
		graph->m_nodeStates.insert(value,new NodeState(QVariant::fromValue(value),&graph->m_stateMachine,graph->m_rootState));
		if(!graph->m_outEdges.contains(value))
			graph->m_outEdges.insert(value,QList<GraphScenePath*>());

		graph->nodesChanged();
	}
}

template<>
void GraphSceneItem::appendCallback(QQmlListProperty<GraphScenePath> *property, GraphScenePath *value)
{
	GraphSceneItem *graph = reinterpret_cast<GraphSceneItem*>(property->data);

	if(graph)
	{
		assert(!graph->m_paths.contains(value));

		graph->m_paths.append(value);
		graph->m_pathStates.insert(value,new PathState(QVariant::fromValue<void*>(value),&graph->m_stateMachine,graph->m_rootState));

		graph->connect(value,SIGNAL(nodesChanged()),graph,SLOT(updateOutEdges()));
		if(value->from())
			graph->m_outEdges[value->from()].append(value);

		graph->pathsChanged();
	}
}

template<>
int GraphSceneItem::countCallback(QQmlListProperty<QQuickItem> *property)
{
	GraphSceneItem *graph = reinterpret_cast<GraphSceneItem*>(property->data);
	return graph ? graph->nodeList().count() : -1;
}

template<>
int GraphSceneItem::countCallback(QQmlListProperty<GraphScenePath> *property)
{
	GraphSceneItem *graph = reinterpret_cast<GraphSceneItem*>(property->data);
	return graph ? graph->pathList().count() : -1;
}

template<>
QQuickItem *GraphSceneItem::atCallback(QQmlListProperty<QQuickItem> *property, int idx)
{
	GraphSceneItem *graph = reinterpret_cast<GraphSceneItem*>(property->data);
	return graph ? graph->nodeList().value(idx) : 0;
}

template<>
GraphScenePath *GraphSceneItem::atCallback(QQmlListProperty<GraphScenePath> *property, int idx)
{
	GraphSceneItem *graph = reinterpret_cast<GraphSceneItem*>(property->data);
	return graph ? graph->pathList().value(idx) : 0;
}

template<>
void GraphSceneItem::clearCallback(QQmlListProperty<QQuickItem> *property)
{
	GraphSceneItem *graph = reinterpret_cast<GraphSceneItem*>(property->data);
	if(graph)
	{
		graph->m_nodes.clear();

		QMapIterator<QQuickItem*,NodeState*> iter(graph->m_nodeStates);
		while(iter.hasNext())
		{
			NodeState *st = *iter.next();
			QQuickItem *itm = iter.key();
			itm->removeEventFilter(graph);
			delete st;
		}

		graph->m_outEdges.clear();
		graph->m_nodeStates.clear();
		graph->m_grabbedNodes.clear();
		graph->nodesChanged();
	}
}

template<>
void GraphSceneItem::clearCallback(QQmlListProperty<GraphScenePath> *property)
{
	GraphSceneItem *graph = reinterpret_cast<GraphSceneItem*>(property->data);
	if(graph)
	{
		graph->m_paths.clear();

		QMapIterator<GraphScenePath*,PathState*> iter(graph->m_pathStates);
		while(iter.hasNext())
		{
			//delete iter.next();
			graph->disconnect(iter.key());
		}

		graph->m_pathStates.clear();
		graph->m_outEdges.clear();
		graph->pathsChanged();
	}
}


GraphSceneInterface::GraphSceneInterface(QQuickView *s, GraphSceneItem *g)
: graph(g), view(s)
{}

template<>
std::pair<QList<QQuickItem*>::const_iterator,QList<QQuickItem*>::const_iterator> dot::nodes<GraphSceneInterface>(GraphSceneInterface t)
{
	return std::make_pair(t.graph->nodeList().begin(),t.graph->nodeList().end());
}

template<>
std::pair<QList<GraphScenePath*>::const_iterator,QList<GraphScenePath*>::const_iterator> dot::edges<GraphSceneInterface>(GraphSceneInterface t)
{
	return std::make_pair(t.graph->pathList().begin(),t.graph->pathList().end());
}

template<>
std::pair<QList<GraphScenePath*>::const_iterator,QList<GraphScenePath*>::const_iterator> dot::out_edges<GraphSceneInterface>(QQuickItem* n, GraphSceneInterface t)
{
	const QList<GraphScenePath*> &p = t.graph->outEdges(n);
	return std::make_pair(p.begin(),p.end());
}

/*template<>
std::pair<QList<Path*>::const_iterator,QList<Path*>::const_iterator> in_paths<GraphSceneInterface>(uint64_t n, GraphSceneInterface t)
{
	return t.paths_by_head.equal_range(n);
}*/

template<>
QQuickItem* dot::source<GraphSceneInterface>(GraphScenePath* e, GraphSceneInterface)
{
	return e->from();
}

template<>
QQuickItem* dot::sink<GraphSceneInterface>(GraphScenePath* e, GraphSceneInterface)
{
	return e->to();
}

template<>
unsigned int dot::weight<GraphSceneInterface>(GraphScenePath* e, GraphSceneInterface)
{
	return 1;
}

template<>
std::pair<unsigned int,unsigned int> dot::dimensions<GraphSceneInterface>(QQuickItem* n, GraphSceneInterface)
{
	const QRectF &bb = n->boundingRect();
	return std::make_pair(bb.width(),bb.height());
}

template<>
bool dot::has_entry<GraphSceneInterface>(GraphSceneInterface g)
{
	return false;//g.root;
}

template<>
QQuickItem* dot::entry<GraphSceneInterface>(GraphSceneInterface g)
{
	assert(false);
}

template<>
void dot::set_position(QQuickItem *n, const coord &pos, GraphSceneInterface)
{
	n->setX(pos.first);
	n->setY(pos.second);
}

template<>
dot::coord dot::position(QQuickItem *n, GraphSceneInterface)
{
	assert(n);
	QPointF ptn(n->mapToScene(QPointF(n->x(),n->y())));
	return std::make_pair(ptn.x(),ptn.y());
}

template<>
void dot::set_segments(GraphScenePath *e, const std::list<coord> &segs, GraphSceneInterface)
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
		pp.moveTo(QPointF(segs.front().first,segs.front().second));
		pp.lineTo(QPointF(segs.back().first,segs.back().second));
	}

	e->setPath(pp);
}

/*template<>
bool dot::is_free(float x, float y, unsigned int w, unsigned int h, Path *e, GraphSceneInterface g)
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
bool dot::is_free(const dot::vis_node<GraphSceneInterface> &a, const dot::vis_node<GraphSceneInterface> &b, GraphSceneInterface graph)
{
	QPainterPath line;
	QList<QQuickItem*> items;

	line.moveTo(QPointF(a.position.first,a.position.second));
	line.lineTo(QPointF(b.position.first,b.position.second));

	items = graph.graph->childItems();

	if(a.node.is_node())
		items.removeAll(a.node.node());
	if(b.node.is_node())
		items.removeAll(b.node.node());

	// collision?
	QListIterator<QQuickItem*> iter(items);

	while(iter.hasNext())
	{
		QQuickItem *i = iter.next();
		QPointF pos(i->x(),i->y());
		QRectF bb(i->mapToItem(graph.graph,pos),QSizeF(i->width(),i->height()));

		if(line.contains(bb) && graph.graph->nodeList().contains(i))
			return false;
	}
	return true;
}

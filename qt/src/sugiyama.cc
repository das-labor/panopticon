#include "sugiyama.hh"

Sugiyama::Sugiyama(QQuickItem *parent)
: QQuickPaintedItem(parent), _graph(), _delegate(nullptr), _vertices(), _edges(), _direct(false), _mapper()
{
	connect(&_mapper,SIGNAL(mapped(QObject*)),this,SLOT(updateEdge(QObject*)));
}

Sugiyama::~Sugiyama(void)
{
	disconnect();
	clear();
}

void Sugiyama::route(void)
{
	if(po::num_edges(graph()))
	{
		SugiyamaInterface iface{this};

		emit routingStart();

		if(direct())
		{
			for(auto e: iters(dot::edges(iface)))
			{
				auto a = dot::source(e,iface), b = dot::sink(e,iface);
				auto ap = dot::position(a,iface), bp = dot::position(b,iface);
				auto aw = dot::dimensions(a,iface), bw = dot::dimensions(b,iface);

				dot::set_segments(e,{std::make_pair(ap.first + aw.first / 2,ap.second + aw.second / 2),
														 std::make_pair(bp.first + bw.first / 2,bp.second + bw.second / 2)},iface);
			}
		}
		else
		{
			dot::astar<SugiyamaInterface>(iface);
		}

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
			QQuickItem *q = get<1>(get_vertex(vx,*_graph));
			// ctx deleted wen q is deleted

			if(q)
			{
				q->setVisible(false);
				q->setParentItem(0);
				q->deleteLater();
			}
		}

		for(auto ed: iters(po::edges(*_graph)))
		{
			QQuickItem *a = get<2>(get_edge(ed,*_graph));
			QQuickItem *b = get<3>(get_edge(ed,*_graph));

			if(a)
				a->deleteLater();
			if(b)
				b->deleteLater();
		}

		_graph = boost::none;
	}
}

po::digraph<std::tuple<QVariant,QQuickItem*,QQmlContext*>,std::tuple<QVariant,QPainterPath,QQuickItem*,QQuickItem*>>& Sugiyama::graph(void)
{
	if(!_graph)
	{
		_graph = po::digraph<std::tuple<QVariant,QQuickItem*,QQmlContext*>,std::tuple<QVariant,QPainterPath,QQuickItem*,QQuickItem*>>();


		QListIterator<QVariant> i(_vertices);
		while(i.hasNext())
		{
			QVariant var = i.next();
			QQuickItem *itm = 0;
			QQmlContext *ctx = 0;

			if(_delegate)
			{
				ctx = new QQmlContext(QQmlEngine::contextForObject(this));
				itm = qobject_cast<QQuickItem*>(_delegate->create(ctx));
				itm->setParentItem(this);
			}

			insert_vertex(std::make_tuple(var,itm,ctx),*_graph);
		}

		QListIterator<QVariant> j(_edges);
		while(j.hasNext())
		{
			updateEdge(j.next().value<QObject*>());
		}
	}

	return *_graph;
}

void Sugiyama::updateEdge(QObject *obj)
{
	using vx_desc = boost::graph_traits<po::digraph<std::tuple<QVariant,QQuickItem*,QQmlContext*>,std::tuple<QVariant,QPainterPath,QQuickItem*,QQuickItem*>>>::vertex_descriptor;
	QVariant var = QVariant::fromValue(obj);

	if(obj)
	{
		QQmlProperty from(obj,"from");
		QQmlProperty to(obj,"to");
		QQmlProperty width(obj,"width");
		QQmlProperty color(obj,"color");
		QQmlProperty head(obj,"head");
		QQmlProperty tail(obj,"tail");
		auto p = po::vertices(*_graph);
		auto a = std::find_if(p.first,p.second,[&](vx_desc v) { return get<0>(get_vertex(v,*_graph)) == from.read(); });
		auto b = std::find_if(p.first,p.second,[&](vx_desc v) { return get<0>(get_vertex(v,*_graph)) == to.read(); });
		QQmlComponent *hc = head.read().value<QQmlComponent*>();
		QQmlComponent *tc = tail.read().value<QQmlComponent*>();

		if(a != p.second && b != p.second)
		{
			for(auto ex: po::iters(po::edges(*_graph)))
			{
				std::tuple<QVariant,QPainterPath,QQuickItem*,QQuickItem*> &t = get_edge(ex,*_graph);
				if(get<0>(t) == var)
				{
					if(get<2>(t))
						get<2>(t)->deleteLater();
					if(get<3>(t))
						get<3>(t)->deleteLater();

					remove_edge(ex,*_graph);
					break;
				}
			}

			QQuickItem *h = 0, *t = 0;

			if(hc)
			{
				h = qobject_cast<QQuickItem*>(hc->create(QQmlEngine::contextForObject(this)));
				h->setParentItem(this);
			}

			if(tc)
			{
				t = qobject_cast<QQuickItem*>(tc->create(QQmlEngine::contextForObject(this)));
				t->setParentItem(this);
			}

			insert_edge(std::make_tuple(var,QPainterPath(),h,t),*a,*b,*_graph);
		}
		else
		{
			qWarning() << "Edge between unknown nodes";
		}

		ensure(width.connectNotifySignal(this,SLOT(update())));
		ensure(color.connectNotifySignal(this,SLOT(update())));
		ensure(from.connectNotifySignal(&_mapper,SLOT(map())));
		ensure(to.connectNotifySignal(&_mapper,SLOT(map())));
		ensure(head.connectNotifySignal(&_mapper,SLOT(map())));
		ensure(tail.connectNotifySignal(&_mapper,SLOT(map())));
	}
	else
		qWarning() << "Edge" << var << "has no attribute";
}

void Sugiyama::paint(QPainter *p)
{
	ensure(p);
	p->save();

	for(auto e: iters(po::edges(graph())))
	{
		auto t = get_edge(e,graph());
		QObject *obj = get<0>(t).value<QObject*>();
		QQmlProperty width(obj,"width");
		QQmlProperty color(obj,"color");
		QPen pen(QBrush(color.read().value<QColor>()),width.read().toInt());

		p->setPen(pen);
		p->drawPath(get<1>(t));
	}

	p->restore();
}

void Sugiyama::redoAttached(void)
{
	for(auto vx: iters(po::vertices(graph())))
	{
		QVariantList incoming;

		for(auto e: iters(po::in_edges(vx,graph())))
		{
			auto ed = get_edge(e,graph());
			incoming.append(get<0>(ed));
		}

		auto v = get_vertex(vx,graph());
		get<2>(v)->setContextProperty("incoming",QVariant(incoming));
	}
}

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
	QQuickItem *q = get<1>(get_vertex(n,(*t)->graph()));
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
	ensure(false);
	return *po::vertices((*g)->graph()).first;
}

template<>
void dot::set_position(dot::graph_traits<SugiyamaInterface>::node_type n, const dot::coord &pos, SugiyamaInterface t)
{
	QQuickItem *q = get<1>(get_vertex(n,(*t)->graph()));
	q->setX(pos.first);
	q->setY(pos.second);
}

template<>
dot::coord dot::position(dot::graph_traits<SugiyamaInterface>::node_type n, SugiyamaInterface t)
{
	QQuickItem *q = get<1>(get_vertex(n,(*t)->graph()));
	QPointF ptn(QPointF(q->x(),q->y()));
	return std::make_pair(ptn.x(),ptn.y());
}

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

		ensure(angles.size() == segs.size());

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

	auto p = get_edge(e,(*graph)->graph());
	auto from = get_vertex(source(e,(*graph)->graph()),(*graph)->graph());
	auto to = get_vertex(target(e,(*graph)->graph()),(*graph)->graph());

	get<1>(get_edge(e,(*graph)->graph())) = pp;

	(*graph)->positionEnds(get<0>(p).value<QObject*>(),get<2>(p),get<3>(p),get<1>(from),get<1>(to),pp);
	(*graph)->update();
}

template<>
bool dot::is_free(const dot::vis_node<SugiyamaInterface> &a, const dot::vis_node<SugiyamaInterface> &b, SugiyamaInterface graph)
{
	QList<QQuickItem*> items = (*graph)->childItems();

	if(a.node.is_node())
		items.removeAll(get<1>(get_vertex(a.node.node(),(*graph)->graph())));
	if(b.node.is_node())
		items.removeAll(get<1>(get_vertex(b.node.node(),(*graph)->graph())));

	// collision?
	QLineF l(QPointF(a.position.first,a.position.second),QPointF(b.position.first,b.position.second));
	QListIterator<QQuickItem*> iter(items);

	while(iter.hasNext())
	{
		QQuickItem *i = iter.next();
		QPointF pos(i->x(),i->y());
		QRectF bb(pos,QSizeF(i->width(),i->height()));
		auto p = vertices((*graph)->graph());
		auto j = std::find_if(p.first,p.second,[&](dot::graph_traits<SugiyamaInterface>::node_type n) { return get<1>(get_vertex(n,(*graph)->graph())) == i; });
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

void Sugiyama::positionEnds(QObject* itm, QQuickItem* head, QQuickItem* tail, QQuickItem* from, QQuickItem* to, const QPainterPath& path)
{
	if(head)
	{
		QRectF bb = head->boundingRect();
		QLineF vec = contactVector(to,path);
		QPointF pos(vec.p1() - QPointF(bb.width() / 2,bb.height() / 2));

		head->setX(pos.x());
		head->setY(pos.y());
		head->setRotation(90 - vec.angle());
	}

	if(tail)
	{
		QRectF bb = tail->boundingRect();
		QLineF vec = contactVector(from,path);
		QPointF pos(vec.p1() - QPointF(bb.width() / 2,bb.height() / 2));

		tail->setX(pos.x());
		tail->setY(pos.y());
		tail->setRotation(90 - vec.angle());
	}
}

QLineF Sugiyama::contactVector(QQuickItem *itm, const QPainterPath& path) const
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
				QPointF b_ptn = path.pointAtPercent(p);
				QLineF normal = ln.normalVector().translated(b_ptn - ln.p1());
				QPointF cut_ptn;

				ensure(ln.intersect(normal,&cut_ptn) != QLineF::NoIntersection);
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
		QLineF vec = QLineF::fromPolar(1,path.angleAtPercent(t));
		QPointF p1 = path.pointAtPercent(t);

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

qreal Sugiyama::approximateDistance(const QPointF &pnt, const QPainterPath& path) const
{
	qreal dist = std::numeric_limits<qreal>::max();
	std::function<qreal(qreal,qreal)> iter;
	iter = [&](qreal from, qreal to)
	{
		qreal len = to - from;
		qreal mid = from + len / 2.0;

		if(len < 0.001f)
			return mid;

		qreal left = QLineF(pnt,path.pointAtPercent(from + len / 4)).length();
		qreal right = QLineF(pnt,path.pointAtPercent(to - len / 4)).length();

		dist = std::min(std::min(dist,left),right);

		if(left < right)
			return iter(from,mid);
		else
			return iter(mid,to);
	};

	iter(0,1);
	return dist;
}

#include <QtConcurrentRun>

#include "sugiyama.hh"
#include "dot/dot.hh"

const int Sugiyama::delta = 3;

Sugiyama::Sugiyama(QQuickItem *parent)
: QQuickPaintedItem(parent),
	_delegate(nullptr), _vertices(), _edges(), _direct(false),
	_graph(), _mapper(), _layoutWatcher(), _routeWatcher()
{
	connect(&_mapper,SIGNAL(mapped(QObject*)),this,SLOT(updateEdge(QObject*)));
	connect(&_layoutWatcher,
					SIGNAL(finished()),
					this,
					SLOT(processLayout()));
	connect(&_routeWatcher,
					SIGNAL(finished()),
					this,
					SLOT(processRoute()));
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
		if(direct())
		{
			for(auto e: iters(po::edges(graph())))
			{
				auto from = po::source(e,graph()), to = po::target(e,graph());
				auto from_obj = std::get<1>(get_vertex(from,graph())), to_obj = std::get<1>(get_vertex(to,graph()));

				QPainterPath pp = toBezier({
					point{from,point::Exit,static_cast<int>(from_obj->x() + from_obj->width( )/ 2),
										 static_cast<int>(from_obj->y() + from_obj->height() + delta)},
					point{to,point::Entry,static_cast<int>(to_obj->x() + to_obj->width() / 2),
									 static_cast<int>(to_obj->y() - delta)}});

				std::get<1>(get_edge(e,graph())) = pp;
				auto ee = get_edge(e,graph());
				positionEnds(std::get<2>(ee),std::get<3>(ee),std::get<1>(get_vertex(source(e,graph()),graph())),std::get<1>(get_vertex(target(e,graph()),graph())),pp);
			}

			emit routingDone();
			update();
		}
		else
		{
			_routeWatcher.cancel();

			std::unordered_map<itmgraph::vertex_descriptor,QRect> bbs;
			for(auto _vx: iters(po::vertices(graph())))
			{
				auto vx = std::get<1>(get_vertex(_vx,graph()));
				bbs.emplace(_vx,QRect(vx->x(),vx->y(),vx->width(),vx->height()));
			}

			itmgraph g = graph();
			_routeWatcher.setFuture(QtConcurrent::run(std::bind(doRoute,g,bbs)));
		}
	}
}

void Sugiyama::layout(void)
{
	if(po::num_edges(graph()))
	{
		emit layoutStart();
		_layoutWatcher.cancel();

		std::unordered_map<itmgraph::vertex_descriptor,int> widths;
		for(auto vx: iters(po::vertices(graph())))
			widths.emplace(vx,std::get<1>(get_vertex(vx,graph()))->width());

		itmgraph g = graph();
		_layoutWatcher.setFuture(QtConcurrent::run(std::bind(doLayout,g,100,widths)));
	}
}

void Sugiyama::processLayout(void)
{
	for(auto p: _layoutWatcher.future().result())
	{
		auto vx = get_vertex(p.first,graph());
		std::get<2>(vx)->setContextProperty("firstRank",QVariant(std::get<0>(p.second)));
		std::get<2>(vx)->setContextProperty("lastRank",QVariant(std::get<1>(p.second)));
		std::get<2>(vx)->setContextProperty("computedX",QVariant(std::get<2>(p.second)));
	}

	emit layoutDone();
	route();
}

void Sugiyama::processRoute(void)
{
	std::unordered_map<itmgraph::edge_descriptor,QPainterPath> r = _routeWatcher.future().result();
	for(auto e: iters(po::edges(graph())))
	{
		std::get<1>(get_edge(e,graph())) = r.at(e);
		auto ee = get_edge(e,graph());
		positionEnds(std::get<2>(ee),std::get<3>(ee),std::get<1>(get_vertex(source(e,graph()),graph())),std::get<1>(get_vertex(target(e,graph()),graph())),r.at(e));
	}

	emit routingDone();
	update();
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

itmgraph& Sugiyama::graph(void)
{
	if(!_graph)
	{
		_graph = itmgraph();

		QListIterator<QVariant> i(_vertices);
		while(i.hasNext())
		{
			QVariant var = i.next();
			QQuickItem *itm = 0;
			QQmlContext *ctx = 0;

			if(_delegate)
			{
				ctx = new QQmlContext(QQmlEngine::contextForObject(this));
				ctx->setContextProperty("modelData",var);
				ctx->setContextProperty("incomingEdges",QVariantList());
				ctx->setContextProperty("incomingNodes",QVariantList());
				ctx->setContextProperty("outgoingNodes",QVariantList());
				ctx->setContextProperty("outgoingEdges",QVariantList());
				ctx->setContextProperty("firstRank",QVariant());
				ctx->setContextProperty("lastRank",QVariant());
				ctx->setContextProperty("computedX",QVariant());
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
	using vx_desc = boost::graph_traits<itmgraph>::vertex_descriptor;
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
				std::tuple<QVariant,QPainterPath,QQuickItem*,QQuickItem*,QQmlContext*,QQmlContext*> &t = get_edge(ex,*_graph);
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
			QQmlContext *hctx = 0, *tctx = 0;

			if(hc)
			{
				hctx = new QQmlContext(QQmlEngine::contextForObject(this));
				hctx->setContextProperty("edge",obj);
				h = qobject_cast<QQuickItem*>(hc->create(hctx));
				h->setParentItem(this);
			}

			if(tc)
			{
				tctx = new QQmlContext(QQmlEngine::contextForObject(this));
				tctx->setContextProperty("edge",obj);
				t = qobject_cast<QQuickItem*>(tc->create(tctx));
				t->setParentItem(this);
			}

			insert_edge(std::make_tuple(var,QPainterPath(),h,t,hctx,tctx),*a,*b,*_graph);
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
	p->setRenderHints(QPainter::Antialiasing | QPainter::TextAntialiasing,true);

	for(auto e: iters(po::edges(graph())))
	{
		auto t = get_edge(e,graph());
		QObject *obj = get<0>(t).value<QObject*>();
		QQmlProperty width(obj,"width");
		QQmlProperty color(obj,"color");
		QPen pen(QBrush(color.read().value<QColor>()),width.read().toInt());

		pen.setCosmetic(true);
		p->setPen(pen);
		p->drawPath(get<1>(t));
	}

	p->restore();
}

void Sugiyama::redoAttached(void)
{
	for(auto vx: iters(po::vertices(graph())))
	{
		QVariantList incomingEdges;
		QVariantList incomingNodes;
		QVariantList outgoingEdges;
		QVariantList outgoingNodes;

		for(auto e: iters(po::in_edges(vx,graph())))
		{
			auto ed = get_edge(e,graph());
			incomingEdges.append(get<0>(ed));
			auto wx = get_vertex(source(e,graph()),graph());
			incomingNodes.append(QVariant::fromValue(get<1>(wx)));
		}

		for(auto e: iters(po::out_edges(vx,graph())))
		{
			auto ed = get_edge(e,graph());
			outgoingEdges.append(get<0>(ed));
			auto wx = get_vertex(target(e,graph()),graph());
			outgoingNodes.append(QVariant::fromValue(get<1>(wx)));
		}

		auto v = get_vertex(vx,graph());
		get<2>(v)->setContextProperty("incomingEdges",QVariant(incomingEdges));
		get<2>(v)->setContextProperty("incomingNodes",QVariant(incomingNodes));
		get<2>(v)->setContextProperty("outgoingEdges",QVariant(outgoingEdges));
		get<2>(v)->setContextProperty("outgoingNodes",QVariant(outgoingNodes));
	}
}

void Sugiyama::positionEnds(QQuickItem* head, QQuickItem* tail, QQuickItem* from, QQuickItem* to, const QPainterPath& path)
{
	if(head)
	{
		QRectF bb = head->boundingRect();
		QRectF to_bb(QQuickPaintedItem::mapFromItem(to,to->boundingRect().topLeft()),QSizeF(to->width(),to->height()));
		QLineF vec = contactVector(to_bb,path);
		QPointF pos(vec.p1() - QPointF(bb.width() / 2,bb.height() / 2));

		head->setX(pos.x());
		head->setY(pos.y());
		head->setRotation(90 - vec.angle());
	}

	if(tail)
	{
		QRectF bb = tail->boundingRect();
		QRectF from_bb(QQuickPaintedItem::mapFromItem(from,from->boundingRect().topLeft()),QSizeF(from->width(),from->height()));
		QLineF vec = contactVector(from_bb,path);
		QPointF pos(vec.p1() - QPointF(bb.width() / 2,bb.height() / 2));

		tail->setX(pos.x());
		tail->setY(pos.y());
		tail->setRotation(90 - vec.angle());
	}
}

QLineF contactVector(QRectF const& bb, const QPainterPath& path)
{
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

qreal approximateDistance(const QPointF &pnt, const QPainterPath& path)
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


std::unordered_map<itmgraph::vertex_descriptor,std::tuple<unsigned int,unsigned int,unsigned int>>
doLayout(itmgraph graph, unsigned int nodesep, std::unordered_map<itmgraph::vertex_descriptor,int> widths)
{
	auto pos = dot::layout(graph);
	auto xpos = dot::order(pos,widths,nodesep,graph);
	std::unordered_map<itmgraph::vertex_descriptor,std::tuple<unsigned int,unsigned int,unsigned int>> ret;

	for(auto p: pos)
	{
		ret.emplace(p.first,std::make_tuple(std::get<0>(p.second),std::get<0>(p.second),xpos.at(p.first)));
	}

	return ret;
}

std::unordered_map<itmgraph::edge_descriptor,QPainterPath>
doRoute(itmgraph graph, std::unordered_map<itmgraph::vertex_descriptor,QRect> bboxes)
{
	std::unordered_set<point> points;
	visgraph vis;

	for(auto desc: iters(po::vertices(graph)))
	{
		auto bb = bboxes.at(desc);
		QPoint pos = bb.topLeft();
		QSize sz = bb.size();

		points.insert(point{desc,point::Entry,pos.x() + sz.width() / 2,pos.y() - 3*Sugiyama::delta});
		points.insert(point{desc,point::Exit,pos.x() + sz.width() / 2,pos.y() + sz.height() + 3*Sugiyama::delta});

		points.insert(point{desc,point::Corner,pos.x() - Sugiyama::delta,pos.y() - Sugiyama::delta});
		points.insert(point{desc,point::Corner,pos.x() - Sugiyama::delta,pos.y() + sz.height() + Sugiyama::delta});
		points.insert(point{desc,point::Corner,pos.x() + sz.width() + Sugiyama::delta,pos.y() - Sugiyama::delta});
		points.insert(point{desc,point::Corner,pos.x() + sz.width() + Sugiyama::delta,pos.y() + sz.height() + Sugiyama::delta});
	}

	// find edges
	for(auto from: points)
	{
		for(auto to: points)
		{
			QPoint from_pos(from.x,from.y);
			QPoint to_pos(to.x,to.y);

			if(to.type != point::Exit && from.type != point::Entry)
			{
				bool add = true;
				QLineF l(from_pos,to_pos);

				for(auto wx: iters(po::vertices(graph)))
				{
					QRectF bb = bboxes.at(wx);
					QPointF c;

					if(l.intersect(QLineF(bb.topLeft(),bb.topRight()),&c) == QLineF::BoundedIntersection ||
						 l.intersect(QLineF(bb.topRight(),bb.bottomRight()),&c) == QLineF::BoundedIntersection ||
						 l.intersect(QLineF(bb.bottomRight(),bb.bottomLeft()),&c) == QLineF::BoundedIntersection ||
						 l.intersect(QLineF(bb.bottomLeft(),bb.topLeft()),&c) == QLineF::BoundedIntersection)
					{
						add = false;
						break;
					}
				}

				if(add)
				{
					vis.insert(std::make_pair(from,to));
					//vis.insert(std::make_pair(to,from));
				}
			}
		}
	}

	std::unordered_map<itmgraph::edge_descriptor,QPainterPath> ret;

	for(auto e: iters(po::edges(graph)))
	{
		auto from = po::source(e,graph);
		auto to = po::target(e,graph);

		auto from_bb = bboxes.at(from);
		auto to_bb = bboxes.at(to);

		QPoint from_pos = from_bb.topLeft();
		QPoint to_pos = to_bb.topLeft();

		QSize from_sz = from_bb.size();
		QSize to_sz = to_bb.size();

		auto r = dijkstra(point{from,point::Exit,from_pos.x() + from_sz.width() / 2,from_pos.y() + from_sz.height() + 3*Sugiyama::delta},
											point{to,point::Entry,to_pos.x() + to_sz.width() / 2,to_pos.y() - 3*Sugiyama::delta},vis);
		r.push_front(point{from,point::Center,from_pos.x() + from_sz.width() / 2,from_pos.y() + from_sz.height() / 2});
		r.push_back(point{to,point::Center,to_pos.x() + to_sz.width() / 2,to_pos.y() + to_sz.height() / 2});
		if(r.empty())
		{
			qWarning() << "No route from" << from_pos << "to" << to_pos;
			ret.emplace(e,QPainterPath());
		}
		else
		{
			QPainterPath pp = toBezier(r);
			ret.emplace(e,pp);
		}
	}

	return ret;
}

std::list<point> dijkstra(point start, point goal, visgraph const& graph)
{
	std::unordered_map<point,double> distance;
	std::unordered_set<point> worklist;
	std::unordered_map<point,point> came_from;
	std::list<point> ret({goal});

	std::transform(graph.begin(),graph.end(),std::inserter(worklist,worklist.end()),[](const std::pair<point,point>& p) { return p.first; });
	std::transform(graph.begin(),graph.end(),std::inserter(worklist,worklist.end()),[](const std::pair<point,point>& p) { return p.second; });
	distance.insert(std::make_pair(start,0));

	Q_ASSERT(graph.count(start));
	//Q_ASSERT(graph.count(goal));

	for(auto w: worklist)
		distance.insert(std::make_pair(w,std::numeric_limits<double>::infinity()));

	while(!worklist.empty())
	{
		auto it = std::min_element(worklist.begin(),worklist.end(),[&](point a,point b)
				{ return distance.at(a) < distance.at(b); });
		auto vx = *it;

		worklist.erase(it);

		for(auto succ: po::iters(graph.equal_range(vx)))
		{
			if(succ.second != vx && worklist.count(succ.second))
			{
				double edge_cost = std::sqrt(std::pow(std::abs(succ.second.x - vx.x),2) + std::pow(std::abs(succ.second.y - vx.y),2));
				double cum_cost = distance.at(vx) + edge_cost;

				if(!distance.count(succ.second) || distance.at(succ.second) > cum_cost)
				{
					distance[succ.second] = cum_cost;
					came_from[succ.second] = vx;
				}
			}
		}
	}

	if(came_from.count(goal))
	{
		while(ret.front() != start)
			ret.push_front(came_from.at(ret.front()));
		return ret;
	}
	else
		return {};
}

QPainterPath toBezier(const std::list<point> &segs)
{
	QPainterPath pp;

	// draw segments with bezier curves
	if(segs.size() > 2)
	{
		std::list<qreal> angles;
		auto d = std::next(segs.begin());
		QPointF f1(segs.front().x,segs.front().y);
		QPointF f2(std::next(segs.begin())->x,std::next(segs.begin())->y);

		angles.push_back(QLineF(f1,f2).angle());
		while(d != std::prev(segs.end()))
		{
			QPointF a(std::prev(d)->x,std::prev(d)->y);
			QPointF b(d->x,d->y);
			QPointF c(std::next(d)->x,std::next(d)->y);

			QLineF ln(a,b);
			angles.push_back(ln.angle() + ln.angleTo(QLineF(b,c)) / 2.0);
			++d;
		}

		QPointF x(std::prev(segs.end(),2)->x,std::prev(segs.end(),2)->y);
		QPointF y(std::prev(segs.end())->x,std::prev(segs.end())->y);
		angles.push_back(QLineF(x,y).angle());

		ensure(angles.size() == segs.size());

		size_t idx = 0;
		while(idx < segs.size() - 1)
		{
			QPointF ptn1(std::next(segs.begin(),idx)->x,std::next(segs.begin(),idx)->y);
			qreal alpha1 = *std::next(angles.begin(),idx);

			QPointF ptn2(std::next(segs.begin(),idx + 1)->x,std::next(segs.begin(),idx + 1)->y);
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
		QPointF p1(segs.front().x,segs.front().y), p2(segs.back().x,segs.back().y);

		pp.moveTo(p1);
		pp.lineTo(p2);
	}

	return pp;
}

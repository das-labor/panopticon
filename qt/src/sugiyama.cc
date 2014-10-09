#include "sugiyama.hh"

#include "dot/dot.hh"

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
		if(direct())
		{
			for(auto e: iters(po::edges(graph())))
			{
				auto from = po::source(e,graph()), to = po::target(e,graph());
				auto from_obj = std::get<1>(get_vertex(from,graph())), to_obj = std::get<1>(get_vertex(to,graph()));

				QPainterPath pp = to_bezier({
					point{from,true,static_cast<int>(from_obj->x() + from_obj->width( )/ 2),
										 static_cast<int>(from_obj->y() + from_obj->height() / 2)},
					point{to,true,static_cast<int>(to_obj->x() + to_obj->width() / 2),
									 static_cast<int>(to_obj->y() + to_obj->height() / 2)}});

				std::get<1>(get_edge(e,graph())) = pp;
			}
		}
		else
		{
			std::unordered_set<point> points;

			for(auto desc: iters(po::vertices(graph())))
			{
				auto vx = get_vertex(desc,graph());
				QPoint pos(std::get<1>(vx)->x(),std::get<1>(vx)->y());
				QSize sz(std::get<1>(vx)->width(),std::get<1>(vx)->height());
				const int delta = 3;

				points.insert(point{desc,true,pos.x() + sz.width() / 2,pos.y() + sz.height() / 2});
				points.insert(point{desc,false,pos.x() - delta,pos.y() - delta});
				points.insert(point{desc,false,pos.x() - delta,pos.y() + sz.height() + delta});
				points.insert(point{desc,false,pos.x() + sz.width() + delta,pos.y() - delta});
				points.insert(point{desc,false,pos.x() + sz.width() + delta,pos.y() + sz.height() + delta});
			}

			visgraph vis;
			_visgraph.clear();

			// find edges
			for(auto from: points)
			{
				for(auto to: points)
				{
					if(from != to)
					{
						QPoint from_pos(from.x,from.y);
						QPoint to_pos(to.x,to.y);
						auto from_obj = std::get<1>(get_vertex(from.node,graph()));
						auto to_obj = std::get<1>(get_vertex(to.node,graph()));
						QRect from_bb(from_obj->x(),from_obj->y(),from_obj->width(),from_obj->height());
						QRect to_bb(to_obj->x(),to_obj->y(),to_obj->width(),to_obj->height());

						if(from.is_center == to.is_center || from.node != to.node)
						{
							bool add = true;
							QLineF l(from_pos,to_pos);

							for(auto _wx: iters(po::vertices(graph())))
							{
								auto wx = std::get<1>(get_vertex(_wx,graph()));
								QPointF pos(wx->x(),wx->y());
								QRectF bb(pos,QSizeF(wx->width(),wx->height()));
								QPointF c;

								if(!(from.is_center && from_obj == wx) &&
									 !(to.is_center && to_obj == wx) &&
									 (l.intersect(QLineF(bb.topLeft(),bb.topRight()),&c) == QLineF::BoundedIntersection ||
									  l.intersect(QLineF(bb.topRight(),bb.bottomRight()),&c) == QLineF::BoundedIntersection ||
									  l.intersect(QLineF(bb.bottomRight(),bb.bottomLeft()),&c) == QLineF::BoundedIntersection ||
									  l.intersect(QLineF(bb.bottomLeft(),bb.topLeft()),&c) == QLineF::BoundedIntersection))
								{
									add = false;
									break;
								}
							}

							if(add)
							{
								vis.emplace(from,to);
								vis.emplace(to,from);
								_visgraph.push_back(QLine(from.x,from.y,to.x,to.y));
							}
						}
					}
				}
			}

			std::cout << vis.size() << std::endl;

			for(auto e: iters(po::edges(graph())))
			{
				auto from = po::source(e,graph());
				auto to = po::target(e,graph());

				auto from_obj = std::get<1>(get_vertex(from,graph()));
				auto to_obj = std::get<1>(get_vertex(to,graph()));

				QPoint from_pos(from_obj->x(),from_obj->y());
				QPoint to_pos(to_obj->x(),to_obj->y());

				QSize from_sz(from_obj->width(),from_obj->height());
				QSize to_sz(to_obj->width(),to_obj->height());

				std::cout << "route " << from.id << " -> " << to.id << ": ";
				auto route = astar(point{from,true,from_pos.x() + from_sz.width() / 2,from_pos.y() + from_sz.height() / 2},
													 point{to,true,to_pos.x() + to_sz.width() / 2,to_pos.y() + to_sz.height() / 2},vis);

				for(auto c: route)
					std::cout << c.node.id << ", ";
				std::cout << std::endl;

				QPainterPath pp = to_bezier(route);
				auto p = get_edge(e,graph());
				std::get<1>(get_edge(e,graph())) = pp;
				positionEnds(std::get<0>(p).value<QObject*>(),get<2>(p),get<3>(p),from_obj,to_obj,pp);
			}
		}

		update();

		//emit routingDone();
	}
}

void Sugiyama::layout(void)
{
	if(po::num_edges(graph()))
	{
		emit layoutStart();
		auto pos = dot::layout(graph());
		std::unordered_map<decltype(_graph)::value_type::vertex_descriptor,int> width;

		for(auto v: iters(po::vertices(graph())))
			width.emplace(v,std::get<1>(get_vertex(v,graph()))->width());

		auto xpos = dot::order(pos,width,100,graph());
		//po::digraph<std::tuple<QVariant,QQuickItem*,QQmlContext*>,std::tuple<QVariant,QPainterPath,QQuickItem*,QQuickItem*>>& graph(void);

		for(auto p: pos)
		{
			std::cout << p.first.id << " xpos: " << xpos.at(p.first) << std::endl;
			auto vx = get_vertex(p.first,graph());
			std::get<2>(vx)->setContextProperty("firstRank",QVariant(std::get<0>(p.second)));
			std::get<2>(vx)->setContextProperty("lastRank",QVariant(std::get<1>(p.second)));
			std::get<2>(vx)->setContextProperty("computedX",QVariant(xpos.at(p.first)));
		}

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

	QPen pen(QBrush(QColor("red")),1);
	p->setPen(pen);

	/*for(auto q: _visgraph)
		p->drawLine(q);*/

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

std::list<point> Sugiyama::astar(point start, point goal, visgraph const& graph)
{
	std::unordered_map<point,double> distance;
	//std::unordered_map<visgraph::const_iterator,visgraph::const_iterator> path_ptr;
	std::unordered_set<point> worklist;
	//std::unordered_set<visgraph::const_iterator> closedlist;
	std::unordered_map<point,point> came_from;
	std::list<point> ret({goal});

	/*coord from_pos = position(source(e,tag),tag), to_pos = position(sink(e,tag),tag);
	std::pair<unsigned int,unsigned int> from_sz = dimensions(source(e,tag),tag), to_sz = dimensions(sink(e,tag),tag);
	vis_node<T> start(std::make_pair(from_pos.first + from_sz.first / 2,from_pos.second + from_sz.second / 2),source(e,tag));
	vis_node<T> finish(std::make_pair(to_pos.first + to_sz.first / 2,to_pos.second + to_sz.second / 2),sink(e,tag));*/

	std::transform(graph.begin(),graph.end(),std::inserter(worklist,worklist.end()),[](const std::pair<point,point>& p) { return p.first; });
	std::transform(graph.begin(),graph.end(),std::inserter(worklist,worklist.end()),[](const std::pair<point,point>& p) { return p.second; });
	distance.insert(std::make_pair(start,0));

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
			if((!succ.second.is_center || succ.second == goal) && succ.second != vx && worklist.count(succ.second))
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
		while(ret.front() != start)
			ret.push_front(came_from.at(ret.front()));

	return ret;
}

QPainterPath Sugiyama::to_bezier(const std::list<point> &segs)
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

/*
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
	q->setX(pos.first - q->width() / 2);
	q->setY(pos.second - q->height() / 2);
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
}*/

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

#include <QtConcurrent>

#include "dot/dot.hh"

#include "sugiyama.hh"

const int Sugiyama::nodeBorderPadding = 20;
const int Sugiyama::edgeRadius = 15;
const int Sugiyama::nodePortPadding = 10;

Procedure::Procedure(QObject* p)
: QObject(p), _procedure(boost::none)
{}

Procedure::~Procedure(void) {}

Sugiyama::Sugiyama(QQuickItem* p)
: _vertex(nullptr), _edge(nullptr), _procedure(nullptr),
	_cache(), _mapper(), _layoutWatcher(), _routeWatcher()
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
	connect(this,SIGNAL(vertexChanged()),this,SLOT(layout()));
	connect(this,SIGNAL(procedureChanged()),this,SLOT(layout()));
}

void Sugiyama::setVertex(QQmlComponent* c)
{
	if(_vertex)
		_vertex->deleteLater();
	_vertex = c;
	emit vertexChanged();
}

void Sugiyama::setEdge(QQmlComponent* c)
{
	if(_edge)
		_edge->deleteLater();
	_edge = c;
	emit edgeChanged();
}

void Sugiyama::setProcedure(QObject* o)
{
	Procedure* proc = qobject_cast<Procedure*>(o);
	boost::optional<bool> next(boost::none);

	{
		std::lock_guard<std::mutex> guard(_mutex);

		if(proc && proc != _procedure)
		{
			std::function<void(Procedure*,bool)> f = [&](Procedure* p, bool v)
			{
				if(p && p->procedure())
				{
					auto i = _cache.find(*p->procedure());

					if(i != _cache.end())
					{
						for(auto vx: iters(vertices(std::get<0>(i->second))))
						{
							get_vertex(vx,std::get<0>(i->second)).item->setVisible(v);
						}

						for(auto ed: iters(edges(std::get<0>(i->second))))
						{
							auto edge = get_edge(ed,std::get<0>(i->second));

							if(edge.edge)
								edge.edge->setVisible(v);
						}
					}
				}
			};

			f(_procedure,false);
			f(proc,true);

			_procedure = proc;
			next = (_procedure && _procedure->procedure() && !_cache.count(*_procedure->procedure()));

			emit procedureChanged();
		}
	}

	if(next)
	{
		if(*next)
		{
			layout();
		}
		else
		{
			update();
		}
	}
}

void Sugiyama::paint(QPainter* p)
{
	ensure(p);
	p->save();
	p->setRenderHints(QPainter::Antialiasing | QPainter::TextAntialiasing,true);

	std::lock_guard<std::mutex> guard(_mutex);

	if(_procedure && _procedure->procedure() && _cache.count(*_procedure->procedure()))
	{
		boost::optional<route_type> const& r = std::get<2>(_cache.at(*_procedure->procedure()));
		itmgraph const& graph = std::get<0>(_cache.at(*_procedure->procedure()));

		if(r)
		{
			for(auto e: *r)
			{
				auto t = get_edge(e.first,graph);
				QObject *obj = t.edge;
				QQmlProperty lineWidth(obj,"lineWidth");
				QQmlProperty color(obj,"color");
				QPen pen(QBrush(color.read().value<QColor>()),lineWidth.read().toInt());

				pen.setCosmetic(true);
				p->setPen(pen);
				p->drawPath(e.second.first);
			}
		}
	}

	p->restore();
}

std::pair<po::proc_wloc,std::unordered_map<itmgraph::vertex_descriptor,std::tuple<unsigned int,unsigned int,unsigned int>>>
doLayout(itmgraph graph, unsigned int nodesep, std::unordered_map<itmgraph::vertex_descriptor,int> widths, po::proc_wloc proc)
{
	auto pos = dot::layout(graph);
	auto xpos = dot::order(pos,widths,nodesep,graph);
	std::unordered_map<itmgraph::vertex_descriptor,std::tuple<unsigned int,unsigned int,unsigned int>> ret;

	for(auto p: pos)
	{
		ret.emplace(p.first,std::make_tuple(std::get<0>(p.second),std::get<0>(p.second),xpos.at(p.first)));
	}
	return std::make_pair(proc,ret);
}

void Sugiyama::layout(void)
{
	std::lock_guard<std::mutex> guard(_mutex);

	if(_procedure)
	{
		_layoutWatcher.cancel();
		try
		{
			_layoutWatcher.waitForFinished();
		}
		catch(...)
		{
			;
		}

		emit layoutStart();

		boost::optional<po::proc_loc> maybe_proc =_procedure->procedure();

		if(maybe_proc)
		{
			po::proc_loc proc = *maybe_proc;
			auto i = _cache.find(proc);

			if(i == _cache.end())
			{
				std::unordered_map<po::procedure::graph_type::vertex_descriptor,itmgraph::vertex_descriptor> vx_map;
				itmgraph g;
				po::procedure::graph_type const& h = proc->control_transfers;

				for(auto vx: iters(vertices(h)))
				{
					node_proxy np(_vertex,this);

					struct vis : boost::static_visitor<QString>
					{
						QString operator()(po::bblock_loc bb) const
						{
							QStringList payload;

							for(auto mne: bb->mnemonics())
							{
								QStringList ops;
								for(auto q: mne.operands)
								{
									std::stringstream ss;
									ss << q;
									ops.append("'" + QString::fromStdString(ss.str()) + "'");
								}

								payload.append(QString("{ \"opcode\": \"%1\", \"operands\": [%2] }")
										.arg(QString::fromStdString(mne.opcode))
										.arg(ops.join(", ")));
							}

							return QString("{ \"type\":\"bblock\", \"payload\": [%1] }").arg(payload.join(", "));
						}

						QString operator()(po::rvalue v) const
						{
							std::stringstream ss;
							ss << v;

							return QString("{ \"type\": \"value\", \"payload\": '%1' }").arg(QString::fromStdString(ss.str()));
						}
					};

					QString p = boost::apply_visitor(vis(),get_vertex(vx,h));

					np.context->setContextProperty("payload",QVariant(p));
					vx_map.emplace(vx,insert_vertex(np,g));
				}

				std::list<itmgraph::edge_descriptor> to_proc;

				for(auto ed: iters(edges(h)))
				{
					auto from = vx_map.at(source(ed,h));
					auto to = vx_map.at(target(ed,h));
					to_proc.emplace_back(insert_edge(edge_proxy(_edge,this),from,to,g));
				}

				_cache.emplace(proc,cache_type(g,boost::none,boost::none));

				for(auto e: to_proc)
					updateEdgeDecorations(e,_cache.at(proc));

				i = _cache.find(proc);

			}

			ensure(i != _cache.end());

			if(std::get<1>(i->second) == boost::none)
			{
				std::unordered_map<itmgraph::vertex_descriptor,int> widths;
				for(auto vx: iters(po::vertices(std::get<0>(i->second))))
					widths.emplace(vx,get_vertex(vx,std::get<0>(i->second)).item->width());

				_layoutWatcher.setFuture(QtConcurrent::run(std::bind(doLayout,std::get<0>(i->second),100,widths,po::proc_wloc(i->first))));
			}
			else
			{
				for(auto vx: iters(vertices(std::get<0>(i->second))))
				{
					get_vertex(vx,std::get<0>(i->second)).item->setVisible(true);
				}
			}
		}
	}
}

void Sugiyama::updateEdgeDecorations(itmgraph::edge_descriptor e, Sugiyama::cache_type& cache)
{
	edge_proxy& px = get_edge(e,std::get<0>(cache));
	QObject* obj = px.edge;

	if(obj)
	{
		QQmlProperty from(obj,"from");
		QQmlProperty to(obj,"to");
		QQmlProperty lineWidth(obj,"lineWidth");
		QQmlProperty color(obj,"color");
		QQmlProperty head(obj,"head");
		QQmlProperty tail(obj,"tail");
		QQmlProperty label(obj,"label");

		QQmlComponent *hc = head.read().value<QQmlComponent*>();
		QQmlComponent *tc = tail.read().value<QQmlComponent*>();
		QQmlComponent *lc = label.read().value<QQmlComponent*>();

		if(px.head)
		{
			px.head->deleteLater();
		}

		if(px.tail)
		{
			px.tail->deleteLater();
		}

		if(px.label)
			px.label->deleteLater();

		if(hc)
		{
			px.head_context = new QQmlContext(QQmlEngine::contextForObject(obj));
			px.head_context->setContextProperty("edge",obj);
			px.head = qobject_cast<QQuickItem*>(hc->create(px.head_context));
			px.head->setParentItem(px.edge);
		}
		else
		{
			px.head = nullptr;
			px.head_context = nullptr;
		}

		if(tc)
		{
			px.tail_context = new QQmlContext(QQmlEngine::contextForObject(obj));
			px.tail_context->setContextProperty("edge",obj);
			px.tail = qobject_cast<QQuickItem*>(tc->create(px.tail_context));
			px.tail->setParentItem(px.edge);
		}
		else
		{
			px.tail = nullptr;
			px.tail_context = nullptr;
		}

		if(lc)
		{
			px.label_context = new QQmlContext(QQmlEngine::contextForObject(obj));
			px.label_context->setContextProperty("edge",obj);
			px.label = qobject_cast<QQuickItem*>(lc->create(px.label_context));
			px.label->setParentItem(px.edge);
		}
		else
		{
			px.label = nullptr;
			px.label_context = nullptr;
		}

		ensure(lineWidth.connectNotifySignal(this,SLOT(update())));
		ensure(color.connectNotifySignal(this,SLOT(update())));
		ensure(from.connectNotifySignal(&_mapper,SLOT(map())));
		ensure(to.connectNotifySignal(&_mapper,SLOT(map())));
		ensure(head.connectNotifySignal(&_mapper,SLOT(map())));
		ensure(tail.connectNotifySignal(&_mapper,SLOT(map())));
		ensure(label.connectNotifySignal(&_mapper,SLOT(map())));
	}
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

			qreal omega = std::min(QLineF(ptn1,ptn2).length() / 20.0,100.0);
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

std::pair<int,int>
nodePorts(itmgraph::edge_descriptor e, boost::optional<std::unordered_map<itmgraph::vertex_descriptor,QRect>> bboxes, itmgraph const& graph)
{
	std::function<QRect(itmgraph::vertex_descriptor)> get_bb = [&](itmgraph::vertex_descriptor _v)
	{
		if(bboxes)
		{
			return bboxes->at(_v);
		}
		else
		{
			auto v = get_vertex(_v,graph);
			return QRect(v.item->x(),v.item->y(),v.item->width(),v.item->height());
		}
	};

	auto from = po::source(e,graph);
	auto to = po::target(e,graph);

	auto from_bb = get_bb(from);
	auto to_bb = get_bb(to);

	if(from == to)
		return std::make_pair(to_bb.left() + 20,to_bb.left() + 20);

	QPoint from_pos = from_bb.topLeft();
	QPoint to_pos = to_bb.topLeft();

	QSize from_sz = from_bb.size();
	QSize to_sz = to_bb.size();

	std::list<itmgraph::edge_descriptor> in_e, out_e;
	auto in_e_p = in_edges(to,graph);
	auto out_e_p = out_edges(from,graph);

	std::copy(in_e_p.first,in_e_p.second,std::back_inserter(in_e));
	std::copy(out_e_p.first,out_e_p.second,std::back_inserter(out_e));

	in_e.sort([&](itmgraph::edge_descriptor a, itmgraph::edge_descriptor b)
		{ return get_bb(po::source(a,graph)).topLeft().x() < get_bb(po::source(b,graph)).topLeft().x(); });
	out_e.sort([&](itmgraph::edge_descriptor a, itmgraph::edge_descriptor b)
		{ return get_bb(po::target(a,graph)).topLeft().x() < get_bb(po::target(b,graph)).topLeft().x(); });

	const int in_x_ord = std::distance(in_e.begin(),std::find(in_e.begin(),in_e.end(),e));
	const int out_x_ord = std::distance(out_e.begin(),std::find(out_e.begin(),out_e.end(),e));
	const int indeg = in_degree(to,graph) - 1;
	const int outdeg = out_degree(from,graph) - 1;
	const int in_x = to_pos.x() + to_sz.width() / 2 - (indeg * Sugiyama::nodePortPadding) / 2 + (in_x_ord * Sugiyama::nodePortPadding);
	const int out_x = from_pos.x() + from_sz.width() / 2 - (outdeg * Sugiyama::nodePortPadding) / 2 + (out_x_ord * Sugiyama::nodePortPadding);

	return std::make_pair(out_x,in_x);
}

void Sugiyama::positionEdgeDecoration(itmgraph::edge_descriptor e, cache_type const& cache)
{
	itmgraph const& graph = std::get<0>(cache);
	auto edge = get_edge(e,graph);
	QQuickItem* from = get_vertex(po::source(e,graph),graph).item;
	QQuickItem* to = get_vertex(po::target(e,graph),graph).item;
	auto ports = nodePorts(e,boost::none,graph);
	QQuickItem* head = edge.head;
	QQuickItem* tail = edge.tail;
	QQuickItem* label = edge.label;

	if(head)
	{
		QRectF to_bb(QQuickPaintedItem::mapFromItem(to,to->boundingRect().topLeft()),QSizeF(to->width(),to->height()));

		head->setX(ports.second - head->width() / 2);
		head->setY(to_bb.top() - head->height() / 2);
		head->setRotation(180);
	}

	if(tail)
	{
		QRectF from_bb(QQuickPaintedItem::mapFromItem(from,from->boundingRect().topLeft()),QSizeF(from->width(),from->height()));

		tail->setX(ports.first - head->width() / 2);
		tail->setY(from_bb.bottom());
	}

	if(label)
	{
		QRectF bb = label->boundingRect();
		QPointF pnt = std::get<1>(std::get<2>(cache)->at(e));

		label->setX(pnt.x() - bb.width() / 2);
		label->setY(pnt.y() - bb.height() / 2);
	}
}

QPainterPath toPoly(const std::list<point> &segs)
{
	QPainterPath pp;

	// draw segments with polylines and rounded corners
	if(segs.size() > 2)
	{
		qreal prev_gap = 0;
		int idx = 0;

		pp.moveTo(QPointF(segs.front().x,segs.front().y));
		while(idx < static_cast<int>(segs.size()) - 2)
		{
			QPointF f1(std::next(segs.begin(),idx)->x,std::next(segs.begin(),idx)->y);
			QPointF f2(std::next(segs.begin(),idx + 1)->x,std::next(segs.begin(),idx + 1)->y);
			QPointF f3(std::next(segs.begin(),idx + 2)->x,std::next(segs.begin(),idx + 2)->y);
			QLineF l1(f1,f2), l2(f3,f2);

			const bool dir = l1.angleTo(l2) < l2.angleTo(l1);
			const qreal deg = dir ? l1.angleTo(l2) : l2.angleTo(l1);
			const qreal rad = deg / 360.0f * 44.0f/7.0f;
			const qreal x1 = (Sugiyama::edgeRadius * std::cos(rad/2)) / std::tan(rad/2);
			const qreal x2 = std::sqrt(std::pow(Sugiyama::edgeRadius,2) - std::pow(Sugiyama::edgeRadius * std::cos(rad/2),2));
			const qreal len = x1 + x2;
			const qreal gap = len * std::cos(rad/2);
			const qreal sweep = -std::fmod(l1.angle() - l2.angle() - 180,360);

			QLineF l3 = QLineF::fromPolar(len,deg/2 + (dir ? 180 + l1.angle() : l2.angle() - 180)).translated(f2);
			QRectF bb(l3.p2() - QPointF(Sugiyama::edgeRadius,Sugiyama::edgeRadius),QSizeF(2*Sugiyama::edgeRadius,2*Sugiyama::edgeRadius));

			l1.translate(QLineF::fromPolar(prev_gap,l1.angle()).p2());
			l1.setLength(l1.length() - gap - prev_gap);
			l2.setLength(l2.length() - gap);

			pp.moveTo(l1.p1());
			pp.lineTo(l1.p2());
			pp.arcTo(bb,dir ? l1.angle() + 90 : l1.angle() - 90,sweep > 180 ? sweep - 360 : sweep);

			if(idx + 1 == static_cast<int>(segs.size()) - 2)
			{
				pp.moveTo(l2.p2());
				pp.lineTo(l2.p1());
			}

			idx += 1;
			prev_gap = gap;
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

std::list<point> dijkstra(point start, point goal, visgraph const& graph)
{
	std::unordered_map<point,double> distance;
	std::unordered_set<point> worklist;
	std::unordered_map<point,point> came_from;
	std::list<point> ret({goal});

	std::transform(graph.begin(),graph.end(),std::inserter(worklist,worklist.end()),[](const std::pair<point,point>& p) { return p.first; });
	std::transform(graph.begin(),graph.end(),std::inserter(worklist,worklist.end()),[](const std::pair<point,point>& p) { return p.second; });
	distance.insert(std::make_pair(start,0));

	if(graph.count(start) == 0)
	{
		qWarning() << "Dijkstra: start node not part of graph. No solution possible!";
		return {};
	}

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

std::pair<po::proc_wloc,std::unordered_map<itmgraph::edge_descriptor,std::pair<QPainterPath,QPointF>>>
doRoute(itmgraph graph, std::unordered_map<itmgraph::vertex_descriptor,QRect> bboxes, po::proc_wloc proc)
{
	std::unordered_set<point> points;
	visgraph vis;

	for(auto desc: iters(po::vertices(graph)))
	{
		auto bb = bboxes.at(desc);
		QPoint pos = bb.topLeft();
		QSize sz = bb.size();
		int x_ord = 0;
		auto out = out_edges(desc,graph);
		const int loops = std::count_if(out.first,out.second,[&](itmgraph::edge_descriptor e)
			{ return po::source(e,graph) == po::target(e,graph); });
		const int indeg = in_degree(desc,graph) - loops;
		const int outdeg = out_degree(desc,graph) - loops;

		while(x_ord < indeg)
		{
			points.insert(point{desc,point::Entry,pos.x() + sz.width() / 2 - ((indeg - 1) * Sugiyama::nodePortPadding) / 2 + (x_ord * Sugiyama::nodePortPadding),pos.y() - Sugiyama::nodeBorderPadding});
			++x_ord;
		}

		x_ord = 0;
		while(x_ord < outdeg)
		{
			points.insert(point{desc,point::Exit,pos.x() + sz.width() / 2 - ((outdeg - 1) * Sugiyama::nodePortPadding) / 2 + (x_ord * Sugiyama::nodePortPadding),pos.y() + sz.height() + Sugiyama::nodeBorderPadding});
			++x_ord;
		}

		points.insert(point{desc,point::Corner,pos.x() - Sugiyama::nodeBorderPadding,pos.y() - Sugiyama::nodeBorderPadding});
		points.insert(point{desc,point::Corner,pos.x() - Sugiyama::nodeBorderPadding,pos.y() + sz.height() + Sugiyama::nodeBorderPadding});
		points.insert(point{desc,point::Corner,pos.x() + sz.width() + Sugiyama::nodeBorderPadding,pos.y() - Sugiyama::nodeBorderPadding});
		points.insert(point{desc,point::Corner,pos.x() + sz.width() + Sugiyama::nodeBorderPadding,pos.y() + sz.height() + Sugiyama::nodeBorderPadding});
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
				}
			}
		}
	}

	std::unordered_map<itmgraph::edge_descriptor,std::pair<QPainterPath,QPointF>> ret;

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

		int in_x, out_x;
		std::tie(out_x,in_x) = nodePorts(e,bboxes,graph);
		std::list<point> r;

		if(from != to)
		{
			r = dijkstra(point{from,point::Exit,out_x,from_pos.y() + from_sz.height() + Sugiyama::nodeBorderPadding},
									 point{to,point::Entry,in_x,to_pos.y() - Sugiyama::nodeBorderPadding},vis);
		}
		else
		{
			r = {
				point{to,point::Exit,to_bb.left() + 20,to_pos.y() + to_sz.height() + Sugiyama::nodeBorderPadding/2},
				point{to,point::Corner,to_bb.left() - Sugiyama::nodeBorderPadding/2,to_bb.bottom() + Sugiyama::nodeBorderPadding/2},
				point{to,point::Corner,to_bb.left() -Sugiyama::nodeBorderPadding/2,to_bb.top() -Sugiyama::nodeBorderPadding/2},
				point{to,point::Entry,to_bb.left() + 20,to_pos.y() - Sugiyama::nodeBorderPadding/2}
			};
		}

		if(r.empty())
		{
			qWarning() << "No route from" << from_pos << "to" << to_pos;

			QPainterPath pp;

			pp.moveTo(from_bb.center());
			pp.lineTo(to_bb.center());
			ret.emplace(e,std::make_pair(pp,pp.pointAtPercent(.5)));
		}
		else
		{
			QPointF pnt = toPoly(r).pointAtPercent(.5);
			r.push_front(point{from,point::Center,r.front().x,from_pos.y() + from_sz.height() / 2});
			r.push_back(point{to,point::Center,r.back().x,to_pos.y() + to_sz.height() / 2});

			QPainterPath pp = toPoly(r);
			//QPainterPath pp = toBezier(r);
			ret.emplace(e,std::make_pair(pp,pnt));
		}
	}

	return std::make_pair(proc,ret);
}

void Sugiyama::route(void)
{
	std::lock_guard<std::mutex> guard(_mutex);

	if(_procedure)
	{
		emit routeStart();

		boost::optional<po::proc_loc> maybe_proc =_procedure->procedure();

		if(maybe_proc)
		{
			po::proc_loc proc = *maybe_proc;
			auto i = _cache.find(proc);

			if(i != _cache.end())
			{
				itmgraph const& g = std::get<0>(i->second);
				route_type ret;

				for(auto e: iters(po::edges(g)))
				{
					auto from = po::source(e,g), to = po::target(e,g);
					auto from_obj = get_vertex(from,g).item, to_obj = get_vertex(to,g).item;
					int in_x, out_x;
					std::tie(out_x,in_x) = nodePorts(e,boost::none,g);
					QPainterPath pp;
					int to_x = static_cast<int>(to_obj->x()), to_y = static_cast<int>(to_obj->y());
					int to_height = static_cast<int>(to_obj->height());
					int from_y = static_cast<int>(from_obj->y());
					int from_height = static_cast<int>(from_obj->height());

					if(from == to)
					{
						pp = toPoly({
							point{to,point::Exit,to_x + 20,to_y + to_height + Sugiyama::nodeBorderPadding/2},
							point{to,point::Corner,to_x - Sugiyama::nodeBorderPadding/2,to_y + to_height + Sugiyama::nodeBorderPadding/2},
							point{to,point::Corner,to_x - Sugiyama::nodeBorderPadding/2,to_y -Sugiyama::nodeBorderPadding/2},
							point{to,point::Entry,to_x + 20,to_y - Sugiyama::nodeBorderPadding/2}
						});
					}
					else
					{
						pp.moveTo(QPoint(out_x,from_y + from_height / 2));
						pp.lineTo(QPoint(out_x,from_y + from_height + nodeBorderPadding));
						pp.lineTo(QPoint(in_x,to_y - nodeBorderPadding));
						pp.lineTo(QPoint(in_x,to_y + to_height / 2));
					}

					ret.emplace(e,std::make_pair(pp,pp.pointAtPercent(.5)));
				}

				std::get<2>(i->second) = std::move(ret);

				for(auto e: iters(po::edges(g)))
					positionEdgeDecoration(e,i->second);

				emit routeDone();
				update();


				// XXX: race condition
				if(!_routeWatcher.isRunning())
				{
					std::unordered_map<itmgraph::vertex_descriptor,QRect> bbs;
					for(auto _vx: iters(po::vertices(g)))
					{
						auto vx = get_vertex(_vx,g).item;
						bbs.emplace(_vx,QRect(vx->x(),vx->y(),vx->width(),vx->height()));
					}

					_routeWatcher.setFuture(QtConcurrent::run(std::bind(doRoute,g,bbs,proc)));
				}
				else
				{
					_routingNeeded = true;
				}
			}
		}
	}
}

void Sugiyama::updateEdge(QObject* edge)
{
	if(_procedure && _procedure->procedure())
	{
		cache_type& cache = _cache.at(*(_procedure->procedure()));
		itmgraph const& graph = std::get<0>(cache);
		for(auto x: iters(edges(graph)))
			if(get_edge(x,graph).edge == edge)
				return updateEdgeDecorations(x,cache);
		qDebug() << edge << "not found";
	}
}

void Sugiyama::processRoute(void)
{
	if(!_procedure || !_procedure->procedure() || *_procedure->procedure() != _routeWatcher.result().first || !_routingNeeded)
	{
		po::proc_wloc proc = _routeWatcher.result().first;
		std::lock_guard<std::mutex> guard(_mutex);

		ensure(_cache.count(proc));

		std::get<2>(_cache[proc]) = _routeWatcher.result().second;
		cache_type& cache = _cache.at(proc);
		itmgraph& g = std::get<0>(cache);

		for(auto e: iters(po::edges(g)))
		{
			positionEdgeDecoration(e,cache);
		}

		emit routeDone();
		update();
	}

	if(_routingNeeded)
	{
		_routingNeeded = false;
		route();
	}
}

void Sugiyama::processLayout(void)
{
	po::proc_wloc proc = _layoutWatcher.result().first;
	ensure(_cache.count(proc));

	_cache[proc] = cache_type(std::get<0>(_cache.at(proc)),_layoutWatcher.result().second,boost::none);

	for(auto vx: iters(vertices(std::get<0>(_cache.at(proc)))))
	{
		positionNode(vx,std::get<0>(_cache.at(proc)),std::get<1>(_cache.at(proc))->at(vx));
	}

	emit layoutDone();
	route();
}

void Sugiyama::positionNode(itmgraph::vertex_descriptor v, itmgraph const& graph, std::tuple<unsigned int,unsigned int,unsigned int> pos)
{
	node_proxy const& np = get_vertex(v,graph);

	np.context->setContextProperty("firstRank",std::get<0>(pos));
	np.context->setContextProperty("lastRank",std::get<1>(pos));
	np.context->setContextProperty("computedX",std::get<2>(pos));
	np.item->setVisible(true);
}

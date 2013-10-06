#include <QPointF>

#include "interface.hh"

GraphInterface::GraphInterface(QGraphicsScene *s, Scene *g)
: graph(g), scene(s)
{}

template<>
std::pair<QList<QDeclarativeItem*>::const_iterator,QList<QDeclarativeItem*>::const_iterator> dot::nodes<GraphInterface>(GraphInterface t)
{
	return std::make_pair(t.graph->nodeList().begin(),t.graph->nodeList().end());
}

template<>
std::pair<QList<Path*>::const_iterator,QList<Path*>::const_iterator> dot::edges<GraphInterface>(GraphInterface t)
{
	return std::make_pair(t.graph->pathList().begin(),t.graph->pathList().end());
}

template<>
std::pair<QList<Path*>::const_iterator,QList<Path*>::const_iterator> dot::out_edges<GraphInterface>(QDeclarativeItem* n, GraphInterface t)
{
	const QList<Path*> &p = t.graph->outEdges(n);
	return std::make_pair(p.begin(),p.end());
}

/*template<>
std::pair<QList<Path*>::const_iterator,QList<Path*>::const_iterator> in_paths<GraphInterface>(uint64_t n, GraphInterface t)
{
	return t.paths_by_head.equal_range(n);
}*/

template<>
QDeclarativeItem* dot::source<GraphInterface>(Path* e, GraphInterface)
{
	return e->from();
}

template<>
QDeclarativeItem* dot::sink<GraphInterface>(Path* e, GraphInterface)
{
	return e->to();
}

template<>
unsigned int dot::weight<GraphInterface>(Path* e, GraphInterface)
{
	return 1;
}

template<>
std::pair<unsigned int,unsigned int> dot::dimensions<GraphInterface>(QDeclarativeItem* n, GraphInterface)
{
	const QRectF &bb = n->boundingRect();
	return std::make_pair(bb.width(),bb.height());
}

template<>
bool dot::has_entry<GraphInterface>(GraphInterface g)
{
	return false;//g.root;
}

template<>
QDeclarativeItem* dot::entry<GraphInterface>(GraphInterface g)
{
	assert(false);
}

template<>
void dot::set_position(QDeclarativeItem *n, const coord &pos, GraphInterface)
{
	n->setPos(QPointF(pos.first,pos.second));
}

template<>
dot::coord dot::position(QDeclarativeItem *n, GraphInterface)
{
	assert(n);
	QRectF bb = n->mapToScene(n->boundingRect()).boundingRect();
	return std::make_pair(bb.x(),bb.y());
}

template<>
void dot::set_segments(Path *e, const std::list<coord> &segs, GraphInterface)
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

template<>
bool dot::is_free(float x, float y, unsigned int w, unsigned int h, Path *e, GraphInterface g)
{
	QRectF bb(QPointF(x,y),QSizeF(w,h));
	QList<QGraphicsItem*> itms = g.scene->items(bb);

	bool ret = g.scene->sceneRect().contains(bb) &&
						 std::none_of(itms.begin(),itms.end(),[&](QGraphicsItem *i)
						 {
							 return i != e->from() &&
											i != e->to() &&
											g.graph->nodeList().contains(dynamic_cast<QDeclarativeItem*>(i));
						 });
	return ret;
}

template<>
bool dot::is_free(const dot::vis_node<GraphInterface> &a, const dot::vis_node<GraphInterface> &b, GraphInterface graph)
{
	QPainterPath line;
	QList<QGraphicsItem*> items;

	line.moveTo(QPointF(a.position.first,a.position.second));
	line.lineTo(QPointF(b.position.first,b.position.second));

	items = graph.scene->items(line,Qt::IntersectsItemBoundingRect);

	if(a.node.is_node())
		items.removeAll(a.node.node());
	if(b.node.is_node())
		items.removeAll(b.node.node());

	// collision?
	QListIterator<QGraphicsItem*> iter(items);

	while(iter.hasNext())
		if(graph.graph->nodeList().contains(dynamic_cast<QDeclarativeItem*>(iter.next())))
			return false;
	return true;
}

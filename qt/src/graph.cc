#include <QDebug>
#include <QApplication>
#include <QDesktopWidget>

#include <string>
#include <cassert>
#include <sstream>

#include <graph.hh>

Graph::Graph(void)
: m_gvContext(gvContext()), m_graph(0)	
{
	return;
}

Graph::~Graph(void)
{
	deleteGraph();
	gvFreeContext(m_gvContext);
}

QList<QGraphicsObject *> &Graph::nodes(void)
{
	return m_nodes;
}

QList<Arrow *> &Graph::edges(void)
{
	return m_edges;
}

std::pair<Graph::iterator,Graph::iterator> Graph::out_edges(QGraphicsObject *n) 
{
	std::function<bool(Arrow *)> pred = [=](Arrow *a) { return a && a->from() == n; };
	return std::make_pair(iterator(pred,m_incidence.lowerBound(n),m_incidence.end()),iterator(pred,m_incidence.upperBound(n),m_incidence.end()));
}

std::pair<Graph::iterator,Graph::iterator> Graph::in_edges(QGraphicsObject *n) 
{
	std::function<bool(Arrow *)> pred = [=](Arrow *a) { return a &&  a->to() == n; };
	return std::make_pair(iterator(pred,m_incidence.lowerBound(n),m_incidence.end()),iterator(pred,m_incidence.upperBound(n),m_incidence.end()));
}

QRectF Graph::layoutCustom(QString algorithm)
{
	if(nodes().empty())
		return QRectF();

	// first run
	if(m_graph)
		deleteGraph();
	allocateGraph();

	gvLayout(m_gvContext,m_graph,(char *)algorithm.toStdString().c_str());
	materializeGraph();

	return QRectF();
}

QRectF Graph::layoutHierarchically(void)
{
	if(nodes().empty())
		return QRectF();

	// first run
	if(m_graph)
		deleteGraph();
	allocateGraph();

	gvLayout(m_gvContext,m_graph,(char *)std::string("dot").c_str());
	QPointF off(GD_bb(m_graph).UR.x,GD_bb(m_graph).UR.y);

	// now that the nodes are laid out arrange the ports to avoid edges overlapping
	QMapIterator<QGraphicsObject *,Agnode_t *> i(m_nodeProxies);
	while(i.hasNext())
	{
		i.next();
		QGraphicsObject *obj = i.key();
		Agnode_t *node = i.value();
		QMap<int,Agedge_t *> incoming, outgoing;
	
		// incoming
		auto p = in_edges(obj);
		while(p.first != p.second)
		{
			Arrow *arrow = *p.first;
			Agedge_t *edge = m_edgeProxies[arrow];
			QGraphicsObject *src_obj = arrow->from();
			Agnode_t *src_node = m_nodeProxies[src_obj];
			int k = ND_coord(node).y >= ND_coord(src_node).y ? ND_coord(src_node).x * 100 : ND_coord(src_node).x;

			while(incoming.contains(k)) ++k;
			incoming.insert(k,edge);
			++p.first;
		}

		// outgoing
		p = out_edges(obj);
		while(p.first != p.second)
		{
			Arrow *arrow = *p.first;
			Agedge_t *edge = m_edgeProxies[arrow];
			QGraphicsObject *dst_obj = arrow->to();
			Agnode_t *dst_node = m_nodeProxies[dst_obj];
			int k = ND_coord(node).y < ND_coord(dst_node).y ? ND_coord(dst_node).x * 100 : ND_coord(dst_node).x;

			while(outgoing.contains(k)) ++k;
			outgoing.insert(k,edge);
			++p.first;
		}

		std::stringstream ss;
		int j = std::max(incoming.size() - 1,0);

		qDebug() << "insz" << j+1;
		ss << "{{<in0>";
		while(j) ss << "|<in" << j-- << ">";
		ss << "}|{<out0>";
		
		j = std::max(outgoing.size() - 1,0);
		while(j) ss << "|<out" << j-- << ">";
		ss << "}}";
	
		safeset(node,"label",ss.str());

		// arrange incoming
		QMapIterator<int,Agedge_t *> k(incoming);
		int ord = 0;
		while(k.hasNext())
		{
			k.next();
			Agedge_t *edge = k.value();

			safeset(edge,"headport","in" + std::to_string(ord++) + ":n");
			safeset(edge,"arrowhead","none");
		}
			
		// arrange outgoing
		QMapIterator<int,Agedge_t *> l(outgoing);
		ord = 0;
		while(l.hasNext())
		{
			l.next();
			Agedge_t *edge = l.value();

			safeset(edge,"tailport","out" + std::to_string(ord++) + ":s");
			safeset(edge,"arrowtail","none");
		}
	}

	// second run, now with proper ports
	gvFreeLayout(m_gvContext,m_graph);	
	gvLayout(m_gvContext,m_graph,(char *)std::string("dot").c_str());	
	//gvRender(m_gvContext,m_graph,(char *)std::string("xdot").c_str(),stdout);
	materializeGraph();

	return QRectF();
}

void Graph::materializeGraph(void)
{
	assert(m_graph);

	QPointF off(GD_bb(m_graph).UR.x,GD_bb(m_graph).UR.y);

	// move QGraphicsObject's according to Graphviz node proxies
	QMapIterator<QGraphicsObject *,Agnode_t *> k(m_nodeProxies);
	while(k.hasNext())
	{
		k.next();
		QGraphicsObject *n = k.key();
		Agnode_t *p = k.value();
		QSizeF sz(ND_width(p)*72,ND_height(p)*72);
		QPointF orig(ND_coord(p).x,off.y()-ND_coord(p).y);

		n->setPos(orig - QPointF(sz.width() / 2.0,sz.height() / 2.0));
	}

	QMapIterator<Arrow *,Agedge_t *> m(m_edgeProxies);
	while(m.hasNext())
	{
		m.next();

		Arrow *a = m.key();
		Agedge_t *e = m.value();
		QPainterPath pp;
		int i = 1;
		const pointf *p = ED_spl(e)->list[0].list;
		QPointF p0(p[0].x,off.y()-p[0].y);

		assert(((ED_spl(e)->list[0].size - 1) % 3) == 0);
		pp.moveTo(p0);

		while(i < ED_spl(e)->list[0].size)
		{
			QPointF p1(p[i].x,off.y()-p[i].y), 
							p2(p[i+1].x,off.y()-p[i+1].y), 
							p3(p[i+2].x,off.y()-p[i+2].y);	// points

			pp.cubicTo(p1,p2,p3);
			i += 3;
		}

		a->setPath(pp);
	}
}

void Graph::deleteGraph(void)
{
	if(!m_graph)
		return;

	// edges
	QMapIterator<Arrow *,Agedge_t *> i(m_edgeProxies);
	while(i.hasNext())
	{
		i.next();
		agdelete(m_graph,i.value());
	}

	QMapIterator<QGraphicsObject *,Agnode_t *> j(m_nodeProxies);
	while(j.hasNext())
	{
		j.next();
		agdelete(m_graph,j.value());
	}	
	
	agclose(m_graph);
}

void Graph::allocateGraph(void)
{
	if(m_graph)
		deleteGraph();
	
	m_graph = agopen((char *)std::string("g").c_str(),AGDIGRAPH);

	safeset(m_graph,"nodesep","1.2");
	safeset(m_graph,"ranksep","1.2");
	safeset(m_graph,"esep","1");

	// allocate Graphviz nodes shadowing the Nodes in the scene
	QListIterator<QGraphicsObject *> i(nodes());
	while(i.hasNext())
	{
		QGraphicsObject *n = i.next();
		std::string name = std::to_string((ptrdiff_t)n);
		QRectF bb = n->boundingRect();
		Agnode_t *p = agnode(m_graph,(char *)name.c_str());

		safeset(p,"width",std::to_string(bb.width()/72.0));
		safeset(p,"height",std::to_string(bb.height()/72.0));
		safeset(p,"fixedsize","true");
		safeset(p,"shape","record");
		safeset(p,"label","");

		m_nodeProxies.insert(n,p);
	}

	// add Graphviz edges 
	QMapIterator<QGraphicsObject *, Arrow *> j(m_incidence);
	while(j.hasNext())
	{
		j.next();
		Arrow *a = j.value();
		Agnode_t *from = m_nodeProxies[a->from()], *to = m_nodeProxies[a->to()];
		Agedge_t *e = agedge(m_graph,from,to);

		m_edgeProxies.insert(a,e);
	}
}

void Graph::safeset(void *obj, std::string key, std::string value) const
{
	agsafeset(obj,(char *)key.c_str(),(char *)value.c_str(),(char *)value.c_str());
}

void Graph::insert(QGraphicsObject *n)
{
	assert(!m_nodes.contains(n));
	
	addItem(n);
	m_nodes.append(n);
}

void Graph::connect(Arrow *a)
{
	assert(a && m_nodes.contains(a->from()) && m_nodes.contains(a->to()));
	
	addItem(dynamic_cast<QGraphicsItem *>(a));
	m_edges.append(a);
	m_incidence.insert(a->from(),a);
	m_incidence.insert(a->to(),a);
}

void Graph::clear(void)
{
	QGraphicsScene::clear();
	m_incidence.clear();
	m_edges.clear();
	m_nodes.clear();
}


#include <QDebug>
#include <QApplication>
#include <QDesktopWidget>

#include <string>
#include <cassert>
#include <sstream>

#include <graph.hh>

Graph::Graph(void)
{
	return;
}

Graph::~Graph(void)
{
	deleteGraph();
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
	allocateGraph();

	materializeGraph();

	return QRectF();
}

QRectF Graph::layoutHierarchically(void)
{
	if(nodes().empty())
		return QRectF();

	// first run
	allocateGraph();

	materializeGraph();

	return QRectF();
}

void Graph::materializeGraph(void)
{
}

void Graph::deleteGraph(void)
{
		return;
}

void Graph::allocateGraph(void)
{
}

void Graph::safeset(void *obj, std::string key, std::string value) const
{
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


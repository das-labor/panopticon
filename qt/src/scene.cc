#include <QDebug>
#include <QApplication>
#include <QDesktopWidget>

#include <string>
#include <cassert>
#include <sstream>

#include <scene.hh>

Scene::Scene(void)
{
	return;
}

Scene::~Scene(void)
{
	deleteScene();
}

QList<QGraphicsObject *> &Scene::nodes(void)
{
	return m_nodes;
}

QList<Arrow *> &Scene::edges(void)
{
	return m_edges;
}

std::pair<Scene::iterator,Scene::iterator> Scene::out_edges(QGraphicsObject *n)
{
	std::function<bool(Arrow *)> pred = [=](Arrow *a) { return a && a->from() == n; };
	return std::make_pair(iterator(pred,m_incidence.lowerBound(n),m_incidence.end()),iterator(pred,m_incidence.upperBound(n),m_incidence.end()));
}

std::pair<Scene::iterator,Scene::iterator> Scene::in_edges(QGraphicsObject *n)
{
	std::function<bool(Arrow *)> pred = [=](Arrow *a) { return a &&  a->to() == n; };
	return std::make_pair(iterator(pred,m_incidence.lowerBound(n),m_incidence.end()),iterator(pred,m_incidence.upperBound(n),m_incidence.end()));
}

QRectF Scene::layoutCustom(QString algorithm)
{
	if(nodes().empty())
		return QRectF();

	// first run
	allocateScene();

	materializeScene();

	return QRectF();
}

QRectF Scene::layoutHierarchically(void)
{
	if(nodes().empty())
		return QRectF();

	// first run
	allocateScene();

	materializeScene();

	return QRectF();
}

void Scene::materializeScene(void)
{
}

void Scene::deleteScene(void)
{
		return;
}

void Scene::allocateScene(void)
{
}

void Scene::safeset(void *obj, std::string key, std::string value) const
{
}

void Scene::insert(QGraphicsObject *n)
{
	assert(!m_nodes.contains(n));

	addItem(n);
	m_nodes.append(n);
}

void Scene::connect(Arrow *a)
{
	assert(a && m_nodes.contains(a->from()) && m_nodes.contains(a->to()));

	addItem(dynamic_cast<QGraphicsItem *>(a));
	m_edges.append(a);
	m_incidence.insert(a->from(),a);
	m_incidence.insert(a->to(),a);
}

void Scene::clear(void)
{
	QGraphicsScene::clear();
	m_incidence.clear();
	m_edges.clear();
	m_nodes.clear();
}


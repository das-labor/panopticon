#ifndef GRAPH_HH
#define GRAPH_HH

#include <functional>

#include <QGraphicsScene>
#include <QGraphicsObject>
#include <QGraphicsTextItem>
#include <QGraphicsSimpleTextItem>
#include <QGraphicsRectItem>
#include <QGraphicsPathItem>
#include <QVariant>
#include <QVariantAnimation>
#include <QPoint>
#include <QWidget>
#include <QPainter>
#include <QStyleOptionGraphicsItem>

#include <boost/iterator/filter_iterator.hpp>

extern "C" {
#include <gvc.h>
}

class Arrow;
class Graph;

class Arrow
{
public:
	virtual QGraphicsObject *from(void) = 0;
	virtual QGraphicsObject *to(void) = 0;
	virtual QPainterPath path(void) const = 0;
	virtual void setPath(QPainterPath pp) = 0;
	virtual QRectF boundingRect(void) const = 0;
};

class Graph : public QGraphicsScene
{
	Q_OBJECT

public:
	typedef boost::filter_iterator<std::function<bool(Arrow *)>,QMultiMap<QGraphicsObject *,Arrow *>::iterator> iterator;

	Graph(void);
	~Graph(void);
	
	QList<QGraphicsObject *> &nodes(void);
	QList<Arrow *> &edges(void);
	std::pair<iterator,iterator> out_edges(QGraphicsObject *n);
	std::pair<iterator,iterator> in_edges(QGraphicsObject *n);

	void insert(QGraphicsObject *n);
	void connect(Arrow *a);
	void clear(void);

	QRectF layoutCustom(QString algo);
	QRectF layoutHierarchically(void);

private:
	QList<QGraphicsObject *> m_nodes;
	QList<Arrow *> m_edges;
	QMultiMap<QGraphicsObject *,Arrow *> m_incidence;
	
	// Graphviz (libgraph)
	GVC_t *m_gvContext;
	Agraph_t *m_graph;
	QMap<QGraphicsObject *,Agnode_t *> m_nodeProxies;
	QMap<Arrow *,Agedge_t *> m_edgeProxies;

	void allocateGraph(void);
	void deleteGraph(void);
	void materializeGraph(void);
	void safeset(void *obj, std::string key, std::string value) const;
};

#endif

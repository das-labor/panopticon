#ifndef GRAPH_HH
#define GRAPH_HH

#include <functional>

#include <QGraphicsScene>
#include <QGraphicsObject>
#include <QGraphicsTextItem>
#include <QGraphicsRectItem>
#include <QGraphicsPathItem>
#include <QVariant>
#include <QVariantAnimation>
#include <QPoint>
#include <QWidget>
#include <QPainter>
#include <QStyleOptionGraphicsItem>

#include <boost/iterator/filter_iterator.hpp>

class Arrow;
class Graph;

class Arrow : public QGraphicsObject
{
	Q_OBJECT

public:
	Arrow(QPainterPath &pp, QGraphicsObject *f, QGraphicsObject *t);

	virtual QRectF boundingRect(void) const;
	virtual void paint(QPainter *painter, const QStyleOptionGraphicsItem *option, QWidget *widget = 0);
	
	QGraphicsObject *from(void);
	QGraphicsObject *to(void);

	void setHighlighted(bool tp);
	void setPath(QPainterPath &pp);

private:
	QGraphicsPathItem m_path;
	QGraphicsObject *m_from;
	QGraphicsObject *m_to;
	QPolygonF m_head;
	bool m_highlighted;
};

class Graph : public QGraphicsScene
{
	Q_OBJECT

public:
	typedef boost::filter_iterator<std::function<bool(Arrow *)>,QMultiMap<QGraphicsObject *,Arrow *>::iterator> iterator;

	Graph(void);
	
	QList<QGraphicsObject *> &nodes(void);
	QList<Arrow *> &edges(void);
	std::pair<iterator,iterator> out_edges(QGraphicsObject *n);
	std::pair<iterator,iterator> in_edges(QGraphicsObject *n);

	void insert(QGraphicsObject *n);
	void connect(QGraphicsObject *a, QGraphicsObject *b);
	void clear(void);
	QRectF graphLayout(QString algo);

private:
	QList<QGraphicsObject *> m_nodes;
	QList<Arrow *> m_edges;
	QMultiMap<QGraphicsObject *,Arrow *> m_incidence;
};

#endif

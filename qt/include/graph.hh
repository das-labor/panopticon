#ifndef GRAPH_HH
#define GRAPH_HH

#include <functional>

#include <QGraphicsScene>
#include <QGraphicsObject>
#include <QGraphicsTextItem>
#include <QGraphicsRectItem>
#include <QVariant>
#include <QVariantAnimation>
#include <QPoint>
#include <QWidget>
#include <QPainter>
#include <QStyleOptionGraphicsItem>

class Node;
class Arrow;
class Animation;
class Graph;

class Node : public QGraphicsObject
{
	Q_OBJECT

public:
	Node(QString name, QPoint ptn = QPoint(0,0));

	virtual QRectF boundingRect(void) const;
	virtual void paint(QPainter *painter, const QStyleOptionGraphicsItem *option, QWidget *widget = 0);
	
	void smoothSetPos(QPointF ptn);
	void setTitle(QString s);

protected:
	virtual QVariant itemChange(GraphicsItemChange change, const QVariant &value);

private:
	QGraphicsTextItem m_text;
	QGraphicsRectItem m_rect;
	Animation *m_animation;
};

class Arrow : public QGraphicsObject
{
	Q_OBJECT

public:
	Arrow(QGraphicsObject *f, QGraphicsObject *t);

	virtual QRectF boundingRect(void) const;
	virtual void paint(QPainter *painter, const QStyleOptionGraphicsItem *option, QWidget *widget = 0);
	
	QGraphicsObject *from(void);
	QGraphicsObject *to(void);

	void setHighlighted(bool tp);

private slots:
	void updated(void);

private:
	QGraphicsObject *m_from;
	QGraphicsObject *m_to;
	QPolygonF m_head;
	bool m_highlighted;
};

class Animation : public QVariantAnimation
{
	Q_OBJECT

public:
	Animation(std::function<void(const QVariant &)> func, QObject *parent = 0);

protected:
	virtual void updateCurrentValue(const QVariant &value);
	
private:
	std::function<void(const QVariant &)> m_function;
};

class Graph : public QGraphicsScene
{
	Q_OBJECT

public:
	Graph(void);
	
	QList<QGraphicsObject *> &nodes(void);
	QList<Arrow *> &edges(void);
	std::pair<QMultiMap<QGraphicsObject *,Arrow *>::iterator,QMultiMap<QGraphicsObject *,Arrow *>::iterator> out_edges(QGraphicsObject *n);

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

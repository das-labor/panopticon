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
class Scene;

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
	Arrow(Node *f, Node *t);

	virtual QRectF boundingRect(void) const;
	virtual void paint(QPainter *painter, const QStyleOptionGraphicsItem *option, QWidget *widget = 0);
	
	Node *from(void);
	Node *to(void);

	void setHighlighted(bool tp);

private slots:
	void updated(void);

private:
	Node *m_from;
	Node *m_to;
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

class Scene : public QGraphicsScene
{
	Q_OBJECT

public:
	Scene(void);
	
	QList<Node *> &nodes(void);
	QList<Arrow *> &edges(void);
	std::pair<QMultiMap<Node *,Arrow *>::iterator,QMultiMap<Node *,Arrow *>::iterator> out_edges(Node *n);

	void insert(Node *n);
	void connect(Node *a, Node *b);

public slots:
	QRectF graphLayout(void);

private:
	QList<Node *> m_nodes;
	QList<Arrow *> m_edges;
	QMultiMap<Node *,Arrow *> m_incidence;
};

#endif

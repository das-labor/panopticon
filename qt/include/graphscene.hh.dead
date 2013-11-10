#include <QQmlListProperty>
#include <QList>
#include <QDebug>
#include <QStateMachine>
#include <QEventTransition>

#include <QtQml>
#include <QtQuick>
#include <QPainter>
#include <QPen>

#include "graphstatemachine.hh"
#include "dot/dot.hh"

#pragma once

class GraphScenePen : public QObject, public QPen
{
	Q_OBJECT
	Q_PROPERTY(QColor color READ color WRITE setColor)
	Q_PROPERTY(qreal width READ width WRITE setWidth)
	Q_PROPERTY(PenStyle style READ style WRITE setStyle)
	Q_PROPERTY(CapStyle capStyle READ capStyle WRITE setCapStyle)
	Q_PROPERTY(JoinStyle joinStyle READ joinStyle WRITE setJoinStyle)
	Q_ENUMS(PenStyle)
	Q_ENUMS(CapStyle)
	Q_ENUMS(JoinStyle)

public:
	enum PenStyle
	{
		NoPen =	0,
		SolidLine = 1,
		DashLine = 2,
		DotLine = 3,
		DashDotLine = 4,
		DashDotDotLine = 5,
		CustomDashLine = 6,
	};

	enum CapStyle
	{
		FlatCap = 0x00,
		SquareCap = 0x10,
		RoundCap = 0x20,
	};

	enum JoinStyle
	{
		MiterJoin = 0x00,
		BevelJoin = 0x40,
		RoundJoin = 0x80,
		SvgMiterJoin = 0x100,
	};

	GraphScenePen(void) {}
	virtual ~GraphScenePen(void) {}

	QColor color(void) const { return QPen::color(); }
	qreal width(void) const { return QPen::widthF(); }
	PenStyle style(void) const { return static_cast<PenStyle>(QPen::style()); }
	CapStyle capStyle(void) const { return static_cast<CapStyle>(QPen::capStyle()); }
	JoinStyle joinStyle(void) const { return static_cast<JoinStyle>(QPen::joinStyle()); }

	void setColor(const QColor &x) { QPen::setColor(x); emit changed(); }
	void setWidth(qreal x) { QPen::setWidthF(x); }
	void setStyle(PenStyle x) { QPen::setStyle(static_cast<Qt::PenStyle>(x)); emit changed(); }
	void setCapStyle(CapStyle x) { QPen::setCapStyle(static_cast<Qt::PenCapStyle>(x)); emit changed(); }
	void setJoinStyle(JoinStyle x) { QPen::setJoinStyle(static_cast<Qt::PenJoinStyle>(x)); emit changed(); }

signals:
	void changed(void);
};

class GraphScenePath : public QQuickPaintedItem
{
	Q_OBJECT
	Q_PROPERTY(QQuickItem* from READ from WRITE setFrom)
	Q_PROPERTY(QQuickItem* to READ to WRITE setTo)
	Q_PROPERTY(bool direct READ isDirect WRITE setDirect)
	Q_PROPERTY(GraphScenePen *pen READ pen)
	Q_PROPERTY(QQuickItem* head READ head WRITE setHead)
	Q_PROPERTY(QQuickItem* tail READ tail WRITE setTail)

public:
	GraphScenePath(QQuickItem *from = 0, QQuickItem *to = 0,QQuickItem *parent = 0);
	virtual ~GraphScenePath(void);

	void setPath(const QPainterPath &pp);
	void setDirect(bool b);
	void setPen(const QPen &p);
	void setFrom(QQuickItem *obj);
	void setTo(QQuickItem *obj);
	void setHead(QQuickItem *obj);
	void setTail(QQuickItem *obj);

	virtual QRectF contentsBoundingRect() const;
	virtual void paint(QPainter *painter);

	QQuickItem *from(void) const;
	QQuickItem *to(void) const;
	QQuickItem *head(void) const;
	QQuickItem *tail(void) const;
	bool isDirect(void) const;
	GraphScenePen *pen(void);

public slots:
	void updateGeometry(void);
	void update(void);

signals:
	void nodesChanged(void);

private:
	QLineF contactVector(QQuickItem *itm) const;
	qreal approximateDistance(const QPointF &pnt) const;
	void positionEnds(void);

	QQuickItem *m_from;
	QQuickItem *m_to;
	QQuickItem *m_head;
	QQuickItem *m_tail;
	QPainterPath m_path;
	GraphScenePen m_pen;
	bool m_direct;
	QRectF m_boundingRect;
	QPointF m_fromCenter, m_toCenter;
};

class GraphSceneItem : public QQuickItem
{
	Q_OBJECT
	Q_PROPERTY(QQmlListProperty<QQuickItem> nodes READ nodes NOTIFY nodesChanged)
	Q_PROPERTY(QQmlListProperty<GraphScenePath> paths READ paths NOTIFY pathsChanged)

public:
	GraphSceneItem(QQuickItem *parent = 0);
	virtual ~GraphSceneItem(void);

	QQmlListProperty<QQuickItem> nodes(void);
	QQmlListProperty<GraphScenePath> paths(void);

	const QList<QQuickItem*> &nodeList(void) const;
	const QList<GraphScenePath*> &pathList(void) const;
	const QList<GraphScenePath*> &outEdges(QQuickItem *i) const;

signals:
	void nodesChanged(void);
	void pathsChanged(void);

protected:
	virtual void mousePressEvent(QMouseEvent *event);
	virtual void mouseReleaseEvent(QMouseEvent *event);
	virtual void mouseMoveEvent(QMouseEvent *event);
  virtual bool eventFilter(QObject *obj, QEvent *event);

private:
	QList<QQuickItem*> m_nodes;
	QList<GraphScenePath*> m_paths;
	QMap<QQuickItem*,QList<GraphScenePath*>> m_outEdges;
	QMap<QQuickItem*,NodeState*> m_nodeStates;
	QMap<GraphScenePath*,PathState*> m_pathStates;
	StateMachine m_stateMachine;
	QState *m_rootState;
	QSet<QQuickItem*> m_grabbedNodes;

	void layoutNodes(void);
	void routeGraphScenePaths(void);

	template<typename T>
	static void appendCallback(QQmlListProperty<T> *property, T *value);
	template<typename T>
	static int countCallback(QQmlListProperty<T> *property);
	template<typename T>
	static T *atCallback(QQmlListProperty<T> *property, int idx);
	template<typename T>
	static void clearCallback(QQmlListProperty<T> *property);

private slots:
	void sent(const Event &n);
	void updateOutEdges(void);
};

template<>
void GraphSceneItem::appendCallback(QQmlListProperty<QQuickItem> *property, QQuickItem *value);
template<>
void GraphSceneItem::appendCallback(QQmlListProperty<GraphScenePath> *property, GraphScenePath *value);
template<>
int GraphSceneItem::countCallback(QQmlListProperty<QQuickItem> *property);
template<>
int GraphSceneItem::countCallback(QQmlListProperty<GraphScenePath> *property);
template<>
QQuickItem *GraphSceneItem::atCallback(QQmlListProperty<QQuickItem> *property, int idx);
template<>
GraphScenePath *GraphSceneItem::atCallback(QQmlListProperty<GraphScenePath> *property, int idx);
template<>
void GraphSceneItem::clearCallback(QQmlListProperty<QQuickItem> *property);
template<>
void GraphSceneItem::clearCallback(QQmlListProperty<GraphScenePath> *property);

struct GraphSceneInterface
{
	GraphSceneInterface(QQuickView *v, GraphSceneItem *g);

	GraphSceneItem *graph;
	QQuickView *view;
};

namespace dot
{
	template<>
	struct graph_traits<GraphSceneInterface>
	{
		typedef QQuickItem* node_type;
		typedef GraphScenePath* edge_type;

		typedef QList<QQuickItem*>::const_iterator node_iterator;
		typedef QList<GraphScenePath*>::const_iterator edge_iterator;
	};

	template<>
	std::pair<QList<QQuickItem*>::const_iterator,QList<QQuickItem*>::const_iterator> nodes<GraphSceneInterface>(GraphSceneInterface t);
	template<>
	std::pair<QList<GraphScenePath*>::const_iterator,QList<GraphScenePath*>::const_iterator> edges<GraphSceneInterface>(GraphSceneInterface t);
	template<>
	std::pair<QList<GraphScenePath*>::const_iterator,QList<GraphScenePath*>::const_iterator> out_edges<GraphSceneInterface>(QQuickItem* n, GraphSceneInterface t);
	/*template<>
	std::pair<QList<GraphScenePath*>::const_iterator,QList<GraphScenePath*>::const_iterator> in_paths<GraphSceneInterface>(uint64_t n, GraphSceneInterface t)*/
	template<>
	QQuickItem* source<GraphSceneInterface>(GraphScenePath* e, GraphSceneInterface);
	template<>
	QQuickItem* sink<GraphSceneInterface>(GraphScenePath* e, GraphSceneInterface);
	template<>
	unsigned int weight<GraphSceneInterface>(GraphScenePath* e, GraphSceneInterface);
	template<>
	std::pair<unsigned int,unsigned int> dimensions<GraphSceneInterface>(QQuickItem* n, GraphSceneInterface);
	template<>
	bool has_entry<GraphSceneInterface>(GraphSceneInterface g);
	template<>
	QQuickItem* entry<GraphSceneInterface>(GraphSceneInterface g);
	template<>
	void set_position(QQuickItem *n, const coord &pos, GraphSceneInterface);
	template<>
	coord position(QQuickItem *n, GraphSceneInterface);
	template<>
	void set_segments(GraphScenePath *e, const std::list<coord> &segs, GraphSceneInterface);
	/*template<>
	bool is_free(float x, float y, unsigned int w, unsigned int h, GraphScenePath *e, GraphSceneInterface g);*/
	template<>
	bool is_free(const vis_node<GraphSceneInterface> &a, const vis_node<GraphSceneInterface> &b, GraphSceneInterface graph);
}

#include <unordered_map>

#include <QQmlListProperty>
#include <QList>
#include <QDebug>
#include <QHash>
#include <QVariant>

#include <QtQml>
#include <QtQuick>

#include <panopticon/digraph.hh>
#include <panopticon/hash.hh>

#pragma once

namespace
{
	struct point
	{
		bool operator==(point const& p) const { return p.node == node && p.x == x && p.y == y && is_center == p.is_center; }
		bool operator!=(point const& p) const { return !(p == *this); }
		po::digraph<std::tuple<QVariant,QQuickItem*,QQmlContext*>,std::tuple<QVariant,QPainterPath,QQuickItem*,QQuickItem*>>::vertex_descriptor node;
		bool is_center;
		int x, y;
	};
}

namespace std
{
	template<>
	struct hash<point>
	{
		size_t operator()(struct point const& p) const
		{
			return po::hash_struct<int,int,int>(p.node.id,p.x,p.y);
		}
	};

	template<>
	struct hash<std::tuple<QVariant,QQuickItem*,QQmlContext*>>
	{
		size_t operator()(const std::tuple<QVariant,QQuickItem*,QQmlContext*>& p) const
		{
			return po::hash_struct(get<1>(p));
		}
	};
}

class Sugiyama : public QQuickPaintedItem
{
	Q_OBJECT

	Q_PROPERTY(QQmlComponent* delegate READ delegate WRITE setDelegate NOTIFY delegateChanged)

	Q_PROPERTY(QVariantList vertices READ vertices WRITE setVertices NOTIFY verticesChanged)
	Q_PROPERTY(QVariantList edges READ edges WRITE setEdges NOTIFY edgesChanged)
	Q_PROPERTY(bool direct READ direct WRITE setDirect NOTIFY directChanged)

	using itmgraph = po::digraph<std::tuple<QVariant,QQuickItem*,QQmlContext*>,std::tuple<QVariant,QPainterPath,QQuickItem*,QQuickItem*>>;

public:
	Sugiyama(QQuickItem *parent = 0);
	virtual ~Sugiyama(void);

	QQmlComponent* delegate(void) const { return _delegate; }
	QVariantList vertices(void) const { return _vertices; }
	QVariantList edges(void) const { return _edges; }
	bool direct(void) const { return _direct; }

	void setDelegate(QQmlComponent* c) { _delegate = c; }
	void setVertices(QVariantList l) { _vertices = l; clear(); emit verticesChanged(); redoAttached(); layout(); route(); }
	void setEdges(QVariantList l) { _edges = l; clear(); emit edgesChanged(); redoAttached(); layout(); route(); }
	void setDirect(bool b) { _direct = b; emit directChanged(); route(); }

	itmgraph& graph(void);

	virtual void paint(QPainter *) override;
	void positionEnds(QObject* itm, QQuickItem* head, QQuickItem *tail, QQuickItem* from, QQuickItem* to, const QPainterPath& path);
	QLineF contactVector(QQuickItem *itm, const QPainterPath& pp) const;
	qreal approximateDistance(const QPointF &pnt, const QPainterPath& pp) const;
	void redoAttached(void);

public slots:
	void layout(void);
	void route(void);
	void updateEdge(QObject*);

signals:
	void verticesChanged(void);
	void edgesChanged(void);
	void delegateChanged(void);
	void directChanged(void);

	void layoutStart(void);
	void layoutDone(void);
	void routingStart(void);
	void routingDone(void);

private:
	using visgraph = std::unordered_multimap<point,point>;

	mutable boost::optional<itmgraph> _graph;
	QQmlComponent* _delegate;
	QVariantList _vertices;
	QVariantList _edges;
	bool _direct;
	QSignalMapper _mapper;
	std::list<QLine> _visgraph;

	void clear(void);

	static std::list<point> astar(point start, point goal, visgraph const& graph);
	static QPainterPath to_bezier(const std::list<point> &segs);
};

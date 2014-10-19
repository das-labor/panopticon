#include <unordered_map>

#include <QQmlListProperty>
#include <QList>
#include <QDebug>
#include <QHash>
#include <QVariant>
#include <QFuture>
#include <QFutureWatcher>

#include <QtQml>
#include <QtQuick>

#include <panopticon/digraph.hh>
#include <panopticon/hash.hh>

#pragma once

namespace
{
	using itmgraph = po::digraph<std::tuple<QVariant,QQuickItem*,QQmlContext*>,std::tuple<QVariant,QPainterPath,QQuickItem*,QQuickItem*,QQmlContext*,QQmlContext*>>;

	struct point
	{
		enum Type : uint8_t
		{
			Entry,
			Exit,
			Corner,
			Center
		};

		bool operator==(point const& p) const { return p.node == node && p.x == x && p.y == y && type == p.type; }
		bool operator!=(point const& p) const { return !(p == *this); }

		itmgraph::vertex_descriptor node;
		Type type;
		int x, y;
	};

	using visgraph = std::unordered_multimap<point,point>;
}

namespace std
{
	template<>
	struct hash<point>
	{
		size_t operator()(struct point const& p) const
		{
			return po::hash_struct<int,int,int,uint8_t>(p.node.id,p.x,p.y,p.type);
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

public:
	Sugiyama(QQuickItem *parent = nullptr);
	virtual ~Sugiyama(void);

	QQmlComponent* delegate(void) const { return _delegate; }
	QVariantList vertices(void) const { return _vertices; }
	QVariantList edges(void) const { return _edges; }
	bool direct(void) const { return _direct; }

	void setDelegate(QQmlComponent* c) { _delegate = c; }
	void setVertices(QVariantList l) { if(_vertices != l) { _vertices = l; clear(); emit verticesChanged(); redoAttached(); layout(); } }
	void setEdges(QVariantList l) { if(_edges != l) { _edges = l; clear(); emit edgesChanged(); redoAttached(); layout(); } }
	void setDirect(bool b) { if(b != _direct) { _direct = b; emit directChanged(); route(); } }

	virtual void paint(QPainter *) override;

	static const int delta;

public slots:
	void layout(void);
	void route(void);
	void updateEdge(QObject*);
	void processRoute(void);
	void processLayout(void);

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
	// Properties
	QQmlComponent* _delegate;
	QVariantList _vertices;
	QVariantList _edges;
	bool _direct;

	mutable boost::optional<itmgraph> _graph;
	QSignalMapper _mapper;
	QFutureWatcher<std::unordered_map<itmgraph::vertex_descriptor,std::tuple<unsigned int,unsigned int,unsigned int>>> _layoutWatcher;
	QFutureWatcher<std::unordered_map<itmgraph::edge_descriptor,QPainterPath>> _routeWatcher;

	void clear(void);
	itmgraph& graph(void);
	void positionEnds(QQuickItem* head, QQuickItem *tail, QQuickItem* from, QQuickItem* to, const QPainterPath& path);
	void redoAttached(void);

};

	std::unordered_map<itmgraph::edge_descriptor,QPainterPath>
		doRoute(itmgraph graph, std::unordered_map<itmgraph::vertex_descriptor,QRect> bboxes);
	QPainterPath toBezier(const std::list<point> &segs);
	std::unordered_map<itmgraph::vertex_descriptor,std::tuple<unsigned int,unsigned int,unsigned int>>
		doLayout(itmgraph, unsigned int, std::unordered_map<itmgraph::vertex_descriptor,int>);
	QLineF contactVector(QRectF const& bb, const QPainterPath& pp);
	qreal approximateDistance(const QPointF &pnt, const QPainterPath& pp);
	std::list<point> dijkstra(point start, point goal, visgraph const& graph);

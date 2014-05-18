#include <unordered_map>

#include <QQmlListProperty>
#include <QList>
#include <QDebug>
#include <QHash>
#include <QVariant>

#include <QtQml>
#include <QtQuick>

#include "dot/dot.hh"

#include <panopticon/digraph.hh>
#include <panopticon/hash.hh>

#pragma once

namespace std
{
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
	Sugiyama(QQuickItem *parent = 0);
	virtual ~Sugiyama(void);

	QQmlComponent* delegate(void) const { return _delegate; }
	QVariantList vertices(void) const { return _vertices; }
	QVariantList edges(void) const { return _edges; }
	bool direct(void) const { return _direct; }

	void setDelegate(QQmlComponent* c) { _delegate = c; }
	void setVertices(QVariantList l) { _vertices = l; clear(); emit verticesChanged(); layout(); route(); }
	void setEdges(QVariantList l) { _edges = l; clear(); emit edgesChanged(); redoAttached(); layout(); route(); }
	void setDirect(bool b) { _direct = b; emit directChanged(); route(); }

	po::digraph<std::tuple<QVariant,QQuickItem*,QQmlContext*>,std::tuple<QVariant,QPainterPath,QQuickItem*,QQuickItem*>>& graph(void);

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
	mutable boost::optional<po::digraph<std::tuple<QVariant,QQuickItem*,QQmlContext*>,std::tuple<QVariant,QPainterPath,QQuickItem*,QQuickItem*>>> _graph;
	QQmlComponent* _delegate;
	QVariantList _vertices;
	QVariantList _edges;
	bool _direct;
	QSignalMapper _mapper;

	void clear(void);
};

using SugiyamaInterface = boost::optional<Sugiyama*>;

namespace dot
{
	template<>
	struct graph_traits<SugiyamaInterface>
	{
		using graph = po::digraph<std::tuple<QVariant,QQuickItem*,QQmlContext*>,std::tuple<QVariant,QPainterPath,QQuickItem*,QQuickItem*>>;
		using node_type = boost::graph_traits<graph>::vertex_descriptor;
		using edge_type = boost::graph_traits<graph>::edge_descriptor;

		using node_iterator = boost::graph_traits<graph>::vertex_iterator;
		using edge_iterator = boost::graph_traits<graph>::edge_iterator;
		using out_edge_iterator = boost::graph_traits<graph>::out_edge_iterator;
	};

	template<>
	std::pair<graph_traits<SugiyamaInterface>::node_iterator,graph_traits<SugiyamaInterface>::node_iterator> nodes<SugiyamaInterface>(SugiyamaInterface t);
	template<>
	std::pair<graph_traits<SugiyamaInterface>::edge_iterator,graph_traits<SugiyamaInterface>::edge_iterator> edges<SugiyamaInterface>(SugiyamaInterface t);
	template<>
	std::pair<graph_traits<SugiyamaInterface>::out_edge_iterator,graph_traits<SugiyamaInterface>::out_edge_iterator> out_edges<SugiyamaInterface>(graph_traits<SugiyamaInterface>::node_type n, SugiyamaInterface t);
	template<>
	graph_traits<SugiyamaInterface>::node_type source<SugiyamaInterface>(graph_traits<SugiyamaInterface>::edge_type e, SugiyamaInterface);
	template<>
	graph_traits<SugiyamaInterface>::node_type sink<SugiyamaInterface>(graph_traits<SugiyamaInterface>::edge_type e, SugiyamaInterface);
	template<>
	unsigned int weight<SugiyamaInterface>(graph_traits<SugiyamaInterface>::edge_type e, SugiyamaInterface);
	template<>
	std::pair<unsigned int,unsigned int> dimensions<SugiyamaInterface>(graph_traits<SugiyamaInterface>::node_type n, SugiyamaInterface);
	template<>
	bool has_entry<SugiyamaInterface>(SugiyamaInterface g);
	template<>
	graph_traits<SugiyamaInterface>::node_type entry<SugiyamaInterface>(SugiyamaInterface g);
	template<>
	void set_position(graph_traits<SugiyamaInterface>::node_type n, const coord &pos, SugiyamaInterface);
	template<>
	coord position(graph_traits<SugiyamaInterface>::node_type n, SugiyamaInterface);
	template<>
	void set_segments(graph_traits<SugiyamaInterface>::edge_type e, const std::list<coord> &segs, SugiyamaInterface);
	template<>
	bool is_free(const vis_node<SugiyamaInterface> &a, const vis_node<SugiyamaInterface> &b, SugiyamaInterface graph);
}

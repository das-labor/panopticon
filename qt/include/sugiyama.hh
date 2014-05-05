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

/*class GraphScenePen : public QObject, public QPen
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
};*/

namespace std
{
	template<>
	struct hash<std::pair<QVariant,QQuickItem*>>
	{
		size_t operator()(const std::pair<QVariant,QQuickItem*>& p) const
		{
			return po::hash_struct(p.second);
		}
	};
}

class Sugiyama : public QQuickItem
{
	Q_OBJECT

	Q_PROPERTY(QQmlComponent* vertexDelegate READ vertexDelegate WRITE setVertexDelegate NOTIFY vertexDelegateChanged)
	Q_PROPERTY(QQmlComponent* edgeDelegate READ edgeDelegate WRITE setEdgeDelegate NOTIFY edgeDelegateChanged)

	Q_PROPERTY(QVariantList vertices READ vertices WRITE setVertices NOTIFY verticesChanged)
	Q_PROPERTY(QVariantList edges READ edges WRITE setEdges NOTIFY edgesChanged)

public:
	Sugiyama(QQuickItem *parent = 0);
	virtual ~Sugiyama(void);

	QQmlComponent* vertexDelegate(void) const { return _vertexDelegate; }
	QQmlComponent* edgeDelegate(void) const { return _edgeDelegate; }
	QVariantList vertices(void) const { return _vertices; }
	QVariantList edges(void) const { return _edges; }

	void setVertexDelegate(QQmlComponent* c) { _vertexDelegate = c; }
	void setEdgeDelegate(QQmlComponent* c) { _edgeDelegate = c; }
	void setVertices(QVariantList l) { _vertices = l; clear(); emit verticesChanged(); layout(); route(); }
	void setEdges(QVariantList l) { _edges = l; clear(); emit edgesChanged(); layout(); route(); }

	po::digraph<std::pair<QVariant,QQuickItem*>,std::pair<QVariant,QQuickItem*>>& graph(void);

signals:
	void verticesChanged(void);
	void edgesChanged(void);
	void vertexDelegateChanged(void);
	void edgeDelegateChanged(void);

	void vertsChanged(void);
	void edgsChanged(void);

	void layoutStart(void);
	void layoutDone(void);
	void routingStart(void);
	void routingDone(void);

private:
	mutable boost::optional<po::digraph<std::pair<QVariant,QQuickItem*>,std::pair<QVariant,QQuickItem*>>> _graph;
	QQmlComponent* _vertexDelegate;
	QQmlComponent* _edgeDelegate;
	QVariantList _vertices;
	QVariantList _edges;

	void layout(void);
	void route(void);
	void clear(void);
};

using SugiyamaInterface = boost::optional<Sugiyama*>;

namespace dot
{
	template<>
	struct graph_traits<SugiyamaInterface>
	{
		using graph = typename po::digraph<std::pair<QVariant,QQuickItem*>,std::pair<QVariant,QQuickItem*>>;
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
	//template<>
	//std::pair<QList<QQuickItem*>::const_iterator,QList<QQuickItem*>::const_iterator> in_paths<SugiyamaInterface>(uint64_t n, SugiyamaInterface t)
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
	//template<>
	//bool is_free(float x, float y, unsigned int w, unsigned int h, QQuickItem *e, SugiyamaInterface g);
	template<>
	bool is_free(const vis_node<SugiyamaInterface> &a, const vis_node<SugiyamaInterface> &b, SugiyamaInterface graph);
}

#ifndef INTERFACE_HH
#define INTERFACE_HH

#include <dot/dot.hh>
#include <scene.hh>

struct GraphInterface
{
	GraphInterface(QGraphicsScene *s, Scene *g);

	Scene *graph;
	QGraphicsScene *scene;
};

namespace dot
{
	template<>
	struct graph_traits<GraphInterface>
	{
		typedef QDeclarativeItem* node_type;
		typedef Path* edge_type;

		typedef QList<QDeclarativeItem*>::const_iterator node_iterator;
		typedef QList<Path*>::const_iterator edge_iterator;
	};

	template<>
	std::pair<QList<QDeclarativeItem*>::const_iterator,QList<QDeclarativeItem*>::const_iterator> nodes<GraphInterface>(GraphInterface t);
	template<>
	std::pair<QList<Path*>::const_iterator,QList<Path*>::const_iterator> edges<GraphInterface>(GraphInterface t);
	template<>
	std::pair<QList<Path*>::const_iterator,QList<Path*>::const_iterator> out_edges<GraphInterface>(QDeclarativeItem* n, GraphInterface t);
	/*template<>
	std::pair<QList<Path*>::const_iterator,QList<Path*>::const_iterator> in_paths<GraphInterface>(uint64_t n, GraphInterface t)*/
	template<>
	QDeclarativeItem* source<GraphInterface>(Path* e, GraphInterface);
	template<>
	QDeclarativeItem* sink<GraphInterface>(Path* e, GraphInterface);
	template<>
	unsigned int weight<GraphInterface>(Path* e, GraphInterface);
	template<>
	std::pair<unsigned int,unsigned int> dimensions<GraphInterface>(QDeclarativeItem* n, GraphInterface);
	template<>
	bool has_entry<GraphInterface>(GraphInterface g);
	template<>
	QDeclarativeItem* entry<GraphInterface>(GraphInterface g);
	template<>
	void set_position(QDeclarativeItem *n, const coord &pos, GraphInterface);
	template<>
	coord position(QDeclarativeItem *n, GraphInterface);
	template<>
	void set_segments(Path *e, const std::list<coord> &segs, GraphInterface);
	template<>
	bool is_free(float x, float y, unsigned int w, unsigned int h, Path *e, GraphInterface g);
	template<>
	bool is_free(const vis_node<GraphInterface> &a, const vis_node<GraphInterface> &b, GraphInterface graph);
}

#endif

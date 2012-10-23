#include <bgl.hh>

// Incidence Graph
Node *source(Arrow *a, Graph *s)
{
	return a->from();
}

Node *target(Arrow *a, Graph *s)
{
	return a->to();
}

std::pair<QMultiMap<Node *,Arrow *>::iterator,QMultiMap<Node *,Arrow *>::iterator> out_edges(Node *n, Graph *s)
{
	return s->out_edges(n);
}

unsigned int out_degree(Node *n, Graph *s)
{
	auto p = s->out_edges(n);
	return std::distance(p.first,p.second);
}

// VertexListGraph
std::pair<QList<Node *>::iterator,QList<Node *>::iterator> vertices(Graph *s)
{
	return std::make_pair(s->nodes().begin(),s->nodes().end());
}

unsigned int num_vertices(Graph *s)
{
	return (unsigned int)std::max(0,s->nodes().size());
}

// EdgeListGraph
std::pair<QList<Arrow *>::iterator,QList<Arrow *>::iterator> edges(Graph *s)
{
	return std::make_pair(s->edges().begin(),s->edges().end());
}

unsigned int num_edges(Graph *s)
{
	return (unsigned int)std::max(0,s->edges().size());
}

#include <bgl.hh>

// Incidence Scene
Node *source(Arrow *a, Scene *s)
{
	return a->from();
}

Node *target(Arrow *a, Scene *s)
{
	return a->to();
}

std::pair<QMultiMap<Node *,Arrow *>::iterator,QMultiMap<Node *,Arrow *>::iterator> out_edges(Node *n, Scene *s)
{
	return s->out_edges(n);
}

unsigned int out_degree(Node *n, Scene *s)
{
	auto p = s->out_edges(n);
	return std::distance(p.first,p.second);
}

// VertexListScene
std::pair<QList<Node *>::iterator,QList<Node *>::iterator> vertices(Scene *s)
{
	return std::make_pair(s->nodes().begin(),s->nodes().end());
}

unsigned int num_vertices(Scene *s)
{
	return (unsigned int)std::max(0,s->nodes().size());
}

// EdgeListScene
std::pair<QList<Arrow *>::iterator,QList<Arrow *>::iterator> edges(Scene *s)
{
	return std::make_pair(s->edges().begin(),s->edges().end());
}

unsigned int num_edges(Scene *s)
{
	return (unsigned int)std::max(0,s->edges().size());
}

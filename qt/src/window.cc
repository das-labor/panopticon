#include <window.hh>

Window::Window(void)
: m_viewport(&m_graph,this)
{
	setWindowTitle("Panopticum v0.8");
	resize(1000,800);
	move(500,200);
	setCentralWidget(&m_viewport);	

	Node *n_a = new Node("A",QPoint(0,0));
	Node *n_b = new Node("B",QPoint(0,0));
	Node *n_c = new Node("C",QPoint(0,0));
	Node *n_d = new Node("D",QPoint(0,0));
	Node *n_e = new Node("E",QPoint(0,0));
	Node *n_f = new Node("F",QPoint(0,0));
	Node *n_g = new Node("G",QPoint(0,0));
	Node *n_h = new Node("H",QPoint(0,0));
	Node *n_i = new Node("I",QPoint(0,0));
	Node *n_j = new Node("J",QPoint(0,0));
	
	m_graph.insert(n_a);
	m_graph.insert(n_b);
	m_graph.insert(n_c);
	m_graph.insert(n_d);
	m_graph.insert(n_e);
	m_graph.insert(n_f);
	m_graph.insert(n_g);
	m_graph.insert(n_h);
	m_graph.insert(n_i);
	m_graph.insert(n_j);

	m_graph.connect(n_a,n_b);
	m_graph.connect(n_a,n_c);
	m_graph.connect(n_c,n_d);
	m_graph.connect(n_c,n_e);
	m_graph.connect(n_e,n_f);
	m_graph.connect(n_f,n_h);
	m_graph.connect(n_f,n_g);
	m_graph.connect(n_f,n_i);
	m_graph.connect(n_i,n_j);

	m_graph.graphLayout();
}

#ifndef WINDOW_HH
#define WINDOW_HH

#include <QMainWindow>
#include <QGraphicsView>

#include <graph.hh>

class Window : public QMainWindow
{
	Q_OBJECT

public:
	Window(void);

private:
	Graph m_graph;
	QGraphicsView m_viewport;
};

#endif

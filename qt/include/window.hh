#ifndef WINDOW_HH
#define WINDOW_HH

#include <QMainWindow>

#include <actions.hh>
#include <procedurelist.hh>

class Window : public QMainWindow
{
	Q_OBJECT

public:
	Window(void);
	virtual ~Window(void);

private slots:
	void ensureFlowgraphWidget(void);
	void activate(po::proc_ptr proc);

private:
	QTabWidget *m_tabs;
	ProcedureList *m_procList;
	FilterWidget *m_filterWidget;
	po::flow_ptr m_flowgraph;
	QAction *m_action;
};

#endif

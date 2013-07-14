#ifndef WINDOW_HH
#define WINDOW_HH

#include <QMainWindow>

#include <flowgraphwidget.hh>
#include <procedurewidget.hh>
#include <filterwidget.hh>

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
	FlowgraphWidget *m_flowView;
	ProcedureWidget *m_procView;
	ProcedureList *m_procList;
	FilterWidget *m_filterWidget;
	po::flow_ptr m_flowgraph;
	QAction *m_action;
};

#endif

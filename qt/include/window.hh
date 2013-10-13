#include <QMainWindow>

#include <procedurelist.hh>
#include <filterwidget.hh>

#pragma once

class Window : public QMainWindow
{
	Q_OBJECT
	Q_PROPERTY(po::flow_ptr flowgraph READ flowgraph WRITE setFlowgraph)

public:
	Window(void);
	virtual ~Window(void);

	po::flow_ptr flowgraph(void);
	void setFlowgraph(po::flow_ptr f);

private slots:
	void ensureFlowgraphWidget(void);
	void activate(po::proc_ptr proc);

private:
	QTabWidget *m_tabs;
	ProcedureList *m_procList;
	FilterWidget *m_filterWidget;
	po::flow_ptr m_flowgraph;
	//QAction *m_action;
};

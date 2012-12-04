#ifndef WINDOW_HH
#define WINDOW_HH

#include <QMainWindow>
#include <QDockWidget>
#include <QTableWidget>
//#include <QComboBox>
//#include <QTabWidget>
//#include <QSortFilterProxyModel>

//class Model;

#include <flowgraphwidget.hh>
#include <procedurewidget.hh>
#include <actions.hh>
#include <model.hh>
/*
class AddressSortProxy : public QSortFilterProxyModel
{
	Q_OBJECT

public:
	AddressSortProxy(Model *m, QObject *parent = 0);

protected:
	virtual bool lessThan(const QModelIndex &left, const QModelIndex &right) const;
};

class ProcedureList : public QDockWidget
{
	Q_OBJECT

public:
	ProcedureList(Model *m, QWidget *parent = 0);
	
	QModelIndex currentFlowgraph(int column = 0) const;
	QItemSelectionModel *selectionModel(void);
	QAbstractProxyModel *model(void);

signals:
	void activated(const QModelIndex &idx);

private slots:
	void rebase(int i);

private:
	QTableView *m_list;
	QComboBox *m_combo;
	AddressSortProxy *m_proxy;
};*/

class ProcedureList : public QDockWidget
{
	Q_OBJECT

public:
	ProcedureList(po::flow_ptr flow, QWidget *parent = 0);
	
public slots:
	void snapshot(void);

private:
	po::flow_ptr m_flowgraph;
	QTableWidget m_list;
};

class Window : public QMainWindow
{
	Q_OBJECT

public:
	Window(void);
	virtual ~Window(void);

	Model *model(void);

private slots:
	void activate(const QModelIndex &idx);

private:
	Model *m_model;
	QTabWidget *m_tabs;
	FlowgraphWidget *m_flowView;
	ProcedureWidget *m_procView;
	ProcedureList *m_procList;
	Disassemble *m_action;
};

#endif

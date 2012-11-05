#ifndef WINDOW_HH
#define WINDOW_HH

#include <QMainWindow>
#include <QDockWidget>
#include <QTableView>
#include <QComboBox>
#include <QSortFilterProxyModel>
#include <QAction>

#include <callgraph.hh>
#include <model.hh>

class DisplayProcedureAction : public QAction
{
	Q_OBJECT

public:
	DisplayProcedureAction(QObject *parent = 0);

public slots:
	void activate(const QModelIndex &idx);
};

class AddressSortProxy : public QSortFilterProxyModel
{
	Q_OBJECT

public:
	AddressSortProxy(Model *m, QObject *parent = 0);

protected:
	virtual bool lessThan(const QModelIndex &left, const QModelIndex &right) const;
};

class ProcedureView : public QDockWidget
{
	Q_OBJECT

public:
	ProcedureView(Model *m, QWidget *parent = 0);
	
	QModelIndex currentFlowgraph(int column = 0) const;
	QItemSelectionModel *selectionModel(void);
	QAbstractItemModel *model(void);

signals:
	void activated(const QModelIndex &idx);

private slots:
	void rebase(int i);

private:
	QTableView *m_list;
	QComboBox *m_combo;
	AddressSortProxy *m_proxy;
};

class Window : public QMainWindow
{
	Q_OBJECT

public:
	Window(void);
	virtual ~Window(void);

private:
	Model *m_model;
	Callgraph *m_callgraph;
	ProcedureView *m_procView;
};

#endif

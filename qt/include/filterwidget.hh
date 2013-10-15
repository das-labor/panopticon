#include <QDockWidget>
#include <QTreeWidget>

#include <source.hh>

#pragma once

/*
class ProcedureListItem : public QSTableWidgetItem
{
public:
	enum Field
	{
		EntryPoint,
		Name,
		Size,
	};

	ProcedureListItem(po::proc_ptr p, Field f);

	po::proc_ptr procedure(void) const;
	Field field(void) const;

	virtual bool operator<(const QTableWidgetItem &i) const;
	virtual QTableWidgetItem *clone(void) const;

protected:
	po::proc_ptr m_procedure;
};*/

class FilterWidget : public QDockWidget
{
	Q_OBJECT

public:
	FilterWidget(QWidget *parent = 0);

/*public slots:
	void currentChanged(const QModelIndex &, const QModelIndex &);

signals:
	void activated(po::proc_ptr proc);
	void selected(po::proc_ptr proc);*/
	std::list<std::pair<po::rrange,po::address_space>> projection(void) const;

signals:
	void projectionChanged(const std::list<std::pair<po::rrange,po::address_space>> &proj);

private:
	po::graph<po::address_space,po::rrange> m_graph;
	QTreeWidget m_view;

/*private slots:
	void activateItem(QTableWidgetItem *tw);*/
};

#ifndef PROCEDURELIST_HH
#define PROCEDURELIST_HH

#include <QDockWidget>
#include <QTableWidget>
#include <QTableWidgetItem>

#include <flowgraph.hh>
#include <procedure.hh>

Q_DECLARE_METATYPE(po::flow_ptr)

class ProcedureListItem : public QTableWidgetItem
{
public:
	enum Field
	{
		EntryPoint,
		Name,
		Size,
	};

	ProcedureListItem(po::proc_ptr p, Field f);
	virtual ~ProcedureListItem(void);

	po::proc_ptr procedure(void) const;
	Field field(void) const;

	virtual bool operator<(const QTableWidgetItem &i) const;
	virtual QTableWidgetItem *clone(void) const;

protected:
	po::proc_ptr m_procedure;
};

class ProcedureList : public QDockWidget
{
	Q_OBJECT
	Q_PROPERTY(po::flow_ptr flowgraph READ flowgraph WRITE setFlowgraph)

public:
	ProcedureList(QWidget *parent = 0);
	virtual ~ProcedureList(void);

	po::flow_ptr flowgraph(void);
	void setFlowgraph(po::flow_ptr f);

public slots:
	void select(po::proc_ptr proc);
	void currentChanged(const QModelIndex &, const QModelIndex &);

signals:
	void activated(po::proc_ptr proc);
	void selected(po::proc_ptr proc);

private:
	po::flow_ptr m_flowgraph;
	QTableWidget m_list;
	std::map<po::proc_ptr,QTableWidgetItem *> m_procedureToItem;
	std::map<QTableWidgetItem *,po::proc_ptr> m_itemToProcedure;

private slots:
	void activateItem(QTableWidgetItem *tw);
};

#endif

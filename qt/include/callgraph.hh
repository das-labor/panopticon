#ifndef CALLGRAPH_HH
#define CALLGRAPH_HH

#include <graph.hh>

class Callgraph : public Graph
{
	Q_OBJECT

public:
	Callgraph(QAbstractItemModel *m, QModelIndex i, QItemSelectionModel *s, QWidget *parent = 0);
	virtual ~Callgraph(void);

signals:
	void activated(const QModelIndex &idx);

protected:
	void populate(void);
	virtual void mouseDoubleClickEvent(QMouseEvent *event);

private slots:
	void sceneSelectionChanged(void);
	void modelSelectionChanged(const QItemSelection &selected, const QItemSelection &deselected);
	void dataChanged(const QModelIndex &topLeft, const QModelIndex &bottomRight);

private:
	std::map<ptrdiff_t,Node *> m_uid2procedure;
	std::map<Node *,int> m_procedure2row;
	QItemSelectionModel *m_selection;
};

#endif

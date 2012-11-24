#ifndef FLOWGRAPHWIDGET_HH
#define FLOWGRAPHWIDGET_HH

#include <graphwidget.hh>

class FlowgraphWidget : public GraphWidget
{
	Q_OBJECT

public:
	FlowgraphWidget(QAbstractItemModel *m, QModelIndex i, QItemSelectionModel *s, QWidget *parent = 0);
	virtual ~FlowgraphWidget(void);

signals:
	void activated(const QModelIndex &idx);

protected:
	void populate(void);
	virtual void mouseDoubleClickEvent(QMouseEvent *event);

private slots:
	void sceneSelectionChanged(void);
	void modelSelectionChanged(const QItemSelection &selected, const QItemSelection &deselected);
	void dataChanged(const QModelIndex &topLeft, const QModelIndex &bottomRight);
	void sceneRectChanged(const QRectF &r);

private:
	std::map<ptrdiff_t,Node *> m_uid2procedure;
	std::map<Node *,int> m_procedure2row;
	QItemSelectionModel *m_selection;
};

#endif

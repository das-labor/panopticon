#ifndef CFLOWGRAPH_HH
#define CFLOWGRAPH_HH

#include <graph.hh>

class CFlowgraph : public Graph
{
	Q_OBJECT

public:
	CFlowgraph(QAbstractItemModel *m, QModelIndex i, QWidget *parent = 0);

protected:
	void populate(void);
};

#endif

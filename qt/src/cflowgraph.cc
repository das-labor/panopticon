#include <QDebug>
#include <cflowgraph.hh>

CFlowgraph::CFlowgraph(QAbstractItemModel *m, QModelIndex i, QWidget *parent)
: Graph(m,i,parent) 
{
	populate();
};

void CFlowgraph::populate(void)
{
	qDebug() << "pop cfg";
}

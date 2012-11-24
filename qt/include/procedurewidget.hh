#ifndef PROCEDURE_WIDGET_HH
#define PROCEDURE_WIDGET_HH

#include <graphwidget.hh>

class ProcedureWidget : public GraphWidget
{
	Q_OBJECT

public:
	ProcedureWidget(QAbstractItemModel *m, QModelIndex i, QWidget *parent = 0);

protected:
	void populate(void);
};

#endif

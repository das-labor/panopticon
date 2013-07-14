#ifndef PROCEDURE_WIDGET_HH
#define PROCEDURE_WIDGET_HH

#include <flowgraph.hh>
#include <procedure.hh>

class ProcedureWidget : public GraphWidget
{
	Q_OBJECT

public:
	ProcedureWidget(po::flow_ptr f, po::proc_ptr p, QWidget *parent = 0);

	void setProcedure(po::proc_ptr p);

protected:
	void snapshot(void);

private:
	po::flow_ptr m_flowgraph;
	po::proc_ptr m_procedure;
};

#endif

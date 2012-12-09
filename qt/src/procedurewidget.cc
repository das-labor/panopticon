#include <map>

#include <basic_block.hh>

#include <basicblockwidget.hh>
#include <controltransferwidget.hh>
#include <procedurewidget.hh>

ProcedureWidget::ProcedureWidget(po::flow_ptr f, po::proc_ptr p, QWidget *parent)
: GraphWidget(parent), m_flowgraph(f), m_procedure(0)
{
	assert(f);
	setProcedure(p);
}

void ProcedureWidget::setProcedure(po::proc_ptr p)
{
	m_procedure = p;
	snapshot();
}

void ProcedureWidget::snapshot(void)
{
	qDebug() << "snapshot" << QString::fromStdString(m_procedure->name) << "of" << QString::fromStdString(m_flowgraph->name);
	
	std::map<po::bblock_ptr,BasicBlockWidget *> nodes;
	std::lock_guard<std::mutex> guard(m_flowgraph->mutex);

	m_scene.clear();

	// nodes
	for(po::bblock_ptr bb: m_procedure->basic_blocks)
	{
		auto m = new BasicBlockWidget(m_flowgraph,m_procedure,bb,0);

		m_scene.insert(m);
		nodes.insert(make_pair(bb,m));
	}
	
	// edges
	for(po::bblock_ptr bb: m_procedure->basic_blocks)
	{
		BasicBlockWidget *from = nodes[bb];
		
		for(const po::ctrans &ct: bb->outgoing())
		{
			if(ct.bblock)
			{
				BasicBlockWidget *to = nodes[ct.bblock];

				m_scene.connect(new ControlTransferWidget(ct.guard,from,to));
			}
		}
	}

	m_scene.layoutHierarchically();
	BasicBlockWidget *e = nodes[m_procedure->entry];

	assert(e);
	centerOn(e->pos() + e->boundingRect().center());
}

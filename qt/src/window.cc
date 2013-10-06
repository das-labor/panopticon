#include <functional>
#include <thread>
#include <fstream>

#include <QVBoxLayout>
#include <QDebug>
#include <QStatusBar>

#include <window.hh>

#include <avr/avr.hh>

Window::Window(void)
: m_flowView(0), m_procView(0), m_filterWidget(0), m_flowgraph(new po::flowgraph())
{
	setWindowTitle("Panopticum v0.8");
	resize(1000,800);
	move(500,200);

	m_tabs = new QTabWidget(this);
	m_procList = new ProcedureList(m_flowgraph,this);
	m_filterWidget = new FilterWidget(this);
	//m_action = new Disassemble("../sosse",flow,[&](po::proc_ptr p, unsigned int i) { if(p) m_procList->snapshot(); },this);

	setCentralWidget(m_tabs);
	addDockWidget(Qt::LeftDockWidgetArea,m_procList);
	addDockWidget(Qt::LeftDockWidgetArea,m_filterWidget);

	connect(m_procList,SIGNAL(activated(po::proc_ptr)),this,SLOT(activate(po::proc_ptr)));

	new std::thread([this](QStatusBar *st)
	{
		//QStatusBar *st = statusBar();
		std::ifstream fs("sosse");
		std::vector<uint16_t> bytes;

		if (fs.bad())
        std::cout << "I/O error while reading" << std::endl;
    else if (fs.fail())
        std::cout << "Non-integer data encountered" << std::endl;
		else
		{
			QMetaObject::invokeMethod(st,"showMessage",Qt::QueuedConnection,Q_ARG(QString,"Reading..."));
			while(fs.good() && !fs.eof())
			{
				uint16_t c;
				fs.read((char *)&c,sizeof(c));
				bytes.push_back(c);
			}

			QMetaObject::invokeMethod(st,"showMessage",Qt::QueuedConnection,Q_ARG(QString,"Disassembling..."));
			po::avr::disassemble(bytes,0,m_flowgraph,[&](unsigned int done, unsigned int todo)
			{
				QMetaObject::invokeMethod(m_procList,"snapshot",Qt::QueuedConnection);
				QMetaObject::invokeMethod(st,"showMessage",Qt::QueuedConnection,Q_ARG(QString,QString("Disassembling... (%1 of %2 pending)").arg(todo).arg(done)));
			});
			QMetaObject::invokeMethod(st,"showMessage",Qt::QueuedConnection,Q_ARG(QString,"Done"),Q_ARG(int,10));
			QMetaObject::invokeMethod(this,"ensureFlowgraphWidget",Qt::QueuedConnection);
		}
	},statusBar());
}

Window::~Window(void)
{
	// pervents null dereference if m_procView still has selections
	//m_procList->selectionModel()->clear();
	//delete m_flowView;
}

void Window::activate(po::proc_ptr proc)
{
	assert(proc);

	qDebug() << QString::fromStdString(proc->name) << "activated!";
	if(!m_procView)
	{
		m_procView = new ProcedureWidget(m_flowgraph,proc,this);
		m_tabs->addTab(m_procView,"Control Flow Graph");
	}
	else
		m_procView->setProcedure(proc);
	m_tabs->setCurrentWidget(m_procView);
}

void Window::ensureFlowgraphWidget(void)
{
	if(!m_flowView)
	{
		m_flowView = new FlowgraphWidget(m_flowgraph,this);
		connect(m_flowView,SIGNAL(selected(po::proc_ptr)),m_procList,SLOT(select(po::proc_ptr)));
		connect(m_procList,SIGNAL(selected(po::proc_ptr)),m_flowView,SLOT(select(po::proc_ptr)));
	}

	if(m_tabs->indexOf(m_flowView) == -1)
		m_tabs->addTab(m_flowView,"Callgraph");
}

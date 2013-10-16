#include <functional>
#include <thread>
#include <fstream>

#include <QVBoxLayout>
#include <QDebug>
#include <QStatusBar>
#include <QCoreApplication>
#include <QQuickView>
#include <QQuickItem>
#include <QAbstractItemModel>
#include <QDebug>
#include <QQmlApplicationEngine>
#include <QQmlComponent>
#include <QQmlContext>

#include "window.hh"

#include "linearscene.hh"
#include <avr/avr.hh>

Window::Window(void)
{
	setWindowTitle("Panopticum v0.8");
	resize(1000,800);
	move(500,200);

	m_tabs = new QTabWidget(this);
	m_procList = new ProcedureList(this);
	m_filterWidget = new FilterWidget(this);

	//m_action = new Disassemble("../sosse",flow,[&](po::proc_ptr p, unsigned int i) { if(p) m_procList->snapshot(); },this);
	//m_action = new Open(QCoreApplication::arguments().at(1),this);

	setCentralWidget(m_tabs);
	addDockWidget(Qt::LeftDockWidgetArea,m_procList);
	addDockWidget(Qt::LeftDockWidgetArea,m_filterWidget);

	LinearSceneModel *lsm = new LinearSceneModel();

	lsm->setGraph(m_filterWidget->graph());
	connect(m_filterWidget,SIGNAL(graphChanged(const po::graph<po::address_space,po::rrange> &)),
					lsm,SLOT(setGraph(const po::graph<po::address_space,po::rrange> &)));

	auto view = new QQuickView();
  view->rootContext()->setContextProperty("linearModel", lsm);
	view->setResizeMode(QQuickView::SizeRootObjectToView);
	view->setSource(QUrl("qrc:/Hex.qml"));
	QObject::connect(view->rootObject(),SIGNAL(select(int,int,int,int)),lsm,SLOT(select(int,int,int,int)));
 	QWidget *container = QWidget::createWindowContainer(view);
  container->setMinimumSize(200, 200);
	m_tabs->addTab(container,QString::fromStdString("Hexdump"));
	connect(m_procList,SIGNAL(activated(po::proc_ptr)),this,SLOT(activate(po::proc_ptr)));

	//m_action->trigger();
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
	/*if(!m_procView)
	{
		m_procView = new ProcedureWidget(m_flowgraph,proc,this);
		m_tabs->addTab(m_procView,"Control Flow Graph");
	}
	else
		m_procView->setProcedure(proc);
	m_tabs->setCurrentWidget(m_procView);*/
}

void Window::ensureFlowgraphWidget(void)
{
/*	if(!m_flowView)
	{
		m_flowView = new FlowgraphWidget(m_flowgraph,this);
		connect(m_flowView,SIGNAL(selected(po::proc_ptr)),m_procList,SLOT(select(po::proc_ptr)));
		connect(m_procList,SIGNAL(selected(po::proc_ptr)),m_flowView,SLOT(select(po::proc_ptr)));
	}

	if(m_tabs->indexOf(m_flowView) == -1)
		m_tabs->addTab(m_flowView,"Callgraph");*/
}

po::flow_ptr Window::flowgraph(void)
{
	return m_flowgraph;
}

void Window::setFlowgraph(po::flow_ptr f)
{
	m_flowgraph = f;
	m_procList->setFlowgraph(f);
}

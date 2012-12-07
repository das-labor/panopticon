#include <functional>
#include <thread>
#include <fstream>
#include <QVBoxLayout>
#include <QHeaderView>
#include <QDebug>
#include <QStatusBar>
#include <window.hh>
#include <deflate.hh>

#include <avr/avr.hh>

ProcedureList::ProcedureList(po::flow_ptr f, QWidget *parent)
: QDockWidget("Procedures",parent), m_flowgraph(f)
{
	m_list.horizontalHeader()->hide();
	m_list.horizontalHeader()->setStretchLastSection(true);
	m_list.setShowGrid(false);
	m_list.verticalHeader()->hide();
	m_list.setSelectionBehavior(QAbstractItemView::SelectRows);
	m_list.setSelectionMode(QAbstractItemView::SingleSelection);
	setWidget(&m_list);
	
	connect(&m_list,SIGNAL(itemActivated(QTableWidgetItem *)),this,SLOT(activateItem(QTableWidgetItem*)));
	connect(m_list.selectionModel(),SIGNAL(currentChanged(const QModelIndex&,const QModelIndex &)),this,SLOT(currentChanged(const QModelIndex&,const QModelIndex &)));
	snapshot();
}

void ProcedureList::snapshot(void)
{
	std::lock_guard<std::mutex> guard(m_flowgraph->mutex);

	m_list.clear();
	m_list.setColumnCount(2);
	m_list.setRowCount(m_flowgraph->procedures.size());

	unsigned int row = 0;
	for(po::proc_ptr p: m_flowgraph->procedures)
	{
		if(!p) continue;

		QTableWidgetItem *col0 = new QTableWidgetItem(p->entry ? QString("%1").arg(p->entry->area().begin) : "(no entry)");
		QTableWidgetItem *col1 = new QTableWidgetItem(p->name.size() ? QString::fromStdString(p->name) : "(unnamed)");

		col0->setFlags(Qt::ItemIsSelectable | Qt::ItemIsEnabled);
		col0->setData(Qt::UserRole,QVariant((qulonglong)p.get()));
		col1->setFlags(Qt::ItemIsSelectable | Qt::ItemIsEnabled);
		col1->setData(Qt::UserRole,QVariant((qulonglong)p.get()));

		m_list.setItem(row,0,col0);
		m_list.setItem(row,1,col1);

		++row;
	}
	
	m_list.resizeRowsToContents();
	m_list.resizeColumnsToContents();
	m_list.sortItems(1,Qt::AscendingOrder);
}

void ProcedureList::select(po::proc_ptr proc)
{
	int row = 0;

	if(!proc)
	{
		m_list.setCurrentItem(0);
		emit selected(nullptr);
		return;
	}

	while(row < m_list.rowCount())
	{
		QTableWidgetItem *item = m_list.item(row++,0);
		
		if(item->data(Qt::UserRole).toULongLong() == (qulonglong)proc.get())
		{
			if(m_list.currentItem() != item)
			{
				m_list.setCurrentItem(item);
				emit selected(proc);
			}
			return;
		}
	}
}

void ProcedureList::currentChanged(const QModelIndex &current, const QModelIndex &previous)
{
	QTableWidgetItem *i = m_list.currentItem();
	
	if(!i) 
	{
		emit selected(nullptr);
		return;
	}
	
	for(po::proc_ptr p: m_flowgraph->procedures)
		if((qulonglong)p.get() == i->data(Qt::UserRole).toULongLong())
			emit selected(p);
}

void ProcedureList::activateItem(QTableWidgetItem *tw)
{
	assert(tw);
	
	QString name = tw->text();
	std::lock_guard<std::mutex> guard(m_flowgraph->mutex);

	for(po::proc_ptr p: m_flowgraph->procedures)
		if(QString::fromStdString(p->name) == name)
		{
			emit activated(p);
			return;
		}
	
	assert(false);
}

Window::Window(void)
: m_flowView(0), m_procView(0), m_flowgraph(new po::flowgraph())
{
	setWindowTitle("Panopticum v0.8");
	resize(1000,800);
	move(500,200);

	m_tabs = new QTabWidget(this);
	m_procList = new ProcedureList(m_flowgraph,this);
	//m_action = new Disassemble("../sosse",flow,[&](po::proc_ptr p, unsigned int i) { if(p) m_procList->snapshot(); },this);

	setCentralWidget(m_tabs);
	addDockWidget(Qt::LeftDockWidgetArea,m_procList);

	connect(m_procList,SIGNAL(activated(po::proc_ptr)),this,SLOT(activate(po::proc_ptr)));

	//m_action->trigger(
	new std::thread([this](QStatusBar *st)
	{
		std::ifstream fs("../sosse");
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
			po::avr::disassemble(bytes,0,m_flowgraph,[&](void)
			{
				QMetaObject::invokeMethod(m_procList,"snapshot",Qt::QueuedConnection);
	//			QMetaObject::invokeMethod(fw,"snapshot",Qt::QueuedConnection);
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

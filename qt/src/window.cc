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
/*
AddressSortProxy::AddressSortProxy(Model *m, QObject *parent)
: QSortFilterProxyModel(parent)
{
	setSourceModel(m);
	setDynamicSortFilter(true);
}

bool AddressSortProxy::lessThan(const QModelIndex &left, const QModelIndex &right) const
{
	return sourceModel()->data(left,Qt::DisplayRole).toString().toULongLong(0,0) < 
				 sourceModel()->data(right,Qt::DisplayRole).toString().toULongLong(0,0);
}

ProcedureList::ProcedureList(Model *m, QWidget *parent)
: QDockWidget("Procedures",parent), m_list(new QTableView(this)), m_combo(new QComboBox(this)), m_proxy(new AddressSortProxy(m,this))
{
	QWidget *w = new QWidget(this);
	QVBoxLayout *l = new QVBoxLayout(w);
	l->addWidget(m_combo);
	l->addWidget(m_list);
	w->setLayout(l);
	setWidget(w);

	m_combo->setModel(m_proxy);
	m_list->setModel(m_proxy);

	m_list->horizontalHeader()->hideSection(Model::UniqueIdColumn);
	m_list->horizontalHeader()->moveSection(Model::BasicBlocksColumn,Model::AreaColumn);
	m_list->horizontalHeader()->hide();
	m_list->horizontalHeader()->setStretchLastSection(true);
	m_list->setShowGrid(false);
	m_list->verticalHeader()->hide();
	m_list->setSelectionBehavior(QAbstractItemView::SelectRows);
	m_list->setSortingEnabled(true);


	connect(m_list,SIGNAL(activated(const QModelIndex&)),this,SIGNAL(activated(const QModelIndex&)));
	connect(m_combo,SIGNAL(currentIndexChanged(int)),this,SLOT(rebase(int)));
	rebase(0);
}

QModelIndex ProcedureList::currentFlowgraph(int column) const
{
	return m_proxy->index(m_combo->currentIndex(),column);
}

QItemSelectionModel *ProcedureList::selectionModel(void)
{
	return m_list->selectionModel();
}

void ProcedureList::rebase(int i)
{
	qDebug() << "rebase:" << i;
	m_list->setRootIndex(currentFlowgraph(Model::ProceduresColumn));
	m_list->resizeRowsToContents();
	m_list->resizeColumnsToContents();
	m_list->sortByColumn(1,Qt::AscendingOrder);
}

QAbstractProxyModel *ProcedureList::model(void)
{
	return m_proxy;
}*/

ProcedureList::ProcedureList(po::flow_ptr f, QWidget *parent)
: QDockWidget("Procedures",parent), m_flowgraph(f)
{
	m_list.horizontalHeader()->hide();
	m_list.horizontalHeader()->setStretchLastSection(true);
	m_list.setShowGrid(false);
	m_list.verticalHeader()->hide();
	m_list.setSelectionBehavior(QAbstractItemView::SelectRows);
	setWidget(&m_list);
	
	connect(&m_list,SIGNAL(itemActivated(QTableWidgetItem *)),this,SLOT(activateItem(QTableWidgetItem*)));
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
		col1->setFlags(Qt::ItemIsSelectable | Qt::ItemIsEnabled);
		m_list.setItem(row,0,col0);
		m_list.setItem(row,1,col1);
		++row;
	}
	
	m_list.resizeRowsToContents();
	m_list.resizeColumnsToContents();
	m_list.sortItems(1,Qt::AscendingOrder);
	//m_list.update();
	//update();
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
		m_flowView = new FlowgraphWidget(m_flowgraph,0/*m_procList->selectionModel()*/,this);
	
	if(m_tabs->indexOf(m_flowView) == -1)
		m_tabs->addTab(m_flowView,"Callgraph");
}

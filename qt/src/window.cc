#include <functional>
#include <QVBoxLayout>
#include <QHeaderView>
#include <QDebug>

#include <window.hh>
#include <deflate.hh>

AddressSortProxy::AddressSortProxy(Model *m, QObject *parent)
: QSortFilterProxyModel(parent)
{
	setSourceModel(m);
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

	m_list->horizontalHeader()->hideSection(2);
	m_list->horizontalHeader()->moveSection(0,1);
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
}

Window::Window(po::flow_ptr f)
{
	setWindowTitle("Panopticum v0.8");
	resize(1000,800);
	move(500,200);

	std::string path("test.ttl");

	m_tabs = new QTabWidget(this);
	m_model = new Model(f,this);
	m_procList = new ProcedureList(m_model,this);
	m_flowView = new FlowgraphWidget(m_procList->model(),m_procList->currentFlowgraph(),m_procList->selectionModel(),this);
	m_procView = 0;

	m_tabs->addTab(m_flowView,"Callgraph");

	setCentralWidget(m_tabs);
	addDockWidget(Qt::LeftDockWidgetArea,m_procList);

	connect(m_procList,SIGNAL(activated(const QModelIndex&)),this,SLOT(activate(const QModelIndex&)));
	connect(m_flowView,SIGNAL(activated(const QModelIndex&)),this,SLOT(activate(const QModelIndex&)));
}

Window::~Window(void)
{
	// pervents null dereference if m_procView still has selections
	m_procList->selectionModel()->clear();
	delete m_flowView;
}

void Window::activate(const QModelIndex &idx)
{
	assert(idx.isValid());
	const QAbstractItemModel *model = idx.model();

	qDebug() << model->data(idx.sibling(idx.row(),Model::NameColumn)).toString() << "activated!";
	if(!m_procView)
	{
		m_procView = new ProcedureWidget(m_model,m_procList->model()->mapToSource(idx),this);
		m_tabs->addTab(m_procView,"Control Flow Graph");
	}
	else
		m_procView->setRootIndex(m_procList->model()->mapToSource(idx));
	m_tabs->setCurrentWidget(m_procView);
}

#include <functional>
#include <QVBoxLayout>
#include <QHeaderView>
#include <QDebug>

#include <window.hh>
#include <database.hh>

DisplayProcedureAction::DisplayProcedureAction(QObject *parent)
: QAction("Display",parent) {}

void DisplayProcedureAction::activate(const QModelIndex &idx)
{
	assert(idx.isValid());
	const QAbstractItemModel *model = idx.model();

	qDebug() << model->data(idx.sibling(idx.row(),Model::NameColumn)).toString() << "activated!";
}

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

ProcedureView::ProcedureView(Model *m, QWidget *parent)
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

QModelIndex ProcedureView::currentFlowgraph(int column) const
{
	return m_proxy->index(m_combo->currentIndex(),column);
}

QItemSelectionModel *ProcedureView::selectionModel(void)
{
	return m_list->selectionModel();
}

void ProcedureView::rebase(int i)
{
	m_list->setRootIndex(currentFlowgraph(Model::ProceduresColumn));
	m_list->resizeRowsToContents();
	m_list->resizeColumnsToContents();
	m_list->sortByColumn(1,Qt::AscendingOrder);
}

QAbstractItemModel *ProcedureView::model(void)
{
	return m_proxy;
}

Window::Window(void)
{
	setWindowTitle("Panopticum v0.8");
	resize(1000,800);
	move(500,200);

	std::string path("test.ttl");

	m_model = new Model(new database(path),this);
	m_procView = new ProcedureView(m_model,this);
	m_callgraph = new Callgraph(m_procView->model(),m_procView->currentFlowgraph(),m_procView->selectionModel(),this);
	DisplayProcedureAction *disp = new DisplayProcedureAction(this);

	setCentralWidget(m_callgraph);
	addDockWidget(Qt::LeftDockWidgetArea,m_procView);

	connect(m_procView,SIGNAL(activated(const QModelIndex&)),disp,SLOT(activate(const QModelIndex&)));
	connect(m_callgraph,SIGNAL(activated(const QModelIndex&)),disp,SLOT(activate(const QModelIndex&)));
}

Window::~Window(void)
{
	// pervents null dereference if m_procView still has selections
	m_procView->selectionModel()->clear();
	delete m_callgraph;
}

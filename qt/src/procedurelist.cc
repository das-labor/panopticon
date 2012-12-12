#include <cassert>
#include <thread>
#include <set>

#include <QHeaderView>
#include <QDebug>

#include <procedurelist.hh>

ProcedureListItem::ProcedureListItem(po::proc_ptr p, Field f)
: QTableWidgetItem((int)f), m_procedure(p)
{
	assert(p);
	setFlags(Qt::ItemIsSelectable | Qt::ItemIsEnabled);

	switch(f)
	{
	case EntryPoint:
		setText(p->entry ? QString("%1").arg(p->entry->area().begin) : "(no entry)");
		break;
	case Name:
		setText(p->name.size() ? QString::fromStdString(p->name) : "(unnamed)");
		break;
	default:
		setText("(unknown field type)");
	}
}
	
po::proc_ptr ProcedureListItem::procedure(void) const
{
	assert(m_procedure);
	return m_procedure;
}

ProcedureListItem::Field ProcedureListItem::field(void) const
{
	return (Field)type();
}

bool ProcedureListItem::operator<(const QTableWidgetItem &i) const
{
	const ProcedureListItem *p = dynamic_cast<const ProcedureListItem *>(&i);
	assert(p && p->field() == field());

	switch(field())
	{
	case EntryPoint:
		return procedure()->entry->area().begin < p->procedure()->entry->area().begin;
	default:
		return text() < i.text();
	}
}

QTableWidgetItem *ProcedureListItem::clone(void) const
{
	return new ProcedureListItem(procedure(),field());
}

ProcedureList::ProcedureList(po::flow_ptr f, QWidget *parent)
: QDockWidget("Procedures",parent), m_flowgraph(f)
{
	m_list.horizontalHeader()->hide();
	m_list.horizontalHeader()->setStretchLastSection(true);
	m_list.setShowGrid(false);
	m_list.verticalHeader()->hide();
	m_list.setSelectionBehavior(QAbstractItemView::SelectRows);
	m_list.setSelectionMode(QAbstractItemView::SingleSelection);
	m_list.setColumnCount(2);
	setWidget(&m_list);
	
	connect(&m_list,SIGNAL(itemActivated(QTableWidgetItem *)),this,SLOT(activateItem(QTableWidgetItem*)));
	connect(m_list.selectionModel(),SIGNAL(currentChanged(const QModelIndex&,const QModelIndex &)),this,SLOT(currentChanged(const QModelIndex&,const QModelIndex &)));
	snapshot();
}

void ProcedureList::snapshot(void)
{
	std::lock_guard<std::mutex> guard(m_flowgraph->mutex);
	std::set<po::proc_ptr> known;

	int row = 0;
	while(row < m_list.rowCount())
	{
		ProcedureListItem *item = dynamic_cast<ProcedureListItem *>(m_list.item(row,0));
		assert(item);

		if(!m_flowgraph->procedures.count(item->procedure()))
		{
			m_list.removeRow(row);
		}
		else
		{
			++row;
			known.insert(item->procedure());
		}
	}
	
	m_list.setRowCount(m_flowgraph->procedures.size());
	
	assert(known.size() <= m_flowgraph->procedures.size());
	if(known.size() < m_flowgraph->procedures.size())
	{
		std::set<po::proc_ptr> todo;

		std::set_difference(m_flowgraph->procedures.begin(),m_flowgraph->procedures.end(),
												known.begin(),known.end(),
												std::inserter(todo,todo.begin()));

		for(po::proc_ptr p: todo)
		{
			ProcedureListItem *col0 = new ProcedureListItem(p,ProcedureListItem::EntryPoint);
			ProcedureListItem *col1 = new ProcedureListItem(p,ProcedureListItem::Name);

			m_list.setItem(row,0,col0);
			m_list.setItem(row,1,col1);

			++row;
		}
	}
	
	m_list.resizeRowsToContents();
	m_list.resizeColumnsToContents();
	m_list.sortItems(0,Qt::AscendingOrder);
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
		ProcedureListItem *item = dynamic_cast<ProcedureListItem *>(m_list.item(row++,0));
		
		if(item->procedure() == proc)
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
	ProcedureListItem *i = dynamic_cast<ProcedureListItem *>(m_list.currentItem());
	
	if(!i) 
	{
		emit selected(nullptr);
		return;
	}
	
	for(po::proc_ptr p: m_flowgraph->procedures)
		if(p == i->procedure())
			emit selected(p);
}

void ProcedureList::activateItem(QTableWidgetItem *i)
{
	ProcedureListItem *tw = dynamic_cast<ProcedureListItem *>(i);
	
	assert(tw);
	emit activated(tw->procedure());
}

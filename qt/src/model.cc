#include <model.hh>

// TODO: custom "Selection" role: None, Hover, Selected

Model::Model(database *db, QObject *parent)
: QAbstractItemModel(parent), m_database(db)
{
	// pointer tagging
	static_assert(alignof(flowgraph*) >= Model::LastTag,"Pointer alignment of >= Model::LastTag needed for tagging");
	assert(alignof(flowgraph*) >= Model::LastTag);
	
	flow_ptr flow = m_database->flowgraph();
	if(flow->name.empty())
		flow->name = "flowgraph #1";
	m_flowgraphs.push_back(flow);
	m_selected.insert(flow->procedures.begin()->get());
}

Model::~Model(void)
{
	delete m_database;
}

QModelIndex Model::index(int row, int column, const QModelIndex &parent) const
{
	flowgraph *flow;
	procedure *proc;
	Tag t;
	void *ptr;

	if(!parent.isValid())
	{
		// root
		assert(row >= 0 && (unsigned int)row < m_flowgraphs.size());
		return createIndex(row,column,tag(m_flowgraphs[row].get(),FlowgraphTag));
	}

	tie(ptr,t) = extract(parent.internalId());

	switch(t)
	{
	case FlowgraphTag:
		flow = (flowgraph *)ptr;
		
		if(parent.column() == ProceduresColumn)
		{
			auto i = flow->procedures.begin();
			assert(row >= 0 && flow->procedures.size() > (unsigned int)row);
			advance(i,row);
			return createIndex(row,column,tag(i->get(),ProcedureTag));
		}
		else
			assert(false);

	case ProcedureTag:
		proc = (procedure *)ptr;

		if(parent.column() == CalleesColumn)
		{
			assert(row >= 0 && proc->callees.size() > (unsigned int)row);
			return createIndex(row,column,tag(proc->callees[row].get(),ProcedureTag));
		}
		else
			assert(false);

	default:
		assert(false);
	}
}

QModelIndex Model::parent(const QModelIndex &index) const
{
	ptrdiff_t t;
	void *ptr;
	assert(index.isValid());
	
	tie(ptr,t) = extract(index.internalId());
	switch(t)
	{
	case FlowgraphTag:
		return QModelIndex();
	case ProcedureTag:
		return createIndex(0,ProceduresColumn,tag(m_flowgraphs.front().get(),FlowgraphTag));
	default:
		assert(false);
	}
}

int Model::rowCount(const QModelIndex &parent) const
{
	flowgraph *flow;
	procedure *proc;
	ptrdiff_t t;
	void *ptr;
	
	tie(ptr,t) = extract(parent.internalId());
	
	if(!parent.isValid())
	{
		return m_flowgraphs.size();
	}

	switch(t)
	{
	case FlowgraphTag:
		flow = (flowgraph *)ptr;

		if(parent.column() == ProceduresColumn)
			return flow->procedures.size();
		else
			return 0;
	
	case ProcedureTag:
		proc = (procedure *)ptr;

		if(parent.column() == CalleesColumn)
			return proc->callees.size();
		else
			return 0;

	default:
		assert(false);
	}
}

int Model::columnCount(const QModelIndex &parent) const
{
	ptrdiff_t t;
	void *ptr;
	
	tie(ptr,t) = extract(parent.internalId());
	
	if(!parent.isValid())
	{
		return LastFlowgraphColumn;
	}

	switch(t)
	{
	case FlowgraphTag:
		if(parent.column() == ProceduresColumn)
			return LastProcedureColumn;
		else
			return 0;

	case ProcedureTag:
		if(parent.column() == CalleesColumn)
			return LastProcedureColumn;
		else
			return 0;

	default:
		assert(false);
	}
}

QVariant Model::data(const QModelIndex &index, int role) const
{
	assert(index.isValid());
	
	switch(role)
	{
	case Qt::DisplayRole:
	case Qt::EditRole:
		return QVariant(displayData(index));
	/*case Qt::FontRole:
		return QVariant(fontData(index));	
	case SelectionRole:
		return QVariant(selectionData(index));*/
	default:
		return QVariant();
	}
}

Qt::ItemFlags Model::flags(const QModelIndex &index) const
{
	ptrdiff_t t;
	void *ptr;
	Qt::ItemFlags ret = Qt::ItemIsEnabled | Qt::ItemIsSelectable;

	if(!index.isValid())
		return ret;
	else
		tie(ptr,t) = extract(index.internalId());
	
	switch(t)
	{
	case FlowgraphTag:
	case ProcedureTag:
		return (index.column() == NameColumn ? ret | Qt::ItemIsEditable : ret);
	default:
		return ret;
	}
}

bool Model::setData(const QModelIndex &index, const QVariant &value, int role)
{
	assert(index.isValid());
	bool ret = false;
	
	switch(role)
	{
	case Qt::EditRole:
	case Qt::DisplayRole:
	{
		std::string s = value.toString().toStdString();
		ret = setDisplayData(index,s);
		break;
	}
/*	case SelectionRole:
		ret = setSelectionRole(index,value.toUInt());
		break;*/
	default:
		ret = false;
	}

	if(ret) emit dataChanged(index,index);
	return ret;
}

QString Model::displayData(const QModelIndex &index) const
{
	flowgraph *flow;
	procedure *proc;
	ptrdiff_t t;
	void *ptr;
	
	tie(ptr,t) = extract(index.internalId());

	switch(t)
	{
	case FlowgraphTag:
		flow = (flowgraph *)ptr;
		switch((Column)index.column())
		{
			case NameColumn: 			return QString::fromStdString(flow->name);
			case ProceduresColumn: return QString("%1 procedures").arg(flow->procedures.size());
			default: return QString();
		}
	case ProcedureTag:
		proc = (procedure *)ptr;
		switch((Column)index.column())
		{
		case NameColumn:
			if(proc->name.size())
				return QString::fromStdString(proc->name);
			else
				return QString("(unnamed)");
		case EntryPointColumn:
			if(proc->entry)
				return QString("0x%1").arg(proc->entry->area().begin);
			else
				return QString("(no entry)");
		case BasicBlocksColumn: 
		{
			auto p = proc->all();
			return QString("%1 basic blocks").arg(distance(p.first,p.second));
		}
		case CalleesColumn:
			return QString("%1 callees").arg(proc->callees.size());
		case UniqueIdColumn:
			return QString("%1").arg((ptrdiff_t)proc);
		default: 
			return QString();
		}
	
	default:
		return QString();
	}
}
/*
QFont Model::fontData(const QModelIndex &index) const
{
	procedure *proc;
	ptrdiff_t t;
	void *ptr;
	QFont ret;
	
	tie(ptr,t) = extract(index.internalId());

	switch(t)
	{
	case ProcedureTag:
		proc = (procedure *)ptr;
		
		if(m_hovered == proc) 
			ret.setItalic(true);
		if(m_selected == proc)
			ret.setBold(true);
	
	// fall through
	default:
		return ret;
	}
}

unsigned int Model::selectionData(const QModelIndex &index) const
{
	procedure *proc;
	ptrdiff_t t;
	void *ptr;
	unsigned int ret = NoSelection;
	
	tie(ptr,t) = extract(index.internalId());

	switch(t)
	{
	case ProcedureTag:
		proc = (procedure *)ptr;
		
		if(m_hovered == proc) 
			ret |= WeakSelection;
		if(m_selected == proc)
			ret |= StrongSelection;
	
	// fall through
	default:
		return ret;
	}
}*/

bool Model::setDisplayData(const QModelIndex &index, const std::string &value)
{
	ptrdiff_t t;
	void *ptr;
	flowgraph *flow;

	if(!value.size())
		return false;

	tie(ptr,t) = extract(index.internalId());
	switch(t)
	{
	case FlowgraphTag:
		assert(index.column() == NameColumn);
		flow = (flowgraph *)ptr;

		if(any_of(m_flowgraphs.begin(),m_flowgraphs.end(),[&](const flow_ptr &f) { return f->name == value; }))
			return false;

		flow->name = value;
		return true;
	
	case ProcedureTag:
	{
		assert(index.column() == NameColumn);
		procedure *proc = (procedure *)ptr;
		auto i = find_if(m_flowgraphs.begin(),m_flowgraphs.end(),[&](const flow_ptr &f) 
							{ return any_of(f->procedures.begin(),f->procedures.end(),[&](const proc_ptr &p) 
								{ return p.get() == proc; }); });

		if(i == m_flowgraphs.end())
			return false;

		if(any_of((*i)->procedures.begin(),(*i)->procedures.end(),[&](const proc_ptr &p) { return p.get() != proc && p->name == value; }))
			return false;

		proc->name = value;
		return true;
	}
	default:
		return false;
	}
}

/*
bool Model::setSelectionData(const QModelIndex &index, Selection value)
{
	ptrdiff_t t;
	void *ptr;

	tie(ptr,t) = extract(index.internalId());
	switch(t)
	{
	case ProcedureTag:
		if(value 
		assert(index.column() == NameColumn);
		procedure *proc = (procedure *)ptr;
		auto i = find_if(m_flowgraphs.begin(),m_flowgraphs.end(),[&](const flow_ptr &f) 
							{ return any_of(f->procedures.begin(),f->procedures.end(),[&](const proc_ptr &p) 
								{ return p.get() == proc; }); });

		if(i == m_flowgraphs.end())
			return false;

		if(any_of((*i)->procedures.begin(),(*i)->procedures.end(),[&](const proc_ptr &p) { return p.get() != proc && p->name == value.toString().toStdString(); }))
			return false;

		proc->name = value;
		return true;
	}
	default:
		return false;
	}
}*/

ptrdiff_t Model::tag(void *ptr, Tag t) const
{
	return (ptrdiff_t)ptr | (ptrdiff_t)t;
}

std::pair<void*,Model::Tag> Model::extract(ptrdiff_t p) const
{
	return std::make_pair((void *)(p & ~(ptrdiff_t)MaskTag),(Tag)(p & (ptrdiff_t)MaskTag));
}

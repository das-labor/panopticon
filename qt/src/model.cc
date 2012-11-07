#include <model.hh>

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

		switch(parent.column())
		{
		case CalleesColumn:
			assert(row >= 0 && proc->callees.size() > (unsigned int)row);
			return createIndex(row,column,tag(proc->callees[row].get(),ProcedureTag));
		
		case BasicBlocksColumn:
			assert(row >= 0 && proc->basic_blocks.size() > (unsigned int)row);
			return createIndex(row,column,tag(next(proc->basic_blocks.begin(),row)->get(),BasicBlockTag));
		
		default:
			assert(false);
		}

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
	case BasicBlockTag:
	{
		flow_ptr f = m_flowgraphs.front();
		auto i = find_if(f->procedures.begin(),f->procedures.end(),[&](const proc_ptr &p) 
			{ return any_of(p->basic_blocks.begin(),p->basic_blocks.end(),[&](const bblock_ptr &bb) { return bb.get() == (basic_block *)ptr; }); });
		
		assert(i != f->procedures.end());
		return createIndex(distance(f->procedures.begin(),i),BasicBlocksColumn,tag(i->get(),ProcedureTag));
	}
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

		switch(parent.column())
		{
		case CalleesColumn:
			return proc->callees.size();
		case BasicBlocksColumn:
			return proc->basic_blocks.size();
		default:
			return 0;
		}
	
	case BasicBlockTag:
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
	
	case BasicBlockTag:
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
	basic_block *bblock;
	ptrdiff_t t;
	void *ptr;
	
	tie(ptr,t) = extract(index.internalId());

	switch(t)
	{
	case FlowgraphTag:
		flow = (flowgraph *)ptr;
		switch((Column)index.column())
		{
		case NameColumn:
			return QString::fromStdString(flow->name);
		case ProceduresColumn: 
			return QString("%1 procedures").arg(flow->procedures.size());
		default: 
			assert(false);
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
			return QString("%1 basic blocks").arg(distance(proc->basic_blocks.begin(),proc->basic_blocks.end()));
		case CalleesColumn:
			return QString("%1 callees").arg(proc->callees.size());
		case UniqueIdColumn:
			return QString("%1").arg((ptrdiff_t)proc);
		default:
			assert(false);
		}

	case BasicBlockTag:
		bblock = (basic_block *)ptr;
		switch((Column)index.column())
		{
		case AreaColumn:
			return QString("%1:%2").arg(bblock->area().begin).arg(bblock->area().end);
		case MnemonicsColumn:
			return QString("%1 mnemonics").arg(bblock->mnemonics().size());
		case PredecessorsColumn:
		{
			auto p = bblock->predecessors();
			return QString("%1 predecessors").arg(distance(p.first,p.second));
		}
		case SuccessorsColumn:
		{
			auto p = bblock->successors();
			return QString("%1 successors").arg(distance(p.first,p.second));
		}
		default:
			assert(false);
		}

	default:
		assert(false);
	}
}

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

ptrdiff_t Model::tag(void *ptr, Tag t) const
{
	return (ptrdiff_t)ptr | (ptrdiff_t)t;
}

std::pair<void*,Model::Tag> Model::extract(ptrdiff_t p) const
{
	return std::make_pair((void *)(p & ~(ptrdiff_t)MaskTag),(Tag)(p & (ptrdiff_t)MaskTag));
}

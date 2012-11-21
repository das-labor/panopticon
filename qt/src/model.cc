#include <model.hh>

#include <flowgraph.hh>
#include <procedure.hh>
#include <mnemonic.hh>

Model::Model(po::deflate *d, QObject *parent)
: QAbstractItemModel(parent), m_deflate(d)
{
	// pointer tagging
	static_assert(alignof(po::flowgraph*) >= Model::LastTag,"Pointer alignment of >= Model::LastTag needed for tagging");
	assert(alignof(po::flowgraph*) >= Model::LastTag);
	
	po::flow_ptr flow = m_deflate->flowgraph();
	if(flow->name.empty())
		flow->name = "flowgraph #1";
	m_flowgraphs.push_back(flow);
	m_selected.insert(flow->procedures.begin()->get());
}

Model::~Model(void)
{
	delete m_deflate;
}

QModelIndex Model::index(int row, int column, const QModelIndex &parent) const
{
	po::flowgraph *flow;
	po::procedure *proc;
	po::basic_block *bblock;
	Tag t;
	void *ptr;

	if(!parent.isValid())
	{
		// root
		assert(row >= 0 && (unsigned int)row < m_flowgraphs.size());
		return createIndex(row,column,tag(m_flowgraphs[row].get(),FlowgraphTag));
	}

	std::tie(ptr,t) = extract(parent.internalId());

	switch(t)
	{
	case FlowgraphTag:
		flow = (po::flowgraph *)ptr;
		
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
		proc = (po::procedure *)ptr;

		switch(parent.column())
		{
		case CalleesColumn:
			assert(row >= 0 && proc->callees.size() > (unsigned int)row);
			return createIndex(row,column,tag(next(proc->callees.begin(),row)->get(),ProcedureTag));
		
		case BasicBlocksColumn:
			assert(row >= 0 && proc->basic_blocks.size() > (unsigned int)row);
			return createIndex(row,column,tag(next(proc->basic_blocks.begin(),row)->get(),BasicBlockTag));
		
		default:
			assert(false);
		}
	
	case BasicBlockTag:
		bblock = (po::basic_block *)ptr;

		switch(parent.column())
		{
		case SuccessorsColumn:
		{
			auto p = bblock->successors();
			assert(row >= 0 && bblock->outgoing().size() > (unsigned int)row);
			return createIndex(row,column,tag(next(p.first,row)->get(),BasicBlockTag));
		}

		case PredecessorsColumn:
		{
			auto p = bblock->predecessors();
			assert(row >= 0 && bblock->incoming().size() > (unsigned int)row);
			return createIndex(row,column,tag(next(p.first,row)->get(),BasicBlockTag));
		}

		case MnemonicsColumn:
		{
			assert(row >= 0 && bblock->mnemonics().size() > (unsigned int)row);
			return createIndex(row,column,tag((void *)&bblock->mnemonics()[row],MnemonicTag));
		}

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
	
	std::tie(ptr,t) = extract(index.internalId());
	switch(t)
	{
	case FlowgraphTag:
		return QModelIndex();
	case ProcedureTag:
		return createIndex(0,ProceduresColumn,tag(m_flowgraphs.front().get(),FlowgraphTag));
	case BasicBlockTag:
	{
		po::flow_ptr f = m_flowgraphs.front();
		auto i = std::find_if(f->procedures.begin(),f->procedures.end(),[&](const po::proc_ptr &p) 
			{ return std::any_of(p->basic_blocks.begin(),p->basic_blocks.end(),[&](const po::bblock_ptr &bb) { return bb.get() == (po::basic_block *)ptr; }); });
		
		assert(i != f->procedures.end());
		return createIndex(distance(f->procedures.begin(),i),BasicBlocksColumn,tag(i->get(),ProcedureTag));
	}
	case MnemonicTag:
	{
		const po::mnemonic *mne = (const po::mnemonic *)ptr;
		for(po::flow_ptr flow: m_flowgraphs)
			for(po::proc_ptr proc: flow->procedures)
			{
				int i = 0;
				for(po::bblock_ptr bb: proc->basic_blocks)
					if(bb->area().includes(mne->area))
						return createIndex(i,MnemonicsColumn,tag(bb.get(),BasicBlockTag));
					else
						++i;
			}
		assert(false);
	}

	default:
		assert(false);
	}
}

int Model::rowCount(const QModelIndex &parent) const
{
	po::flowgraph *flow;
	po::procedure *proc;
	po::basic_block *bblock;
	//const mnemonic *mne;
	ptrdiff_t t;
	void *ptr;
	
	std::tie(ptr,t) = extract(parent.internalId());
	
	if(!parent.isValid())
	{
		return m_flowgraphs.size();
	}

	switch(t)
	{
	case FlowgraphTag:
		flow = (po::flowgraph *)ptr;

		if(parent.column() == ProceduresColumn)
			return flow->procedures.size();
		else
			return 0;
	
	case ProcedureTag:
		proc = (po::procedure *)ptr;

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
		bblock = (po::basic_block *)ptr;

		switch(parent.column())
		{		
		case SuccessorsColumn:
			return bblock->outgoing().size();
		case PredecessorsColumn:
			return bblock->incoming().size();
		case MnemonicsColumn:
			return bblock->mnemonics().size();
		default:
			return 0;
		}

	case MnemonicTag:
		//mne = (const mnemonic *)ptr;

		switch(parent.column())
		{
		default:
			return 0;
		}

	default:
		assert(false);
	}
}

int Model::columnCount(const QModelIndex &parent) const
{
	ptrdiff_t t;
	void *ptr;
	
	std::tie(ptr,t) = extract(parent.internalId());
	
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
		switch(parent.column())
		{
		case PredecessorsColumn:
		case SuccessorsColumn:
			return LastBasicBlockColumn;
		case MnemonicsColumn:
			return LastMnemonicColumn;
		default:
			return 0;
		}
	
	case MnemonicTag:
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
		std::tie(ptr,t) = extract(index.internalId());
	
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
	po::flowgraph *flow;
	po::procedure *proc;
	po::basic_block *bblock;
	const po::mnemonic *mne;
	ptrdiff_t t;
	void *ptr;
	
	std::tie(ptr,t) = extract(index.internalId());

	switch(t)
	{
	case FlowgraphTag:
		flow = (po::flowgraph *)ptr;
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
		proc = (po::procedure *)ptr;
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
			return QString("%1 basic blocks").arg(proc->basic_blocks.size());
		case CalleesColumn:
			return QString("%1 callees").arg(proc->callees.size());
		case UniqueIdColumn:
			return QString("%1").arg((ptrdiff_t)proc);
		default:
			assert(false);
		}

	case BasicBlockTag:
		bblock = (po::basic_block *)ptr;
		switch((Column)index.column())
		{
		case AreaColumn:
			return QString("%1:%2").arg(bblock->area().begin).arg(bblock->area().end);
		case MnemonicsColumn:
			return QString("%1 mnemonics").arg(bblock->mnemonics().size());
		case PredecessorsColumn:
			return QString("%1 predecessors").arg(bblock->incoming().size());
		case SuccessorsColumn:
			return QString("%1 successors").arg(bblock->outgoing().size());
		case UniqueIdColumn:
			return QString("%1").arg((ptrdiff_t)bblock);
		default:
			assert(false);
		}

	case MnemonicTag:
		mne = (const po::mnemonic *)ptr;
		switch((Column)index.column())
		{
		case AreaColumn:
			return QString("%1:%2").arg(mne->area.begin).arg(mne->area.end);
		case OpcodeColumn:
			return QString::fromStdString(mne->opcode);
		case OperandsColumn:
			return QString("%1 operands").arg(mne->operands.size());
		case InstructionsColumn:
			return QString("%1 instructions").arg(mne->instructions.size());
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
	po::flowgraph *flow;

	if(!value.size())
		return false;

	std::tie(ptr,t) = extract(index.internalId());
	switch(t)
	{
	case FlowgraphTag:
		assert(index.column() == NameColumn);
		flow = (po::flowgraph *)ptr;

		if(std::any_of(m_flowgraphs.begin(),m_flowgraphs.end(),[&](const po::flow_ptr &f) { return f->name == value; }))
			return false;

		flow->name = value;
		return true;
	
	case ProcedureTag:
	{
		assert(index.column() == NameColumn);
		po::procedure *proc = (po::procedure *)ptr;
		auto i = std::find_if(m_flowgraphs.begin(),m_flowgraphs.end(),[&](const po::flow_ptr &f) 
							{ return std::any_of(f->procedures.begin(),f->procedures.end(),[&](const po::proc_ptr &p) 
								{ return p.get() == proc; }); });

		if(i == m_flowgraphs.end())
			return false;

		if(std::any_of((*i)->procedures.begin(),(*i)->procedures.end(),[&](const po::proc_ptr &p) { return p.get() != proc && p->name == value; }))
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

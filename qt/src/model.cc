#include <QDebug>
#include <sstream>

#include <model.hh>

#include <flowgraph.hh>
#include <procedure.hh>
#include <mnemonic.hh>

Model::Model(po::flow_ptr flow, QObject *parent)
: QAbstractItemModel(parent), m_nextId(0), m_deflate(0)
{
	//po::flow_ptr flow = m_deflate->flowgraph();
	if(flow->name.empty())
		flow->name = "flowgraph #1";
	m_flowgraphs.push_back(flow);
}

Model::~Model(void)
{
	//delete m_deflate;
}

QModelIndex Model::index(int row, int column, const QModelIndex &parent) const
{
	if(!parent.isValid())
	{
		// root
		assert(row >= 0 && (unsigned int)row < m_flowgraphs.size());
		return createIndex(row,column,m_flowgraphs[row].get());
	}

	const Path &e = path(parent.internalId());

	switch(e.type)
	{
	case Path::FlowgraphType:
		if(parent.column() == ProceduresColumn)
		{
			auto i = e.flow->procedures.begin();
			assert(row >= 0 && e.flow->procedures.size() > (unsigned int)row);
			advance(i,row);
			return createIndex(row,column,e.flow,i->get());
		}
		else
			assert(false);

	case Path::ProcedureType:
		switch(parent.column())
		{
		case CalleesColumn:
			assert(row >= 0 && e.proc->callees.size() > (unsigned int)row);
			return createIndex(row,column,e.flow,next(e.proc->callees.begin(),row)->get());
		
		case BasicBlocksColumn:
			assert(row >= 0 && e.proc->basic_blocks.size() > (unsigned int)row);
			return createIndex(row,column,e.flow,e.proc,next(e.proc->basic_blocks.begin(),row)->get());
		
		default:
			assert(false);
		}
	
	case Path::BasicBlockType:
		switch(parent.column())
		{
		case SuccessorsColumn:
			assert(row >= 0 && e.bblock->outgoing().size() > (unsigned int)row);
			return createIndex(row,column,e.flow,e.proc,next(e.bblock->successors().first,row)->get());

		case PredecessorsColumn:
			assert(row >= 0 && e.bblock->incoming().size() > (unsigned int)row);
			return createIndex(row,column,e.flow,e.proc,next(e.bblock->predecessors().first,row)->get());

		case MnemonicsColumn:
			assert(row >= 0 && e.bblock->mnemonics().size() > (unsigned int)row);
			return createIndex(row,column,e.flow,e.proc,e.bblock,&e.bblock->mnemonics()[row]);
			
		default:
			assert(false);
		}
	
	case Path::MnemonicType:
		switch(parent.column())
		{
		case OperandsColumn:
			assert(row >= 0 && e.mne->operands.size() > (unsigned int)row);
			return createIndex(row,column,e.flow,e.proc,e.bblock,e.mne,row);
		default:
			assert(false);
		}

	default:
		assert(false);
	}
}

QModelIndex Model::parent(const QModelIndex &index) const
{
	assert(index.isValid());
	const Path &e = path(index.internalId());
	
	switch(e.type)
	{
	case Path::FlowgraphType:
		return QModelIndex();
	case Path::ProcedureType:
		return createIndex(0,ProceduresColumn,e.flow);
	case Path::BasicBlockType:
		return createIndex(std::distance(e.flow->procedures.begin(),find_if(e.flow->procedures.begin(),e.flow->procedures.end(),[&](const po::proc_ptr p) { return p.get() == e.proc; })),BasicBlocksColumn,e.flow,e.proc);
	case Path::MnemonicType:
		return createIndex(std::distance(e.proc->basic_blocks.begin(),find_if(e.proc->basic_blocks.begin(),e.proc->basic_blocks.end(),[&](const po::bblock_ptr bb) { return bb.get() == e.bblock; })),MnemonicsColumn,e.flow,e.proc,e.bblock);
	case Path::OperandType:
		return createIndex(std::distance(e.bblock->mnemonics().begin(),std::find_if(e.bblock->mnemonics().begin(),e.bblock->mnemonics().end(),[&](const po::mnemonic &m) 
											 { return &m == e.mne; })),OperandsColumn,e.flow,e.proc,e.bblock,e.mne);

	default:
		assert(false);
	}
}

int Model::rowCount(const QModelIndex &parent) const
{
	if(!parent.isValid())
	{
		return m_flowgraphs.size();
	}

	const Path &e = path(parent.internalId());
	
	switch(e.type)
	{
	case Path::FlowgraphType:
		if(parent.column() == ProceduresColumn)
			return e.flow->procedures.size();
		else
			return 0;
	
	case Path::ProcedureType:
		switch(parent.column())
		{
		case CalleesColumn:
			return e.proc->callees.size();
		case BasicBlocksColumn:
			return e.proc->basic_blocks.size();
		default:
			return 0;
		}
	
	case Path::BasicBlockType:
		switch(parent.column())
		{		
		case SuccessorsColumn:
			return distance(e.bblock->successors().first,e.bblock->successors().second);
		case PredecessorsColumn:
			return distance(e.bblock->predecessors().first,e.bblock->predecessors().second);
		case MnemonicsColumn:
			return e.bblock->mnemonics().size();
		default:
			return 0;
		}

	case Path::MnemonicType:
		switch(parent.column())
		{
		case OperandsColumn:
			return e.mne->operands.size();
		default:
			return 0;
		}

	default:
		assert(false);
	}
}

int Model::columnCount(const QModelIndex &parent) const
{
	if(!parent.isValid())
	{
		return LastFlowgraphColumn;
	}
	
	const Path &e = path(parent.internalId());

	switch(e.type)
	{
	case Path::FlowgraphType:
		if(parent.column() == ProceduresColumn)
			return LastProcedureColumn;
		else
			return 0;

	case Path::ProcedureType:
		if(parent.column() == CalleesColumn)
			return LastProcedureColumn;
		else
			return 0;
	
	case Path::BasicBlockType:
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
	
	case Path::MnemonicType:
		switch(parent.column())
		{
		case OperandsColumn:
			return 1;
		default:
			return 0;
		}

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
	Qt::ItemFlags ret = Qt::ItemIsEnabled | Qt::ItemIsSelectable;

	if(!index.isValid())
		return ret;
	
	const Path &e = path(index.internalId());
	
	switch(e.type)
	{
	case Path::FlowgraphType:
	case Path::ProcedureType:
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
	const Path &e = path(index.internalId());

	switch(e.type)
	{
	case Path::FlowgraphType:
		switch((Column)index.column())
		{
		case NameColumn:
			return QString::fromStdString(e.flow->name);
		case ProceduresColumn: 
			return QString("%1 procedures").arg(e.flow->procedures.size());
		default: 
			assert(false);
		}

	case Path::ProcedureType:
		switch((Column)index.column())
		{
		case NameColumn:
			if(e.proc->name.size())
				return QString::fromStdString(e.proc->name);
			else
				return QString("(unnamed)");
		case EntryPointColumn:
			if(e.proc->entry)
				return QString("0x%1").arg(e.proc->entry->area().begin);
			else
				return QString("(no entry)");
		case BasicBlocksColumn: 
			return QString("%1 basic blocks").arg(e.proc->basic_blocks.size());
		case CalleesColumn:
			return QString("%1 callees").arg(e.proc->callees.size());
		case UniqueIdColumn:
			return QString("%1").arg((ptrdiff_t)e.proc);
		default:
			assert(false);
		}

	case Path::BasicBlockType:
		switch((Column)index.column())
		{
		case AreaColumn:
			return QString("%1:%2").arg(e.bblock->area().begin).arg(e.bblock->area().end);
		case MnemonicsColumn:
			return QString("%1 mnemonics").arg(e.bblock->mnemonics().size());
		case PredecessorsColumn:
			return QString("%1 predecessors").arg(e.bblock->incoming().size());
		case SuccessorsColumn:
			return QString("%1 successors").arg(e.bblock->outgoing().size());
		case UniqueIdColumn:
			return QString("%1").arg((ptrdiff_t)e.bblock);
		default:
			assert(false);
		}

	case Path::MnemonicType:
		switch((Column)index.column())
		{
		case AreaColumn:
			return QString("%1:%2").arg(e.mne->area.begin).arg(e.mne->area.end);
		case OpcodeColumn:
			return QString::fromStdString(e.mne->opcode);
		case OperandsColumn:
		{
			QString ret;
			unsigned int idx = 0;
			std::stringstream os;

			for(const po::mnemonic::token &tok: e.mne->format)
			{
				if(tok.alias.empty())
				{
					assert(idx < e.mne->operands.size());
					if(e.mne->operands[idx].is_constant())
					{
						if(tok.has_sign)
							os << (int)e.mne->operands[idx].constant().value();
						else
							os << e.mne->operands[idx].constant().value();
					}
					else
						os << e.mne->operands[idx];
				}
				else
					os << tok.alias;
				idx += tok.group_size;
			}

			return QString::fromStdString(os.str());
		}
		case InstructionsColumn:
			return QString("%1 instructions").arg(e.mne->instructions.size());
		default:
			assert(false);
		}
	
	case Path::OperandType:
		switch((Column)index.column())
		{
		case ValueColumn:
		{
			std::stringstream ss;
			ss << *next(e.mne->operands.begin(),e.op);
			return QString::fromStdString(ss.str());
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
	const Path &e = path(index.internalId());

	if(!value.size())
		return false;

	switch(e.type)
	{
	case Path::FlowgraphType:
		assert(index.column() == NameColumn);

		if(std::any_of(m_flowgraphs.begin(),m_flowgraphs.end(),[&](const po::flow_ptr &f) { return f->name == value; }))
			return false;

		e.flow->name = value;
		return true;
	
	case Path::ProcedureType:
	{
		assert(index.column() == NameColumn);
		if(std::any_of(e.flow->procedures.begin(),e.flow->procedures.end(),[&](const po::proc_ptr &p) { return p.get() != e.proc && p->name == value; }))
			return false;

		e.proc->name = value;
		return true;
	}
	default:
		return false;
	}
}

QModelIndex Model::createIndex(int row, int col, po::flowgraph *flow, po::procedure *proc, po::basic_block *bblock, const po::mnemonic *mne, int op) const
{
	Path *e = new Path();
	uint key = 0;

	e->flow = flow;
	e->proc = proc;
	e->bblock = bblock;
	e->mne = mne;
	e->op = op;
	
	if(!proc)
		e->type = Path::FlowgraphType;
	else if(!bblock)
		e->type = Path::ProcedureType;
	else if(!mne)
		e->type = Path::BasicBlockType;
	else if(op == -1)
		e->type = Path::MnemonicType;
	else
		e->type = Path::OperandType;

	if(!m_pathToId.contains(*e))
	{
		key = m_nextId++;
		m_idToPath.insert(key,e);
		m_pathToId.insert(*e,key);
	}
	else
	{
		key = m_pathToId[*e];
		delete e;
	}

	return QAbstractItemModel::createIndex(row,col,key);
}

const Path &Model::path(uint k) const
{
	assert(m_idToPath.contains(k));
	return *m_idToPath[k];
}

Path::Path(void) : flow(0), proc(0), bblock(0), mne(0), op(-1) {}

bool Path::operator==(const Path &e) const
{
	return e.type == type && e.flow == flow && e.proc == proc && e.bblock == bblock && e.mne == mne && e.op == op;
}

bool Path::operator!=(const Path &e) const
{
	return !(e == *this);
}

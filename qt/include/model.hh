#ifndef MODEL_HH
#define MODEL_HH

#include <deflate.hh>
#include <flowgraph.hh>
#include <procedure.hh>

#include <QAbstractItemModel>
#include <QFont>
#include <QCache>
#include <QDebug>
#include <QHash>

#include <unordered_map>
#include <functional> 
#include <vector>

struct Path
{
	enum Type
	{
		FlowgraphType = 0,
		ProcedureType = 1,
		BasicBlockType = 2,
		MnemonicType = 3,
		ValueType = 4,
		GuardType = 5,
	};

	Path(void);

	bool operator==(const Path &) const;
	bool operator!=(const Path &) const;

	Type type;
	po::flow_ptr flow;
	po::proc_ptr proc;
	po::bblock_ptr bblock;
	const po::mnemonic *mne;
	const po::rvalue *value;
	po::guard_ptr guard;
};

inline uint qHash(const Path &key)
{
	return key.type ^ (uint)key.flow.get() ^ (uint)key.proc.get() ^ (uint)key.bblock.get() ^ (uint)key.mne ^ (uint)key.value ^ (uint)key.guard.get();
}

class Model : public QAbstractItemModel
{
	Q_OBJECT

public:
	Model(po::flow_ptr f, QObject *parent = 0);

	// signaling
	const std::function<void(po::proc_ptr,unsigned int)> &flowgraphSocket(void) const;

	// reading
	virtual QModelIndex index (int row, int column, const QModelIndex &parent = QModelIndex()) const;
	virtual QModelIndex parent(const QModelIndex &index) const;
	virtual int rowCount(const QModelIndex &parent = QModelIndex()) const;
	virtual int columnCount(const QModelIndex &parent = QModelIndex()) const;
	virtual QVariant data(const QModelIndex &index, int role = Qt::DisplayRole) const;

	// writing
	virtual Qt::ItemFlags flags(const QModelIndex &index) const;
	virtual bool setData(const QModelIndex &index, const QVariant &value, int role = Qt::EditRole);

	enum Column
	{
		// FlowgraphType
		ProceduresColumn = 0,			// ProcedureType
		NameColumn = 1,
		LastFlowgraphColumn = 2,

		// ProcedureType
		BasicBlocksColumn = 0,		// BasicBlockType
		//NameColumn = 1,
		EntryPointColumn = 2,			// BasicBlockType
		CalleesColumn = 3,				// ProcedureType
		UniqueIdColumn = 4,
		LastProcedureColumn = 5,

		// BasicBlockType
		MnemonicsColumn = 0,					// MnemonicType,
		AreaColumn = 1,
		PredecessorsColumn = 2,				// BasicBlockType
		PredecessorGuardsColumn = 3,	// GuardType
		//UniqueIdColumn = 4,
		SuccessorsColumn = 5,					// BasicBlockType
		SuccessorGuardsColumn = 6,		// GuardType
		LastBasicBlockColumn = 7,

		// MnemonicType
		OpcodeColumn = 0,
		//AreaColumn = 1,
		OperandsColumn = 2,				// ValueType
		FormatsColumn = 3,				// QString parallel to OperandsColumn
		InstructionsColumn = 4,		// TODO
		LastMnemonicColumn = 5,

		// ValueType
		ValueColumn = 0,
		DecorationColumn = 1, 		// QStringList len == 2
		SscpColumn = 2,
		LastValueColumn = 3,

		// GuardType
		ValuesColumn = 0,					// ValueType
		LastGuardColumn = 1,
	};

private:
	QVariant displayData(const QModelIndex &index) const;
	bool setDisplayData(const QModelIndex &index, const std::string &value);
	QModelIndex createIndex(int row, int col, po::flow_ptr flow, po::proc_ptr proc = 0, po::bblock_ptr bblock = 0, const po::mnemonic *mne = 0, const po::rvalue *val = 0) const;
	QModelIndex createIndex2(int row, int col, po::flow_ptr flow, po::proc_ptr proc = 0, po::bblock_ptr bblock = 0, po::guard_ptr guard = 0) const;
	const Path &path(uint p) const;

	mutable ptrdiff_t m_nextId;
	mutable QHash<uint,const Path *> m_idToPath;
	mutable QHash<const Path,uint> m_pathToId;
	std::vector<po::flow_ptr> m_flowgraphs;
	std::function<void(po::proc_ptr,unsigned int)> m_flowgraphSocket;

private slots:
	void beginInsertProcedure(unsigned int pos);
	void endInsertProcedure(unsigned int pos);
};

#endif

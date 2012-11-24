#ifndef MODEL_HH
#define MODEL_HH

#include <deflate.hh>
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
		OperandType = 4,
	};

	Path(void);

	bool operator==(const Path &) const;
	bool operator!=(const Path &) const;

	Type type;
	po::flowgraph *flow;
	po::procedure *proc;
	po::basic_block *bblock;
	const po::mnemonic *mne;
	unsigned int op;
};

inline uint qHash(const Path &key)
{
	return key.type ^ (uint)key.flow ^ (uint)key.proc ^ (uint)key.bblock ^ (uint)key.mne ^ key.op;
}

class Model : public QAbstractItemModel
{
	Q_OBJECT

public:
	Model(po::deflate *d, QObject *parent = 0);
	~Model(void);

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
		// FlowgraphTag
		//NameColumn = 0,
		ProceduresColumn = 1,
		LastFlowgraphColumn = 2,

		// ProcedureTag
		NameColumn = 0,
		EntryPointColumn = 1,
		BasicBlocksColumn = 2,
		CalleesColumn = 3,
		UniqueIdColumn = 4,
		LastProcedureColumn = 5,

		// BasicBlockTag
		AreaColumn = 0,
		MnemonicsColumn = 1,
		PredecessorsColumn = 2,
		SuccessorsColumn = 3,
		//UniqueIdColumn = 4,
		LastBasicBlockColumn = 5,

		// MnemonicTag
		//AreaColumn = 0,
		OpcodeColumn = 1,
		OperandsColumn = 2,
		InstructionsColumn = 3,
		LastMnemonicColumn = 4,

		// OperandTag
		ValueColumn = 0,
		LastOperandColumn = 1,
	};

private:
	QString displayData(const QModelIndex &index) const;
	bool setDisplayData(const QModelIndex &index, const std::string &value);
	QModelIndex createIndex(int row, int col, po::flowgraph *flow, po::procedure *proc = nullptr, po::basic_block *bblock = nullptr, const po::mnemonic *mne = 0, int op = -1) const;
	const Path &path(uint p) const;

	mutable ptrdiff_t m_nextId;
	mutable QHash<uint,const Path *> m_idToPath;
	mutable QHash<const Path,uint> m_pathToId;
	po::deflate *m_deflate;
	std::vector<po::flow_ptr> m_flowgraphs;
};

#endif

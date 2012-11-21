#ifndef MODEL_HH
#define MODEL_HH

#include <deflate.hh>
#include <QAbstractItemModel>
#include <QFont>

#include <unordered_map>
#include <functional> 
#include <vector>

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
	};

private:	
	enum Tag
	{
		FlowgraphTag = 0,
		ProcedureTag = 1,
		BasicBlockTag = 2,
		MnemonicTag = 3,
		
		LastTag = 4,
		MaskTag = 7,
	};

	QString displayData(const QModelIndex &index) const;
	bool setDisplayData(const QModelIndex &index, const std::string &value);
	
	// pointer tagging
	ptrdiff_t tag(void *ptr, Tag t) const;
	std::pair<void*,Tag> extract(ptrdiff_t p) const;

	po::deflate *m_deflate;
	std::vector<po::flow_ptr> m_flowgraphs;
	std::set<void *> m_selected;
};

#endif

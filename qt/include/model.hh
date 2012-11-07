#ifndef MODEL_HH
#define MODEL_HH

#include <database.hh>
#include <QAbstractItemModel>
#include <QFont>

#include <unordered_map>
#include <functional> 
#include <vector>

class Model : public QAbstractItemModel
{
	Q_OBJECT

public:
	Model(database *db, QObject *parent = 0);
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

		// MnemonicTag
		//AreaColumn = 0,
		OpcodeColumn = 1,
		OperandsColumn = 2,
		InstructionsColumn = 3,
	};

private:	
	enum Tag
	{
		FlowgraphTag = 0,
		ProcedureTag = 1,
		BasicBlockTag = 2,
		
		LastTag = 3,
		MaskTag = 3,
	};

/*	enum Role
	{
		SelectionRole = Qt::UserRole + 1,
	};

	enum Selection
	{
		NoSelection = 0,
		WeakSelection = 1,
		StringSelection = 2,
	};*/

	QString displayData(const QModelIndex &index) const;
//	QFont fontData(const QModelIndex &index) const;

	bool setDisplayData(const QModelIndex &index, const std::string &value);
	
	// pointer tagging
	ptrdiff_t tag(void *ptr, Tag t) const;
	std::pair<void*,Tag> extract(ptrdiff_t p) const;

	database *m_database;
	std::vector<flow_ptr> m_flowgraphs;
	std::set<void *> m_selected;
};

#endif

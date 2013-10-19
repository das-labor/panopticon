#include <QApplication>
#include <QQuickView>
#include <QQuickItem>
#include <QAbstractListModel>
#include <QDebug>
#include <QQmlApplicationEngine>
#include <QQmlComponent>
#include <QQmlContext>
#include <limits>

#include "linearscene.hh"

LinearSceneColumn::LinearSceneColumn(void) : QObject(), m_data(""), m_selected(false) {}
LinearSceneColumn::LinearSceneColumn(const QString &h, bool sel) : QObject(), m_data(h), m_selected(sel) {}
LinearSceneColumn::~LinearSceneColumn(void) {}

QString LinearSceneColumn::data(void) const { return m_data; }
bool LinearSceneColumn::selected(void) const { return m_selected; }

LinearSceneBlock::LinearSceneBlock(void) : QObject(), m_name(""), m_collapsed(false), m_id(-1) {}
LinearSceneBlock::LinearSceneBlock(const QString &h, bool col, int id) : QObject(), m_name(h), m_collapsed(col), m_id(id) {}
LinearSceneBlock::~LinearSceneBlock(void) {}

QString LinearSceneBlock::name(void) const { return m_name; }
bool LinearSceneBlock::collapsed(void) const { return m_collapsed; }
int LinearSceneBlock::id(void) const { return m_id; }

LinearSceneModel::LinearSceneRow::LinearSceneRow(void)
: type(Row), space(), range(), id(-1)
{}

LinearSceneModel::LinearSceneRow::LinearSceneRow(LinearSceneModel::LinearSceneRow::Type t, const po::address_space &as, const po::rrange &r, int i)
: type(t), space(as), range(r), id(i)
{}

LinearSceneModel::LinearSceneRow::LinearSceneRow(const LinearSceneModel::LinearSceneRow &o)
: type(o.type), space(o.space), range(o.range), id(o.id)
{}

bool LinearSceneModel::LinearSceneRow::operator==(const LinearSceneModel::LinearSceneRow &r) const
{
	return type == r.type && space == r.space && range == r.range && id == r.id;
}

LinearSceneModel::LinearSceneRow &LinearSceneModel::LinearSceneRow::operator+=(const LinearSceneModel::LinearSceneRow &r)
{
	return *this;
}

LinearSceneModel::LinearSceneModel(void)
: QAbstractListModel(), m_firstRow(0), m_lastRow(1), m_firstColumn(0), m_lastColumn(3), m_currentView()
{}

LinearSceneModel::~LinearSceneModel(void) {}

int LinearSceneModel::rowCount(const QModelIndex &parent) const
{
	if(!parent.isValid())
		return boost::icl::length(m_currentView);
	else
		return 0;
}

QVariant LinearSceneModel::data(const QModelIndex &index, int role) const
{
	auto iter = m_currentView.find(index.row());

	if(iter == m_currentView.end())
		return QVariant();

	qDebug() << index.row();

	if(iter->second.type == LinearSceneRow::Row)
	{
		switch(role)
		{
			case Qt::DisplayRole: return QString("-- Row --");
			case Qt::UserRole:
			{
				QList<QVariant> lst;
				lst.append(QVariant::fromValue(new LinearSceneColumn("0xaa",selected(index.row(),0))));
				lst.append(QVariant::fromValue(new LinearSceneColumn("0xbb",selected(index.row(),1))));
				lst.append(QVariant::fromValue(new LinearSceneColumn("0xcc",selected(index.row(),2))));
				lst.append(QVariant::fromValue(new LinearSceneColumn("0xdd",selected(index.row(),3))));
				lst.append(QVariant::fromValue(new LinearSceneColumn("0xff",selected(index.row(),4))));
				return QVariant::fromValue(lst);
			}
			case Qt::UserRole + 1: return "qrc:/Element.qml";
			case Qt::UserRole + 3: return QString("%1")/* + %2").arg(QString::fromStdString(sec.second.name))*/.arg(index.row());
			default: return QVariant();
		}
	}
	else
	{
		switch(role)
		{
			case Qt::DisplayRole: return QString("-- Block head --");
			case Qt::UserRole: return QVariant::fromValue(new LinearSceneBlock(
																	QString::fromStdString(iter->second.space.name),
																	iter->second.type == LinearSceneRow::Collapsed,
																	iter->second.id));
			case Qt::UserRole + 1: return "qrc:/Block.qml";
			case Qt::UserRole + 3: return QString("%1")/* + %2").arg(QString::fromStdString(sec.second.name))*/.arg(index.row());
			default: return QVariant();
		}
	}
}

QHash<int, QByteArray> LinearSceneModel::roleNames(void) const
{
	QHash<int, QByteArray> ret;

	ret.insert(Qt::DisplayRole,QByteArray("display"));
	ret.insert(Qt::UserRole+1,QByteArray("delegate"));
	ret.insert(Qt::UserRole+3,QByteArray("offset"));
	ret.insert(Qt::UserRole,QByteArray("payload"));
	return ret;
}

void LinearSceneModel::select(int firstRow, int firstCol, int lastRow, int lastCol)
{
	QModelIndex up = createIndex(std::min(firstRow,m_firstRow < 0 ? std::numeric_limits<int>::max() : m_firstRow),0);
	QModelIndex down = createIndex(std::max(lastRow,m_lastRow < 0 ? 0 : m_lastRow),0);

	m_firstRow = firstRow;
	m_firstColumn = firstCol;
	m_lastRow = lastRow;
	m_lastColumn = lastCol;

	emit dataChanged(up,down);
}

void LinearSceneModel::toggleVisibility(int rowId, bool hide)
{
	int nextRow = 0;
	auto newRows = decltype(m_currentView)();
	int modFirst = 0, modLast = 0;
	int changedRow = 0;

	for(auto p: m_currentView)
	{
		if(p.second.id == rowId)
		{
			if(p.second.type == LinearSceneRow::Block || p.second.type == LinearSceneRow::Collapsed)
			{
				// Block (header)
				newRows.add(std::make_pair(decltype(m_currentView)::interval_type::right_open(nextRow,nextRow + 1),
																	 LinearSceneRow(hide ? LinearSceneRow::Collapsed : LinearSceneRow::Block,p.second.space,p.second.range,p.second.id)));
				changedRow = nextRow;
				nextRow += 1;

				// Show Rows
				if(!hide)
				{
					auto i = m_hidden.find(p.second.id);

					assert(i != m_hidden.end() && i->second.type == LinearSceneRow::Row);

					modFirst = nextRow;
					modLast = nextRow + boost::icl::length(i->second.range) - 1;
					newRows.add(std::make_pair(decltype(m_currentView)::interval_type::right_open(nextRow,nextRow + boost::icl::length(i->second.range)),i->second));
					nextRow += boost::icl::length(i->second.range);
					m_hidden.erase(i);
				}
			}
			else
			{
				assert(p.second.type == LinearSceneRow::Row);

				// Move Row into m_hidden
				if(hide)
				{
					modFirst = nextRow;
					modLast = nextRow + boost::icl::length(p.first) - 1;
					m_hidden.insert(std::make_pair(p.second.id,p.second));
				}
			}
		}
		else
		{
			newRows.add(std::make_pair(decltype(m_currentView)::interval_type::right_open(nextRow,nextRow + boost::icl::length(p.first)),p.second));
			nextRow += boost::icl::length(p.first);
		}
	}

	if(hide)
		beginRemoveRows(QModelIndex(),modFirst,modLast);
	else
		beginInsertRows(QModelIndex(),modFirst,modLast);

	//beginResetModel();
	m_currentView = newRows;
	//endResetModel();

	if(hide)
		endRemoveRows();
	else
		endInsertRows();

	emit dataChanged(index(changedRow),index(changedRow));
}

void LinearSceneModel::collapse(int id)
{
	toggleVisibility(id,true);
}

void LinearSceneModel::expand(int id)
{
	toggleVisibility(id,false);
}

bool LinearSceneModel::selected(int row, int col) const
{
	return (m_firstRow < row && m_lastRow > row) ||
				 (m_firstRow == row && m_lastRow == row && m_firstColumn <= col && m_lastColumn >= col) ||
				 (m_firstRow == row && m_lastRow != row && m_firstColumn <= col) ||
				 (m_lastRow == row && m_firstRow != row && m_lastColumn >= col);
}

void LinearSceneModel::setGraph(const po::graph<po::address_space,po::rrange> &g)
{
	int id = 0;
	int i = 0;
	beginResetModel();
	m_currentView.clear();
	m_hidden.clear();

	for(auto p: po::projection(g.get_node(po::root(g)),g))
	{
		m_currentView.add(std::make_pair(decltype(m_currentView)::interval_type::right_open(i,i + 1),LinearSceneRow(LinearSceneRow::Block,p.second,p.first,id)));
		m_currentView.add(std::make_pair(decltype(m_currentView)::interval_type::right_open(i + 1,i + 1 + boost::icl::length(p.first)),LinearSceneRow(LinearSceneRow::Row,p.second,p.first,id)));
		i += boost::icl::length(p.first) + 1;
		id += 1;
	}
	endResetModel();
}

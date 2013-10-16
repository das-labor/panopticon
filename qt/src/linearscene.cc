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
LinearSceneColumn::LinearSceneColumn(QString h, bool sel) : QObject(), m_data(h), m_selected(sel) {}
LinearSceneColumn::~LinearSceneColumn(void) {}

QString LinearSceneColumn::data(void) const { return m_data; }
bool LinearSceneColumn::selected(void) const { return m_selected; }

LinearSceneModel::LinearSceneRow::LinearSceneRow(void)
: type(Row), space()
{}

LinearSceneModel::LinearSceneRow::LinearSceneRow(LinearSceneModel::LinearSceneRow::Type t, const po::address_space &as)
: type(t), space(as)
{}

LinearSceneModel::LinearSceneRow::LinearSceneRow(const LinearSceneModel::LinearSceneRow &o)
: type(o.type), space(o.space)
{}

bool LinearSceneModel::LinearSceneRow::operator==(const LinearSceneModel::LinearSceneRow &r) const
{
	return type == r.type && space == r.space;
}

LinearSceneModel::LinearSceneRow &LinearSceneModel::LinearSceneRow::operator+=(const LinearSceneModel::LinearSceneRow &r)
{
	return *this;
}

LinearSceneModel::LinearSceneModel(void)
: QAbstractListModel(), m_firstRow(0), m_lastRow(1), m_firstColumn(0), m_lastColumn(3), m_rows()
{}

LinearSceneModel::~LinearSceneModel(void) {}

int LinearSceneModel::rowCount(const QModelIndex &parent) const
{
	if(!parent.isValid())
		return boost::icl::length(m_rows);
	else
		return 0;
}

QVariant LinearSceneModel::data(const QModelIndex &index, int role) const
{
	auto iter = m_rows.find(index.row());

	if(iter == m_rows.end())
		return QVariant();

	switch(role)
	{
		case Qt::DisplayRole: return QString("-- Display --");
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
		case Qt::UserRole + 2: return QString::fromStdString(iter->second.space.name);
		case Qt::UserRole + 3: return QString("%1")/* + %2").arg(QString::fromStdString(sec.second.name))*/.arg(index.row());
		default: return QVariant();
	}
}

QHash<int, QByteArray> LinearSceneModel::roleNames(void) const
{
	QHash<int, QByteArray> ret;

	ret.insert(Qt::DisplayRole,QByteArray("display"));
	ret.insert(Qt::UserRole+1,QByteArray("delegate"));
	ret.insert(Qt::UserRole+2,QByteArray("block"));
	ret.insert(Qt::UserRole+3,QByteArray("offset"));
	ret.insert(Qt::UserRole,QByteArray("row"));
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

bool LinearSceneModel::selected(int row, int col) const
{
	return (m_firstRow < row && m_lastRow > row) ||
				 (m_firstRow == row && m_lastRow == row && m_firstColumn <= col && m_lastColumn >= col) ||
				 (m_firstRow == row && m_lastRow != row && m_firstColumn <= col) ||
				 (m_lastRow == row && m_firstRow != row && m_lastColumn >= col);
}

void LinearSceneModel::setGraph(const po::graph<po::address_space,po::rrange> &g)
{
	int i = 0;
	beginResetModel();
	m_rows.clear();

	for(auto p: po::projection(g.get_node(po::root(g)),g))
	{
		m_rows.add(std::make_pair(decltype(m_rows)::interval_type::right_open(i,i + boost::icl::length(p.first)),LinearSceneRow(LinearSceneRow::Row,p.second)));
		i += boost::icl::length(p.first);
	}
	endResetModel();
}

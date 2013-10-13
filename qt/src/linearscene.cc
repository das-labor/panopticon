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

LinearSceneRow::LinearSceneRow(void) : QObject(), m_data(""), m_selected(false) {}
LinearSceneRow::LinearSceneRow(QString h, bool sel) : QObject(), m_data(h), m_selected(sel) {}
LinearSceneRow::~LinearSceneRow(void) {}

QString LinearSceneRow::data(void) const { return m_data; }
bool LinearSceneRow::selected(void) const { return m_selected; }

LinearSceneModel::LinearSceneModel(void)
: QAbstractListModel(), m_firstRow(0), m_lastRow(1), m_firstColumn(0), m_lastColumn(3)
{}

LinearSceneModel::~LinearSceneModel(void) {}

int LinearSceneModel::rowCount(const QModelIndex &parent) const
{
	if(!parent.isValid())
		return 100;
	else
		return 0;
}

QVariant LinearSceneModel::data(const QModelIndex &index, int role) const
{
	switch(role)
	{
		case Qt::DisplayRole: return QString("-- Display --");
		case Qt::UserRole:
		{
			QList<QVariant> lst;
			lst.append(QVariant::fromValue(new LinearSceneRow("0xaa",selected(index.row(),0))));
			lst.append(QVariant::fromValue(new LinearSceneRow("0xbb",selected(index.row(),1))));
			lst.append(QVariant::fromValue(new LinearSceneRow("0xcc",selected(index.row(),2))));
			lst.append(QVariant::fromValue(new LinearSceneRow("0xdd",selected(index.row(),3))));
			lst.append(QVariant::fromValue(new LinearSceneRow("0xff",selected(index.row(),4))));
			return QVariant::fromValue(lst);
		}
		default: return QVariant();
	}
}

QHash<int, QByteArray> LinearSceneModel::roleNames(void) const
{
	QHash<int, QByteArray> ret;

	ret.insert(Qt::DisplayRole,QByteArray("display"));
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
				 (m_firstRow == row && m_firstColumn <= col) ||
				 (m_lastRow == row && m_lastColumn >= col);
}

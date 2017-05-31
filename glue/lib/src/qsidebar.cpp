#include <QDebug>
#include "qsidebar.h"

QSidebar::QSidebar(QObject* parent) : QAbstractListModel(parent) {}

QSidebar::~QSidebar() {}

int QSidebar::rowCount(const QModelIndex& parent) const {
	return m_items.size();
}

QVariant QSidebar::data(const QModelIndex& idx, int role) const {
	if(idx.column() != 0 || idx.row() >= m_items.size())
		return QVariant();

	switch(role) {
		case Qt::DisplayRole:
		case Qt::UserRole:
			return QVariant(std::get<0>(m_items[idx.row()]));
		case Qt::UserRole + 1:
			return QVariant(std::get<1>(m_items[idx.row()]));
		case Qt::UserRole + 2:
			return QVariant(std::get<2>(m_items[idx.row()]));
		default:
			return QVariant();
	}
}

QHash<int, QByteArray> QSidebar::roleNames(void) const {
	QHash<int, QByteArray> ret;

	ret.insert(Qt::UserRole, QByteArray("title"));
	ret.insert(Qt::UserRole + 1, QByteArray("subtitle"));
	ret.insert(Qt::UserRole + 2, QByteArray("uuid"));

	return ret;
}

void QSidebar::insert(QString title,QString subtitle,QString uuid) {
	auto tpl = std::make_tuple(title,subtitle,uuid);
	size_t idx = 0;

	for(; idx < m_items.size(); ++idx) {
		auto& x = m_items[idx];

		if(std::get<2>(x) == uuid) {
			x = tpl;
			dataChanged(index(idx,0),index(idx,0));
			return;
		}
	}

	beginInsertRows(QModelIndex(), idx, idx);
	m_items.push_back(tpl);
	endInsertRows();
}

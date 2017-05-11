#include <QObject>
#include <QAbstractListModel>
#include <QModelIndex>
#include <QVariant>

#include <tuple>

#pragma once

class QSidebar : public QAbstractListModel {
	Q_OBJECT

public:
	QSidebar(QObject* parent = 0);
	virtual ~QSidebar();

	virtual int rowCount(const QModelIndex& parent = QModelIndex()) const override;
	virtual QVariant data(const QModelIndex& idx, int role = Qt::DisplayRole) const override;
	virtual QHash<int, QByteArray> roleNames(void) const override;

public slots:
	void insert(QString title,QString subtitle,QString uuid);

protected:
	std::vector<std::tuple<QString,QString,QString>> m_items;
};

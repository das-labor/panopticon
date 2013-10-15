#include <QtQml>
#include <QList>

#include <graph.hh>
#include <source.hh>

#pragma once

class LinearSceneRow : public QObject
{
	Q_OBJECT
	Q_PROPERTY(QString data READ data NOTIFY dataChanged)
	Q_PROPERTY(bool selected READ selected NOTIFY selectedChanged)

public:
	LinearSceneRow(void);
	LinearSceneRow(QString h, bool sel);
	virtual ~LinearSceneRow(void);

	QString data(void) const;
	bool selected(void) const;

signals:
	void dataChanged(void);
	void selectedChanged(void);

private:
	QString m_data;
	bool m_selected;
	QString m_delegate;
};

class LinearSceneModel : public QAbstractListModel
{
	Q_OBJECT

public:
	LinearSceneModel(void);
	virtual ~LinearSceneModel(void);

	virtual int rowCount(const QModelIndex &parent = QModelIndex()) const;
	virtual QVariant data(const QModelIndex &index, int role = Qt::DisplayRole) const;
	virtual QHash<int, QByteArray> roleNames(void) const;

public slots:
	void setProjection(const std::list<std::pair<po::rrange,po::address_space>> &proj);
	void select(int firstRow, int firstCol, int lastRow, int lastCol);

private:
	bool selected(int row, int col) const;

	int m_firstRow, m_lastRow, m_firstColumn, m_lastColumn;
	std::list<std::pair<po::rrange,po::address_space>> m_projection;
};

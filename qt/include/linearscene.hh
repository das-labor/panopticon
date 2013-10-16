#include <QtQml>
#include <QList>

#include <graph.hh>
#include <source.hh>

#pragma once

class LinearSceneColumn : public QObject
{
	Q_OBJECT
	Q_PROPERTY(QString data READ data NOTIFY dataChanged)
	Q_PROPERTY(bool selected READ selected NOTIFY selectedChanged)

public:
	LinearSceneColumn(void);
	LinearSceneColumn(QString h, bool sel);
	virtual ~LinearSceneColumn(void);

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
	void setGraph(const po::graph<po::address_space,po::rrange> &g);
	void select(int firstRow, int firstCol, int lastRow, int lastCol);

private:
	bool selected(int row, int col) const;

	int m_firstRow, m_lastRow, m_firstColumn, m_lastColumn;
	po::graph<po::address_space,po::rrange> m_graph;

	struct LinearSceneRow
	{
		enum Type
		{
			Row,
			Folded,
		};

		LinearSceneRow(void);
		LinearSceneRow(Type t, const po::address_space&);
		LinearSceneRow(const LinearSceneRow &r);

		bool operator==(const LinearSceneRow &r) const;
		LinearSceneRow &operator+=(const LinearSceneRow &r);

		Type type;
		po::address_space space;
	};

	boost::icl::split_interval_map<int,LinearSceneRow> m_rows;
};

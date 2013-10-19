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
	LinearSceneColumn(const QString &h, bool sel);
	virtual ~LinearSceneColumn(void);

	QString data(void) const;
	bool selected(void) const;

signals:
	void dataChanged(void);
	void selectedChanged(void);

private:
	QString m_data;
	bool m_selected;
};

class LinearSceneBlock : public QObject
{
	Q_OBJECT
	Q_PROPERTY(QString name READ name NOTIFY nameChanged)
	Q_PROPERTY(bool collapsed READ collapsed NOTIFY collapsedChanged)
	Q_PROPERTY(int id READ id NOTIFY idChanged)

public:
	LinearSceneBlock(void);
	LinearSceneBlock(const QString &n, bool col, int id);
	virtual ~LinearSceneBlock(void);

	QString name(void) const;
	bool collapsed(void) const;
	int id(void) const;

signals:
	void nameChanged(void);
	void collapsedChanged(void);
	void idChanged(void);

private:
	QString m_name;
	bool m_collapsed;
	int m_id;
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
	void collapse(int id);
	void expand(int id);

private:
	bool selected(int row, int col) const;
	void toggleVisibility(int rowId, bool hide);

	int m_firstRow, m_lastRow, m_firstColumn, m_lastColumn;
	po::graph<po::address_space,po::rrange> m_graph;

	struct LinearSceneRow
	{
		enum Type
		{
			Row,
			Block,
			Collapsed,
		};

		LinearSceneRow(void);
		LinearSceneRow(Type t, const po::address_space&, const po::rrange&, int id);
		LinearSceneRow(const LinearSceneRow &r);

		bool operator==(const LinearSceneRow &r) const;
		LinearSceneRow &operator+=(const LinearSceneRow &r);

		Type type;
		po::address_space space;
		po::rrange range; ///< Key when in m_currentView
		int id;	///< Key when in m_hidden
	};

	boost::icl::split_interval_map<int,LinearSceneRow> m_currentView;
	std::unordered_map<int,LinearSceneRow> m_hidden;
};

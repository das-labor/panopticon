#include <QtQml>
#include <QList>

#include <graph.hh>
#include <source.hh>

#include <elementselection.hh>
#include <delegate.hh>

#pragma once

class Header : public QObject
{
	Q_OBJECT
	Q_PROPERTY(QString name READ name NOTIFY nameChanged)
	Q_PROPERTY(bool collapsed READ collapsed NOTIFY collapsedChanged)
	Q_PROPERTY(int id READ id NOTIFY idChanged)

public:
	Header(void);
	Header(const QString &n, bool col, int id);
	virtual ~Header(void);

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
	// From Window/Tree sidebar
	void setGraph(const po::graph<po::address_space,po::rrange> &g);

	// From Hex.qml/Element.qml
	void setSelect(int firstRow, int firstCol, int lastRow, int lastCol);
	void setVisibility(int blkid, bool vis);

	void delegateModified(const boost::optional<ElementSelection> &);

signals:
	void addAddressSpace(const po::rrange &);

private:
	boost::optional<ElementSelection> m_cursor;
	po::graph<po::address_space,po::rrange> m_graph;

	struct LinearSceneBlock
	{
		enum Type
		{
			Data,
			Header,
			HeaderCollapsed,
		};

		LinearSceneBlock(void);
		LinearSceneBlock(Type t, QSharedPointer<Delegate> d, int id);
		LinearSceneBlock(const LinearSceneBlock &r);

		bool operator==(const LinearSceneBlock &r) const;
		LinearSceneBlock &operator+=(const LinearSceneBlock &r);

		Type type;
		QSharedPointer<Delegate> delegate;
		int id;	///< Key when in m_hidden
	};

	boost::icl::split_interval_map<int,LinearSceneBlock> m_currentView;
	std::unordered_map<int,LinearSceneBlock> m_hidden;
};

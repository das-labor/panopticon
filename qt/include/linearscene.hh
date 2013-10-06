#include <QDeclarativeListProperty>
#include <QDeclarativeItem>
#include <QList>

#include <graph.hh>
#include <source.hh>

#pragma once

class Section : public QDeclarativeItem
{
	Q_OBJECT
	Q_PROPERTY(QString name READ name NOTIFY nameChanged)
	Q_PROPERTY(int rows READ rows NOTIFY rowsChanged)

public:
	Section(void) : m_name("(noname)"), m_rows(0) {}
	Section(const QString &n, int r) : m_name(n), m_rows(r) {}

	QString name(void) const { return m_name; }
	int rows(void) const { return m_rows; }

signals:
	void nameChanged(void);
	void rowsChanged(void);

private:
	QString m_name;
	int m_rows;
};

class LinearScene : public QDeclarativeItem
{
	Q_OBJECT
	Q_PROPERTY(QDeclarativeListProperty<Section> nodes READ nodes NOTIFY nodesChanged)

public:
	LinearScene(QDeclarativeItem *parent = 0);
	virtual ~LinearScene(void);

	QDeclarativeListProperty<Section> nodes(void);
	const QList<Section*> &nodeList(void) const;

public slots:
	void graphChanged(const po::graph<po::address_space,po::rrange> &graph);

signals:
	void nodesChanged(void);

private:
	QList<Section*> m_nodes;

	template<typename T>
	static void appendCallback(QDeclarativeListProperty<T> *property, T *value);
	template<typename T>
	static int countCallback(QDeclarativeListProperty<T> *property);
	template<typename T>
	static T *atCallback(QDeclarativeListProperty<T> *property, int idx);
	template<typename T>
	static void clearCallback(QDeclarativeListProperty<T> *property);
};

template<>
void LinearScene::appendCallback(QDeclarativeListProperty<Section> *property, Section *value);
template<>
int LinearScene::countCallback(QDeclarativeListProperty<Section> *property);
template<>
Section *LinearScene::atCallback(QDeclarativeListProperty<Section> *property, int idx);
template<>
void LinearScene::clearCallback(QDeclarativeListProperty<Section> *property);

#ifndef GRAPH_HH
#define GRAPH_HH

#include <QtDeclarative>
#include <QPainter>
#include <QList>
#include <QDebug>
#include <QStateMachine>
#include <QEventTransition>

#include "path.hh"
#include "stateMachine.hh"

class Scene : public QDeclarativeItem
{
	Q_OBJECT
	Q_PROPERTY(QDeclarativeListProperty<QDeclarativeItem> nodes READ nodes NOTIFY nodesChanged)
	Q_PROPERTY(QDeclarativeListProperty<Path> paths READ paths NOTIFY pathsChanged)

public:
	Scene(QDeclarativeItem *parent = 0);
	virtual ~Scene(void);

	QDeclarativeListProperty<QDeclarativeItem> nodes(void);
	QDeclarativeListProperty<Path> paths(void);

	const QList<QDeclarativeItem*> &nodeList(void) const;
	const QList<Path*> &pathList(void) const;
	const QList<Path*> &outEdges(QDeclarativeItem *i) const;

signals:
	void nodesChanged(void);
	void pathsChanged(void);

protected:
	virtual void mousePressEvent(QGraphicsSceneMouseEvent *event);
	virtual void mouseReleaseEvent(QGraphicsSceneMouseEvent *event);
	virtual void mouseMoveEvent(QGraphicsSceneMouseEvent *event);
  virtual bool eventFilter(QObject *obj, QEvent *event);

private:
	QList<QDeclarativeItem*> m_nodes;
	QList<Path*> m_paths;
	QMap<QDeclarativeItem*,QList<Path*>> m_outEdges;
	QMap<QDeclarativeItem*,NodeState*> m_nodeStates;
	QMap<Path*,PathState*> m_pathStates;
	StateMachine m_stateMachine;
	QState *m_rootState;
	QSet<QDeclarativeItem*> m_grabbedNodes;

	void layoutNodes(void);
	void routePaths(void);

	template<typename T>
	static void appendCallback(QDeclarativeListProperty<T> *property, T *value);
	template<typename T>
	static int countCallback(QDeclarativeListProperty<T> *property);
	template<typename T>
	static T *atCallback(QDeclarativeListProperty<T> *property, int idx);
	template<typename T>
	static void clearCallback(QDeclarativeListProperty<T> *property);

private slots:
	void sent(const Event &n);
	void updateOutEdges(void);
};

template<>
void Scene::appendCallback(QDeclarativeListProperty<QDeclarativeItem> *property, QDeclarativeItem *value);
template<>
void Scene::appendCallback(QDeclarativeListProperty<Path> *property, Path *value);
template<>
int Scene::countCallback(QDeclarativeListProperty<QDeclarativeItem> *property);
template<>
int Scene::countCallback(QDeclarativeListProperty<Path> *property);
template<>
QDeclarativeItem *Scene::atCallback(QDeclarativeListProperty<QDeclarativeItem> *property, int idx);
template<>
Path *Scene::atCallback(QDeclarativeListProperty<Path> *property, int idx);
template<>
void Scene::clearCallback(QDeclarativeListProperty<QDeclarativeItem> *property);
template<>
void Scene::clearCallback(QDeclarativeListProperty<Path> *property);

#endif

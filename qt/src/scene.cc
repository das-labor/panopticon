#include "graph.hh"
#include "interface.hh"

Scene::Scene(QDeclarativeItem *parent)
: QDeclarativeItem(parent), m_nodes(), m_paths(), m_stateMachine(), m_rootState(new QState()), m_grabbedNodes()
{
	new GraphState(&m_stateMachine,m_rootState);

	m_rootState->setChildMode(QState::ParallelStates);

	m_stateMachine.addState(m_rootState);
	m_stateMachine.setInitialState(m_rootState);
	m_stateMachine.start();

	setAcceptedMouseButtons(Qt::LeftButton);
	connect(&m_stateMachine,SIGNAL(sent(const Event &)),this,SLOT(sent(const Event &)));
}

Scene::~Scene(void)
{}

void Scene::mousePressEvent(QGraphicsSceneMouseEvent *event)
{
	QList<QGraphicsItem*> items = scene()->items(event->scenePos());
	QListIterator<QGraphicsItem*> iter(items);

	while(iter.hasNext())
	{
		QDeclarativeItem *item = dynamic_cast<QDeclarativeItem*>(iter.next());
		if(m_nodes.contains(item))
			m_stateMachine.send("press.node",QPair<QString,QVariant>("node",QVariant::fromValue(item)));
	}
	event->accept();
}

void Scene::mouseReleaseEvent(QGraphicsSceneMouseEvent *event)
{
	QList<QGraphicsItem*> items = scene()->items(event->scenePos());
	QListIterator<QGraphicsItem*> iter(items);

	while(iter.hasNext())
	{
		QDeclarativeItem *item = dynamic_cast<QDeclarativeItem*>(iter.next());
		if(m_nodes.contains(item))
			m_stateMachine.send("release.node",QPair<QString,QVariant>("node",QVariant::fromValue(item)));
	}
	event->accept();
}

void Scene::mouseMoveEvent(QGraphicsSceneMouseEvent *event)
{
	QPointF delta = event->scenePos() - event->lastScenePos();
	QSetIterator<QDeclarativeItem*> iter(m_grabbedNodes);

	while(iter.hasNext())
	{
		QDeclarativeItem *i = iter.next();
		i->setPos(i->pos() + delta);
	}
}

bool Scene::eventFilter(QObject *obj, QEvent *event)
{
	switch(event->type())
	{
		case QEvent::GraphicsSceneMousePress:
			mousePressEvent(dynamic_cast<QGraphicsSceneMouseEvent*>(event));
			return true;
		case QEvent::GraphicsSceneMouseRelease:
			mouseReleaseEvent(dynamic_cast<QGraphicsSceneMouseEvent*>(event));
			return true;
		case QEvent::GraphicsSceneMouseMove:
			mouseMoveEvent(dynamic_cast<QGraphicsSceneMouseEvent*>(event));
			return true;
		default:
			return false;
	}
}

void Scene::sent(const Event &n)
{
	//qDebug() << "sent" << n.name();
	m_stateMachine.postEvent(new Event(n));

	if(n.name() == "start.layout")
	{
		layoutNodes();
	}
	else if(n.name() == "start.route")
	{
		routePaths();
	}
	else if(n.name().endsWith(".node"))
	{
		assert(n.has("node"));
		QDeclarativeItem *node = n["node"].value<QDeclarativeItem*>();
		assert(m_nodes.contains(node));

		if(n.name() == "transition.node")
		{
			assert(n.has("state"));
			QString state = n["state"].value<QString>();

			node->setProperty("state",state);
		}
		else if(n.name() == "grab.node")
		{
			m_grabbedNodes.insert(node);
		}
		else if(n.name() == "drop.node")
		{
			m_grabbedNodes.remove(node);
		}
	}
	else if(n.name().endsWith(".path"))
	{
		assert(n.has("path"));
		Path *path = reinterpret_cast<Path*>(n["path"].value<void*>());
		assert(m_paths.contains(path));

		if(n.name() == "transition.path")
		{
			assert(n.has("state"));
			QString state = n["state"].value<QString>();

			path->setProperty("state",state);
		}
	}
}

QDeclarativeListProperty<QDeclarativeItem> Scene::nodes(void)
{
	return QDeclarativeListProperty<QDeclarativeItem>(this,this,&appendCallback<QDeclarativeItem>,&countCallback<QDeclarativeItem>,&atCallback<QDeclarativeItem>,&clearCallback<QDeclarativeItem>);
}

QDeclarativeListProperty<Path> Scene::paths(void)
{
	return QDeclarativeListProperty<Path>(this,this,&appendCallback<Path>,&countCallback<Path>,&atCallback<Path>,&clearCallback<Path>);
}

const QList<QDeclarativeItem*> &Scene::nodeList(void) const
{
	return m_nodes;
}

const QList<Path*> &Scene::pathList(void) const
{
	return m_paths;
}

const QList<Path*> &Scene::outEdges(QDeclarativeItem *i) const
{
	QMap<QDeclarativeItem*,QList<Path*>>::const_iterator j = m_outEdges.constFind(i);

	assert(j != m_outEdges.constEnd());
	return *j;
}

void Scene::updateOutEdges(void)
{
	QMutableMapIterator<QDeclarativeItem*,QList<Path*>> iter(m_outEdges);
	Path *path = qobject_cast<Path*>(sender());

	assert(path);
	while(iter.hasNext())
	{
		iter.next();
		iter.value().removeAll(path);
	}

	if(path->from())
		m_outEdges[path->from()].append(path);
}

void Scene::routePaths(void)
{
	GraphInterface iface(scene(),this);
	dot::astar<GraphInterface>(iface);

	m_stateMachine.postEvent(new Event("done.route"));
}

void Scene::layoutNodes(void)
{
	GraphInterface iface(scene(),this);
	dot::layout<GraphInterface>(iface,100,100);

	m_stateMachine.postEvent(new Event("done.layout"));
}

template<>
void Scene::appendCallback(QDeclarativeListProperty<QDeclarativeItem> *property, QDeclarativeItem *value)
{
	Scene *graph = reinterpret_cast<Scene*>(property->data);

	if(graph)
	{
		assert(!graph->m_nodes.contains(value));

		graph->m_nodes.append(value);
		value->installEventFilter(graph);
		value->setAcceptedMouseButtons(Qt::LeftButton);
		graph->m_nodeStates.insert(value,new NodeState(QVariant::fromValue(value),&graph->m_stateMachine,graph->m_rootState));
		if(!graph->m_outEdges.contains(value))
			graph->m_outEdges.insert(value,QList<Path*>());

		graph->nodesChanged();
	}
}

template<>
void Scene::appendCallback(QDeclarativeListProperty<Path> *property, Path *value)
{
	Scene *graph = reinterpret_cast<Scene*>(property->data);

	if(graph)
	{
		assert(!graph->m_paths.contains(value));

		graph->m_paths.append(value);
		graph->m_pathStates.insert(value,new PathState(QVariant::fromValue<void*>(value),&graph->m_stateMachine,graph->m_rootState));

		graph->connect(value,SIGNAL(nodesChanged()),graph,SLOT(updateOutEdges()));
		if(value->from())
			graph->m_outEdges[value->from()].append(value);

		graph->pathsChanged();
	}
}

template<>
int Scene::countCallback(QDeclarativeListProperty<QDeclarativeItem> *property)
{
	Scene *graph = reinterpret_cast<Scene*>(property->data);
	return graph ? graph->nodeList().count() : -1;
}

template<>
int Scene::countCallback(QDeclarativeListProperty<Path> *property)
{
	Scene *graph = reinterpret_cast<Scene*>(property->data);
	return graph ? graph->pathList().count() : -1;
}

template<>
QDeclarativeItem *Scene::atCallback(QDeclarativeListProperty<QDeclarativeItem> *property, int idx)
{
	Scene *graph = reinterpret_cast<Scene*>(property->data);
	return graph ? graph->nodeList().value(idx) : 0;
}

template<>
Path *Scene::atCallback(QDeclarativeListProperty<Path> *property, int idx)
{
	Scene *graph = reinterpret_cast<Scene*>(property->data);
	return graph ? graph->pathList().value(idx) : 0;
}

template<>
void Scene::clearCallback(QDeclarativeListProperty<QDeclarativeItem> *property)
{
	Scene *graph = reinterpret_cast<Scene*>(property->data);
	if(graph)
	{
		graph->m_nodes.clear();

		QMapIterator<QDeclarativeItem*,NodeState*> iter(graph->m_nodeStates);
		while(iter.hasNext())
		{
			NodeState *st = *iter.next();
			QDeclarativeItem *itm = iter.key();
			itm->removeEventFilter(graph);
			delete st;
		}

		graph->m_outEdges.clear();
		graph->m_nodeStates.clear();
		graph->m_grabbedNodes.clear();
		graph->nodesChanged();
	}
}

template<>
void Scene::clearCallback(QDeclarativeListProperty<Path> *property)
{
	Scene *graph = reinterpret_cast<Scene*>(property->data);
	if(graph)
	{
		graph->m_paths.clear();

		QMapIterator<Path*,PathState*> iter(graph->m_pathStates);
		while(iter.hasNext())
		{
			delete iter.next();
			graph->disconnect(iter.key());
		}

		graph->m_pathStates.clear();
		graph->m_outEdges.clear();
		graph->pathsChanged();
	}
}

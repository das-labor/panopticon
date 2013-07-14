#include <cassert>

#include <QDebug>

#include "stateMachine.hh"

const QEvent::Type Event::eventType = static_cast<QEvent::Type>(QEvent::registerEventType());

Event::Event(QString name, std::initializer_list<QPair<QString,QVariant>> &&lst)
: QEvent(Event::eventType), m_name(name), m_arguments()
{
	for(const QPair<QString,QVariant> &p: lst)
		m_arguments.insert(p.first,p.second);
}

Event::Event(const Event &ev)
: QEvent(Event::eventType), m_name(ev.name()), m_arguments(ev.m_arguments)
{}

Event::~Event(void)
{}

bool Event::has(const QString &a) const
{
	return m_arguments.contains(a);
}

const QVariant &Event::operator[](const QString &n) const
{
	assert(has(n));
	return *m_arguments.constFind(n);
}

const QString &Event::name(void) const
{
	return m_name;
}

Transition::Transition(QState *src)
: QAbstractTransition(src), m_events(), m_onTransition()
{}

Transition::~Transition(void)
{}

void Transition::addEvent(const QString &n,const std::function<bool(const Event&)> &f)
{
	m_events.insert(n,f);
}

void Transition::setOnTransition(const std::function<void(const Event&)> &f)
{
	m_onTransition = f;
}

const QMap<QString,std::function<bool(const Event&)>> &Transition::events(void) const
{
	return m_events;
}

bool Transition::eventTest(QEvent *ev)
{
	Event *e;
	return m_events.empty() ||
		(ev && ev->type() == Event::eventType &&
		 (e = dynamic_cast<Event*>(ev)) &&
		 m_events.contains(e->name()) &&
		 (!m_events.value(e->name()) || m_events.value(e->name())(*e)));
}

void Transition::onTransition(QEvent *ev)
{
	//qDebug() << "move from" << dynamic_cast<State*>(sourceState())->name() << "to" << dynamic_cast<State*>(targetState())->name();
	if(m_onTransition)
	{
		Event *e = dynamic_cast<Event*>(ev);

		if(e)
			m_onTransition(*e);
		else
			m_onTransition(Event(""));
	}
}

State::State(const QString &n, State *parent)
: QState(parent), m_name(n), m_machine(parent->machine()), m_onEntry(), m_onExit()
{}

State::State(const QString &n, StateMachine *machine, QState *parent)
: QState(parent), m_name(n), m_machine(machine), m_onEntry(), m_onExit()
{}

State::~State(void)
{}

void State::setOnEntry(const std::function<void(void)> &f)
{
	m_onEntry = f;
}

void State::setOnExit(const std::function<void(void)> &f)
{
	m_onExit = f;
}

void State::onEntry(QEvent *ev)
{
	//qDebug() << "enter" << name();
	if(m_onEntry)
		m_onEntry();
}

void State::onExit(QEvent *ev)
{
	//qDebug() << "exit" << name();
	if(m_onExit)
		m_onExit();
}

StateMachine *State::machine(void) const
{
	return m_machine;
}

const QString &State::name(void) const
{
	return m_name;
}

StateMachine::StateMachine(QObject *parent)
: QStateMachine(parent)
{}

StateMachine::~StateMachine(void)
{}

SceneState::SceneState(StateMachine *m, QState *parent)
: State("graph",m,parent)
{
	// States
	State *graphState = this;
	graphState->setOnEntry([this](void)
	{
		machine()->send("hide.spinner");
	});
	State *initialState = new State("initial",graphState);
	initialState->setOnEntry([this](void)
	{
		machine()->send("show.spinner");
	});
	State *dirtyNodesState = new State("dirtyNodes",graphState);
	dirtyNodesState->setOnExit([this](void)
	{
		machine()->send("cancel.layout");
	});
	State *visibleNodesState = new State("visibleNodes",graphState);
	State *dirtyPathsState = new State("dirtyPaths",visibleNodesState);
	dirtyPathsState->setOnEntry([this](void)
	{
		machine()->send("start.route");
	});
	dirtyPathsState->setOnExit([this](void)
	{
		machine()->send("cancel.route");
	});
	State *completeState = new State("complete",visibleNodesState);
	visibleNodesState->setInitialState(dirtyPathsState);
	State *dragState = new State("drag",graphState);
	graphState->setInitialState(initialState);

	// Transitions
	Transition *initialTransition1 = new Transition(initialState);
	initialTransition1->setTargetState(dirtyNodesState);
	initialTransition1->setOnTransition([this](const Event &event)
	{
		machine()->send("start.layout");
	});
	Transition *dirtyNodesTransition1 = new Transition(dirtyNodesState);
	dirtyNodesTransition1->setTargetState(dirtyPathsState);
	dirtyNodesTransition1->addEvent("done.layout");
	dirtyNodesTransition1->addEvent("hide.spinner");
	Transition *dirtyPathsTransition1 = new Transition(dirtyPathsState);
	dirtyPathsTransition1->setTargetState(completeState);
	dirtyPathsTransition1->addEvent("done.route");
	Transition *visibleNodesTransition1 = new Transition(visibleNodesState);
	visibleNodesTransition1->setTargetState(dragState);
	visibleNodesTransition1->addEvent("grab.node");
	Transition *dragTransition1 = new Transition(dragState);
	dragTransition1->setTargetState(dirtyPathsState);
	dragTransition1->addEvent("drop.node");
	Transition *graphTransition1 = new Transition(graphState);
	graphTransition1->setTargetState(initialState);
	graphTransition1->addEvent("modify.nodes");
}

NodeState::NodeState(const QVariant &node, StateMachine *m, QState *parent)
: State("node",m,parent), m_node(node)
{
	// States
	State *nodeState = this;
	State *nodeShownState = new State("nodeShown",nodeState);
	State *nodeGrabbedState = new State("nodeGrabbed",nodeShownState);
	nodeGrabbedState->setOnEntry([this](void)
	{
		machine()->send("transition.node", QPair<QString,QVariant>("node",QVariant(m_node)), QPair<QString,QVariant>("state",QVariant("grabbed")));
	});
	State *nodeDroppedState = new State("nodeDropped",nodeShownState);
	nodeDroppedState->setOnEntry([this](void)
	{
		machine()->send("transition.node", QPair<QString,QVariant>("node",QVariant(m_node)), QPair<QString,QVariant>("state",QVariant("")));
	});
	nodeShownState->setInitialState(nodeDroppedState);
	State *nodeHiddenState = new State("nodeHidden",nodeState);
	nodeHiddenState->setOnEntry([this](void)
	{
		machine()->send("transition.node", QPair<QString,QVariant>("node",QVariant(m_node)), QPair<QString,QVariant>("state",QVariant("hidden")));
	});
	nodeState->setInitialState(nodeHiddenState);

	// Transitions
	Transition *nodeGrabbedTransition1 = new Transition(nodeGrabbedState);
	nodeGrabbedTransition1->setTargetState(nodeDroppedState);
	nodeGrabbedTransition1->addEvent("release.node",[this](const Event &event)
	{
		return event["node"] == m_node;
	});
	nodeGrabbedTransition1->setOnTransition([this](const Event &event)
	{
		machine()->send("drop.node", QPair<QString,QVariant>("node",QVariant(event["node"])));
	});
	Transition *nodeDroppedTransition1 = new Transition(nodeDroppedState);
	nodeDroppedTransition1->setTargetState(nodeGrabbedState);
	nodeDroppedTransition1->addEvent("press.node",[this](const Event &event)
	{
		return event["node"] == m_node;
	});
	nodeDroppedTransition1->setOnTransition([this](const Event &event)
	{
		machine()->send("grab.node", QPair<QString,QVariant>("node",QVariant(event["node"])));
	});
	Transition *nodeShownTransition1 = new Transition(nodeShownState);
	nodeShownTransition1->setTargetState(nodeHiddenState);
	nodeShownTransition1->addEvent("start.layout");
	Transition *nodeHiddenTransition1 = new Transition(nodeHiddenState);
	nodeHiddenTransition1->setTargetState(nodeShownState);
	nodeHiddenTransition1->addEvent("done.layout");
}

PathState::PathState(const QVariant &path, StateMachine *m, QState *parent)
: State("path",m,parent), m_path(path)
{
	// States
	State *pathState = this;
	State *pathSimpleState = new State("pathSimple",pathState);
	pathSimpleState->setOnEntry([this](void)
	{
		machine()->send("transition.path", QPair<QString,QVariant>("path",QVariant(m_path)), QPair<QString,QVariant>("state",QVariant("simple")));
	});
	State *pathDetailedState = new State("pathDetailed",pathState);
	pathDetailedState->setOnEntry([this](void)
	{
		machine()->send("transition.path", QPair<QString,QVariant>("path",QVariant(m_path)), QPair<QString,QVariant>("state",QVariant("")));
	});
	State *pathHiddenState = new State("pathHidden",pathState);
	pathHiddenState->setOnEntry([this](void)
	{
		machine()->send("transition.path", QPair<QString,QVariant>("path",QVariant(m_path)), QPair<QString,QVariant>("state",QVariant("hidden")));
	});
	pathState->setInitialState(pathHiddenState);

	// Transitions
	Transition *pathSimpleTransition1 = new Transition(pathSimpleState);
	pathSimpleTransition1->setTargetState(pathHiddenState);
	pathSimpleTransition1->addEvent("start.route");
	Transition *pathDetailedTransition1 = new Transition(pathDetailedState);
	pathDetailedTransition1->setTargetState(pathHiddenState);
	pathDetailedTransition1->addEvent("start.route");
	Transition *pathDetailedTransition2 = new Transition(pathDetailedState);
	pathDetailedTransition2->setTargetState(pathSimpleState);
	pathDetailedTransition2->addEvent("grab.node");
	Transition *pathHiddenTransition1 = new Transition(pathHiddenState);
	pathHiddenTransition1->setTargetState(pathDetailedState);
	pathHiddenTransition1->addEvent("done.route");
}


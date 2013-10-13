#ifndef STATEMACHINE_HH
#define STATEMACHINE_HH

#include <initializer_list>

#include <QStateMachine>
#include <QEvent>
#include <QAbstractTransition>

class Event;
class Transition;
class StateMachine;

class Event : public QEvent
{
public:
	const static QEvent::Type eventType;

	Event(const Event &ev);
	Event(QString name, std::initializer_list<QPair<QString,QVariant>> &&vars = std::initializer_list<QPair<QString,QVariant>>());
	virtual ~Event(void);

	const QString &name(void) const;
	bool has(const QString &arg) const;
	const QVariant &operator[](const QString &a) const;

private:
	QString m_name;
	QMap<QString,QVariant> m_arguments;
};

class Transition : public QAbstractTransition
{
	Q_OBJECT

public:
	Transition(QState *src = 0);
	virtual ~Transition(void);

	void addEvent(const QString &n,const std::function<bool(const Event&)> &f = std::function<bool(const Event&)>());
	const QMap<QString,std::function<bool(const Event&)>> &events(void) const;
	void setOnTransition(const std::function<void(const Event&)> &f);

protected:
	virtual bool eventTest(QEvent *ev);
	virtual void onTransition(QEvent *ev);

private:
	QMap<QString,std::function<bool(const Event&)>> m_events;
	std::function<void(const Event&)> m_onTransition;
};

class State : public QState
{
	Q_OBJECT

public:
	State(const QString &n, State *parent = 0);
	State(const QString &n, StateMachine *machine, QState *parent = 0);
	virtual ~State(void);

	void setOnEntry(const std::function<void(void)> &f);
	void setOnExit(const std::function<void(void)> &f);
	StateMachine *machine(void) const;
	const QString &name(void) const;

protected:
	virtual void onEntry(QEvent *ev);
	virtual void onExit(QEvent *ev);

private:
	QString m_name;
	StateMachine *m_machine;
	std::function<void(void)> m_onEntry;
	std::function<void(void)> m_onExit;
};

class StateMachine : public QStateMachine
{
	Q_OBJECT

public:
	StateMachine(QObject *parent = 0);
	virtual ~StateMachine(void);

	template<typename... Args>
	void send(const QString &n, Args&&... args)
	{
		emit sent(Event(n,{args...}));
	}

signals:
	void sent(const Event &ev);
};

class GraphState : public State
{
	Q_OBJECT

public:
	GraphState(StateMachine *machine, QState *parent = 0);
};

class NodeState : public State
{
	Q_OBJECT

public:
	NodeState(const QVariant &node, StateMachine *machine, QState *parent = 0);

private:
	QVariant m_node;
};

class PathState : public State
{
	Q_OBJECT

public:
	PathState(const QVariant &path, StateMachine *machine, QState *parent = 0);

private:
	QVariant m_path;
};

#endif

#include <QtQuick>
#include <QtCore>
#include <QApplication>
#include <QQmlApplicationEngine>

#include "session.hh"
#include "pen.hh"
#include "sugiyama.hh"
#include "session.hh"

#pragma once

class Panopticon : public QObject
{
	Q_OBJECT
	Q_PROPERTY(QString buildDate READ buildDate NOTIFY buildDateChanged)
	Q_PROPERTY(Session* session READ session NOTIFY sessionChanged)

public:
	static QObject* provider(QQmlEngine*, QJSEngine*);
	static Panopticon& instance(void);

	Panopticon(QObject* p = 0);
	virtual ~Panopticon(void);

	QString buildDate(void) const;
	Session* session(void) const;

	Q_INVOKABLE Session* openSession(const QString&);
	Q_INVOKABLE Session* createSession(const QString&);

signals:
	void buildDateChanged(void);
	void sessionChanged(void);

private:
	Session* _session;

	static Panopticon* _instance;
};

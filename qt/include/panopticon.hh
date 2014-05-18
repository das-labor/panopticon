#include <QtQuick>
#include <QtCore>
#include <QApplication>
#include <QQmlApplicationEngine>

#include "linearview.hh"
#include "session.hh"
#include "pen.hh"
#include "selection.hh"
#include "sugiyama.hh"
#include "session.hh"

#pragma once

class Panopticon : public QApplication
{
	Q_OBJECT
	Q_PROPERTY(QString buildDate READ buildDate NOTIFY buildDateChanged)

public:
	static QObject* provider(QQmlEngine*, QJSEngine*);

	Panopticon(int& argc, char *argv[], const std::string& root = "qrc:/Window.qml");
	virtual ~Panopticon(void);

	QString buildDate(void) const;

	//Q_INVOKABLE Session* openSession(const QString& path) const;
	//Q_INVOKABLE Session* newSession(const QString& path) const;

signals:
	void buildDateChanged(void);

protected:
	QQmlApplicationEngine _engine;
};

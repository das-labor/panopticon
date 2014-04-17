#include <QtQuick>

#include "session.hh"

#pragma once

class Panopticon : public QObject
{
	Q_OBJECT
	Q_PROPERTY(QString buildDate READ buildDate NOTIFY buildDateChanged)

public:
	static QObject* provider(QQmlEngine*, QJSEngine*);

	Panopticon(QObject *parent = nullptr);

	QString buildDate(void) const;

	Q_INVOKABLE Session* openSession(const QString& path) const;
	Q_INVOKABLE Session* newSession(const QString& path) const;

signals:
	void buildDateChanged(void);
};

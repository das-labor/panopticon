#include <QtQuick>

#include "session.hh"

#pragma once

class Panopticon : public QObject
{
	Q_OBJECT

public:
	static QObject* provider(QQmlEngine*, QJSEngine*);

	Panopticon(QObject *parent = nullptr);

	Q_INVOKABLE Session* openSession(const QString& path) const;
	Q_INVOKABLE Session* newSession(const QString& path) const;
};

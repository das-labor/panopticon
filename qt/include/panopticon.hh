/*
 * This file is part of Panopticon (http://panopticon.re).
 * Copyright (C) 2014 Kai Michaelis
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

#include <QtQuick>
#include <QtCore>
#include <QApplication>
#include <QQmlApplicationEngine>

#include "session.hh"
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
	Q_INVOKABLE Session* createAvrSession(const QString&);
	Q_INVOKABLE Session* createRawSession(const QString&);
	Session* createSession(Session *s);

signals:
	void buildDateChanged(void);
	void sessionChanged(void);

private:
	Session* _session;

	static Panopticon* _instance;
};

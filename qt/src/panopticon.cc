/*
 * This file is part of Panopticon (http://panopticon.re).
 * Copyright (C) 2014 Panopticon authors
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

#include "panopticon.hh"

Panopticon* Panopticon::_instance = nullptr;

Panopticon::Panopticon(QObject* p) : QObject(p), _session(nullptr)
{}

Panopticon::~Panopticon(void)
{}

Panopticon& Panopticon::instance(void)
{
	if(!_instance)
		_instance = new Panopticon();
	return *_instance;
}

QObject* Panopticon::provider(QQmlEngine*, QJSEngine*)
{
	return &instance();
}

Session* Panopticon::openSession(const QString& path)
{
	qDebug() << "open:" << path;
	try
	{
		return createSession(Session::open(path));
	}
	catch(std::runtime_error const& e)
	{
		qWarning() << e.what();
		return 0;
	}
}

Session* Panopticon::createRawSession(const QString& path)
{
	qDebug() << "create raw:" << path;
	try
	{
		return createSession(Session::createRaw(path));
	}
	catch(std::runtime_error const& e)
	{
		qWarning() << e.what();
		return 0;
	}
}

Session* Panopticon::createAvrSession(const QString& path)
{
	qDebug() << "create AVR:" << path;
	try
	{
		return createSession(Session::createAvr(path));
	}
	catch(std::runtime_error const& e)
	{
		qWarning() << e.what();
		return 0;
	}
}

Session* Panopticon::createSession(Session *s)
{
	if(_session)
	{
		qDebug() << "Replace old session";
		Session* old = _session;
		_session = s;

		emit sessionChanged();
		delete old;
	}
	else
	{
		_session = s;
		emit sessionChanged();
	}

	return _session;
}

Session* Panopticon::session(void) const
{
	return _session;
}

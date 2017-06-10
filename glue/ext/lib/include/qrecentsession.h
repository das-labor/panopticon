/*
 * Panopticon - A libre disassembler
 * Copyright (C) 2017  Panopticon authors
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

#include <QObject>

#include "glue.h"

#pragma once

class QRecentSession : public QObject {
	Q_OBJECT

public:
	QRecentSession(const RecentSession& obj, QObject* parent = nullptr);
	virtual ~QRecentSession();

	Q_PROPERTY(QString title READ getTitle NOTIFY titleChanged)
	Q_PROPERTY(QString kind READ getKind NOTIFY kindChanged)
	Q_PROPERTY(QString path READ getPath NOTIFY pathChanged)
	Q_PROPERTY(quint64 timestamp READ getTimestamp NOTIFY timestampChanged)

	QString getTitle(void) const;
	QString getKind(void) const;
	QString getPath(void) const;
	quint64 getTimestamp(void) const;

signals:
	void titleChanged(void);
	void kindChanged(void);
	void pathChanged(void);
	void timestampChanged(void);

protected:
	QString m_title;
	QString m_kind;
	QString m_path;
	quint64 m_timestamp;
};

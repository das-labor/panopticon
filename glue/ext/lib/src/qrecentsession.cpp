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

#include "qrecentsession.h"

QRecentSession::QRecentSession(const RecentSession& line, QObject* parent)
: QObject(parent), m_title(line.title), m_kind(line.kind), m_timestamp(line.timestamp), m_path(line.path)
{}

QRecentSession::~QRecentSession() {}

QString QRecentSession::getTitle(void) const { return m_title; }
QString QRecentSession::getKind(void) const { return m_kind; }
quint64 QRecentSession::getTimestamp(void) const { return m_timestamp; }
QString QRecentSession::getPath(void) const { return m_path; }

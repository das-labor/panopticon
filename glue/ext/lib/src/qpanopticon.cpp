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

#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <QQmlEngine>
#include <QtQml/qqml.h>
#include <iostream>

#include "qpanopticon.h"

QObject *qpanopticon_provider(QQmlEngine *engine, QJSEngine *scriptEngine) {
    Q_UNUSED(engine)
    Q_UNUSED(scriptEngine)

		QPanopticon::staticInstance = new QPanopticon();
    return QPanopticon::staticInstance;
}

SubscribeToFunc QPanopticon::staticSubscribeTo = nullptr;
GetFunctionFunc QPanopticon::staticGetFunction = nullptr;
OpenProgramFunc QPanopticon::staticOpenProgram = nullptr;
SaveSessionFunc QPanopticon::staticSaveSession = nullptr;
CommentOnFunc QPanopticon::staticCommentOn = nullptr;
RenameFunctionFunc QPanopticon::staticRenameFunction = nullptr;
SetValueForFunc QPanopticon::staticSetValueFor = nullptr;
UndoFunc QPanopticon::staticUndo = nullptr;
RedoFunc QPanopticon::staticRedo = nullptr;
QPanopticon* QPanopticon::staticInstance = nullptr;
QString QPanopticon::staticInitialFile = QString();
std::vector<QRecentSession*> QPanopticon::staticRecentSessions = {};

QPanopticon::QPanopticon()
: m_recentSessions(), m_currentSession(""),
	m_sidebar(new QSidebar(this)), m_sortedSidebar(new QSortFilterProxyModel(this)), m_canUndo(false), m_canRedo(false)
{
  m_sortedSidebar->setSourceModel(m_sidebar);

	for(auto qobj: staticRecentSessions) {
		updateRecentSession(qobj);
	}
	staticRecentSessions.clear();
}

QPanopticon::~QPanopticon() {}

bool QPanopticon::hasRecentSessions(void) const { return m_recentSessions.size() != 0; }
QString QPanopticon::getCurrentSession(void) const { return m_currentSession; }
QString QPanopticon::getInitialFile(void) const { return staticInitialFile; }

QSidebar* QPanopticon::getSidebar(void) const { return m_sidebar; }
QSortFilterProxyModel* QPanopticon::getSortedSidebar(void) const { return m_sortedSidebar; }
unsigned int QPanopticon::getSidebarSortRole(void) const { return m_sortedSidebar->sortRole(); }
bool QPanopticon::getSidebarSortAscending(void) const { return m_sortedSidebar->sortOrder() == Qt::AscendingOrder; }

int QPanopticon::getBasicBlockPadding(void) const { return 3; }
int QPanopticon::getBasicBlockMargin(void) const { return 8; }
int QPanopticon::getBasicBlockLineHeight(void) const { return 17; }
int QPanopticon::getBasicBlockCharacterWidth(void) const { return 8; }
int QPanopticon::getBasicBlockColumnPadding(void) const { return 26; }
int QPanopticon::getBasicBlockCommentWidth(void) const { return 150; }

bool QPanopticon::getCanUndo(void) const { return m_canUndo; }
bool QPanopticon::getCanRedo(void) const { return m_canRedo; }

QString QPanopticon::getLayoutTask(void) const { return m_layoutTask; }

void QPanopticon::setSidebarSortRole(unsigned int role) {
  m_sortedSidebar->setSortRole(role);
  emit sidebarSortRoleChanged();
}

void QPanopticon::setSidebarSortAscending(bool asc) {
  m_sortedSidebar->sort(0, asc ? Qt::AscendingOrder : Qt::DescendingOrder);
  emit sidebarSortAscendingChanged();
}

int QPanopticon::openProgram(QString path) {
	return QPanopticon::staticOpenProgram(path.toStdString().c_str());
}

int QPanopticon::saveSession(QString path) {
	return QPanopticon::staticSaveSession(path.toStdString().c_str());
}

int QPanopticon::commentOn(QString address, QString comment) {
	qlonglong l = address.toLongLong();
	return QPanopticon::staticCommentOn(l,comment.toStdString().c_str());
}

int QPanopticon::renameFunction(QString uuid, QString name) {
	return QPanopticon::staticRenameFunction(uuid.toStdString().c_str(),name.toStdString().c_str());
}

int QPanopticon::setValueFor(QString uuid, QString variable, QString value) {
	return QPanopticon::staticSetValueFor(uuid.toStdString().c_str(),
			variable.toStdString().c_str(),
			value.toStdString().c_str());
}

int QPanopticon::undo() {
	return QPanopticon::staticUndo();
}

int QPanopticon::redo() {
	return QPanopticon::staticRedo();
}

void QPanopticon::updateUndoRedo(bool undo, bool redo) {
	m_canUndo = undo;
	m_canRedo = redo;

	emit canUndoChanged();
	emit canRedoChanged();
}

void QPanopticon::updateCurrentSession(QString path) {
	m_currentSession = path;
	emit currentSessionChanged();
}

void QPanopticon::updateRecentSession(QRecentSession* sess) {
	sess->setParent(this);
	m_recentSessions.append(QVariant::fromValue(sess));
	emit recentSessionsChanged();
}

void QPanopticon::updateLayoutTask(QString task) {
  m_layoutTask = task;
  emit layoutTaskChanged();
}

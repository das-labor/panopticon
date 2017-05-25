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
	m_sidebar(new QSidebar(this)), m_canUndo(false), m_canRedo(false)
{
	for(auto qobj: staticRecentSessions) {
		updateRecentSession(qobj);
	}
	staticRecentSessions.clear();
}

QPanopticon::~QPanopticon() {}

bool QPanopticon::hasRecentSessions(void) const { return m_recentSessions.size() != 0; }
QString QPanopticon::getCurrentSession(void) const { return m_currentSession; }
QSidebar* QPanopticon::getSidebar(void) { return m_sidebar; }

QString QPanopticon::getInitialFile(void) const { return staticInitialFile; }
int QPanopticon::getBasicBlockPadding(void) const { return 3; }
int QPanopticon::getBasicBlockMargin(void) const { return 8; }
int QPanopticon::getBasicBlockLineHeight(void) const { return 17; }
int QPanopticon::getBasicBlockCharacterWidth(void) const { return 8; }
int QPanopticon::getBasicBlockColumnPadding(void) const { return 26; }
int QPanopticon::getBasicBlockCommentWidth(void) const { return 150; }
bool QPanopticon::getCanUndo(void) const { return m_canUndo; }
bool QPanopticon::getCanRedo(void) const { return m_canRedo; }
QString QPanopticon::getLayoutTask(void) const { return m_layoutTask; }

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

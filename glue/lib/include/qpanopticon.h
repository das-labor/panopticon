#include <QObject>
#include <QQmlContext>
#include <QQuickItem>
#include <QVariant>
#include <vector>
#include <memory>
#include <mutex>
#include <cstdint>

#include "qsidebar.h"
#include "qrecentsession.h"
#include "glue.h"

#pragma once

QObject *qpanopticon_provider(QQmlEngine *engine, QJSEngine *scriptEngine);

class QPanopticon : public QObject {
	Q_OBJECT
public:
	QPanopticon();
	virtual ~QPanopticon();

	Q_PROPERTY(QString initialFile READ getInitialFile)

  // recent sessions
	Q_PROPERTY(QVariantList recentSessions MEMBER m_recentSessions NOTIFY recentSessionsChanged)
	Q_PROPERTY(bool hasRecentSessions READ hasRecentSessions NOTIFY hasRecentSessionsChanged)
	Q_PROPERTY(QString currentSession READ getCurrentSession NOTIFY currentSessionChanged)

	Q_PROPERTY(QSidebar* sidebar MEMBER m_sidebar NOTIFY sidebarChanged)

	Q_PROPERTY(unsigned int basicBlockPadding READ getBasicBlockPadding NOTIFY basicBlockPaddingChanged)
	Q_PROPERTY(unsigned int basicBlockMargin READ getBasicBlockMargin NOTIFY basicBlockMarginChanged)
	Q_PROPERTY(unsigned int basicBlockLineHeight READ getBasicBlockLineHeight NOTIFY basicBlockLineHeightChanged)
	Q_PROPERTY(unsigned int basicBlockCharacterWidth READ getBasicBlockCharacterWidth NOTIFY basicBlockCharacterWidthChanged)
	Q_PROPERTY(unsigned int basicBlockColumnPadding READ getBasicBlockColumnPadding NOTIFY basicBlockColumnPaddingChanged)
	Q_PROPERTY(unsigned int basicBlockCommentWidth READ getBasicBlockCommentWidth NOTIFY basicBlockCommentWidthChanged)

	Q_PROPERTY(bool canUndo READ getCanUndo NOTIFY canUndoChanged)
	Q_PROPERTY(bool canRedo READ getCanRedo NOTIFY canRedoChanged)

	bool hasRecentSessions(void) const;
	QString getCurrentSession(void) const;
	QSidebar* getSidebar(void);
	QString getInitialFile(void) const;

	int getBasicBlockPadding(void) const;
	int getBasicBlockMargin(void) const;
	int getBasicBlockLineHeight(void) const;
	int getBasicBlockCharacterWidth(void) const;
	int getBasicBlockColumnPadding(void) const;
	int getBasicBlockCommentWidth(void) const;

	bool getCanUndo(void) const;
	bool getCanRedo(void) const;

	// C to Rust functions
	static GetFunctionNodesFunc staticGetFunctionNodes;
	static GetFunctionEdgesFunc staticGetFunctionEdges;
	static OpenProgramFunc staticOpenProgram;
	static SaveSessionFunc staticSaveSession;
	static CommentOnFunc staticCommentOn;
	static RenameFunctionFunc staticRenameFunction;
	static SetValueForFunc staticSetValueFor;
	static UndoFunc staticUndo;
	static RedoFunc staticRedo;

	// Singleton instance
	static QPanopticon* staticInstance;

	static QString staticInitialFile;
	static std::vector<QRecentSession*> staticRecentSessions;

public slots:
  // session management
	int openProgram(QString path);
	int saveSession(QString path);

  // actions
	int commentOn(QString address, QString comment);
	int renameFunction(QString uuid, QString name);
	int setValueFor(QString uuid, QString variable, QString value);

  // undo/redo
	int undo();
	int redo();

	void updateUndoRedo(bool undo, bool redo);
	void updateCurrentSession(QString path);
	void updateRecentSession(QRecentSession* sess);

signals:
	void recentSessionsChanged(void);
	void hasRecentSessionsChanged(void);
	void currentSessionChanged(void);
	void sidebarChanged(void);
	void basicBlockPaddingChanged(void);
	void basicBlockMarginChanged(void);
	void basicBlockLineHeightChanged(void);
	void basicBlockCharacterWidthChanged(void);
	void basicBlockColumnPaddingChanged(void);
	void basicBlockCommentWidthChanged(void);
	void canUndoChanged(void);
	void canRedoChanged(void);

protected:
	QVariantList m_recentSessions;
	QString m_currentSession;
	QSidebar* m_sidebar;
	bool m_canUndo;
	bool m_canRedo;
};

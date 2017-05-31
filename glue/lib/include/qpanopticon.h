#include <QObject>
#include <QQmlContext>
#include <QQuickItem>
#include <QVariant>
#include <QSortFilterProxyModel>
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

  // sessions
  Q_PROPERTY(QVariantList recentSessions MEMBER m_recentSessions NOTIFY recentSessionsChanged)
  Q_PROPERTY(bool hasRecentSessions READ hasRecentSessions NOTIFY hasRecentSessionsChanged)
  Q_PROPERTY(QString currentSession READ getCurrentSession NOTIFY currentSessionChanged)

  // sidebar
  Q_PROPERTY(QSidebar* sidebar READ getSidebar NOTIFY sidebarChanged)
  Q_PROPERTY(QSortFilterProxyModel* sortedSidebar READ getSortedSidebar NOTIFY sortedSidebarChanged)
  Q_PROPERTY(unsigned int sidebarSortRole READ getSidebarSortRole WRITE setSidebarSortRole NOTIFY sidebarSortRoleChanged)
  Q_PROPERTY(bool sidebarSortAscending READ getSidebarSortAscending WRITE setSidebarSortAscending NOTIFY sidebarSortAscendingChanged)

  // basic block metrics
  Q_PROPERTY(unsigned int basicBlockPadding READ getBasicBlockPadding NOTIFY basicBlockPaddingChanged)
  Q_PROPERTY(unsigned int basicBlockMargin READ getBasicBlockMargin NOTIFY basicBlockMarginChanged)
  Q_PROPERTY(unsigned int basicBlockLineHeight READ getBasicBlockLineHeight NOTIFY basicBlockLineHeightChanged)
  Q_PROPERTY(unsigned int basicBlockCharacterWidth READ getBasicBlockCharacterWidth NOTIFY basicBlockCharacterWidthChanged)
  Q_PROPERTY(unsigned int basicBlockColumnPadding READ getBasicBlockColumnPadding NOTIFY basicBlockColumnPaddingChanged)
  Q_PROPERTY(unsigned int basicBlockCommentWidth READ getBasicBlockCommentWidth NOTIFY basicBlockCommentWidthChanged)

  // undo/redo
  Q_PROPERTY(bool canUndo READ getCanUndo NOTIFY canUndoChanged)
  Q_PROPERTY(bool canRedo READ getCanRedo NOTIFY canRedoChanged)

  // tasks
  Q_PROPERTY(QString layoutTask READ getLayoutTask NOTIFY layoutTaskChanged)

  bool hasRecentSessions(void) const;
  QString getCurrentSession(void) const;
  QString getInitialFile(void) const;

  QSidebar* getSidebar(void) const;
  QSortFilterProxyModel* getSortedSidebar(void) const;
  unsigned int getSidebarSortRole(void) const;
  bool getSidebarSortAscending(void) const;

  int getBasicBlockPadding(void) const;
  int getBasicBlockMargin(void) const;
  int getBasicBlockLineHeight(void) const;
  int getBasicBlockCharacterWidth(void) const;
  int getBasicBlockColumnPadding(void) const;
  int getBasicBlockCommentWidth(void) const;

  bool getCanUndo(void) const;
  bool getCanRedo(void) const;

  QString getLayoutTask(void) const;

  // C to Rust functions
  static GetFunctionFunc staticGetFunction;
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

  void setSidebarSortRole(unsigned int);
  void setSidebarSortAscending(bool);

  void updateUndoRedo(bool undo, bool redo);
  void updateCurrentSession(QString path);
  void updateRecentSession(QRecentSession* sess);
  void updateLayoutTask(QString task);

signals:
  void recentSessionsChanged(void);
  void hasRecentSessionsChanged(void);
  void currentSessionChanged(void);

  void sidebarChanged(void);
  void sortedSidebarChanged(void);
  void sidebarSortRoleChanged(void);
  void sidebarSortAscendingChanged(void);

  void basicBlockPaddingChanged(void);
  void basicBlockMarginChanged(void);
  void basicBlockLineHeightChanged(void);
  void basicBlockCharacterWidthChanged(void);
  void basicBlockColumnPaddingChanged(void);
  void basicBlockCommentWidthChanged(void);

  void canUndoChanged(void);
  void canRedoChanged(void);

  void layoutTaskChanged(void);

protected:
  QVariantList m_recentSessions;
  QString m_currentSession;
  QSidebar* m_sidebar;
  QSortFilterProxyModel* m_sortedSidebar;
  bool m_canUndo;
  bool m_canRedo;
  QString m_layoutTask;
};

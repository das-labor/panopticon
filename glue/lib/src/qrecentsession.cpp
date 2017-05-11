#include "qrecentsession.h"

QRecentSession::QRecentSession(const RecentSession& line, QObject* parent)
: QObject(parent), m_title(line.title), m_kind(line.kind), m_timestamp(line.timestamp), m_path(line.path)
{}

QRecentSession::~QRecentSession() {}

QString QRecentSession::getTitle(void) const { return m_title; }
QString QRecentSession::getKind(void) const { return m_kind; }
quint64 QRecentSession::getTimestamp(void) const { return m_timestamp; }
QString QRecentSession::getPath(void) const { return m_path; }

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

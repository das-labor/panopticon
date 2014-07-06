#include <QtQuick>
#include <QAbstractItemModel>
#include <panopticon/database.hh>

#pragma once

class Session : public QObject
{
	Q_OBJECT
	Q_PROPERTY(QString title READ title NOTIFY titleChanged)
	Q_PROPERTY(QObject* linear READ linear NOTIFY linearChanged)

public:
	Session(po::session, QObject *parent = nullptr);
	virtual ~Session(void);

	Q_INVOKABLE static Session* open(QString);
	Q_INVOKABLE static Session* create(QString);

	QString title(void) const { return QString::fromStdString(_session.dbase->title); }
	QObject* linear(void) const { return _linear; }

signals:
	void titleChanged(void);
	void linearChanged(void);

private:
	po::session _session;
	QStringListModel* _linear;
};

#include <QtQuick>
#include <QAbstractTableModel>
#include <panopticon/database.hh>
#include <panopticon/region.hh>

#pragma once

class LinearModel : public QAbstractListModel
{
	Q_OBJECT

public:
	LinearModel(QObject* p = nullptr);

	virtual int rowCount(const QModelIndex& parent = QModelIndex()) const;
	virtual QVariant data(const QModelIndex& parent = QModelIndex(), int role = Qt::DisplayRole) const;

	void setProjection(const std::list<std::pair<po::bound,po::region_wloc>>& fl);

protected:
	std::list<std::pair<po::bound,po::region_wloc>> _projection;
};

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
	LinearModel* _linear;
};

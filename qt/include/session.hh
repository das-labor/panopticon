#include <QtQuick>
#include <QAbstractTableModel>
#include <panopticon/database.hh>
#include <panopticon/region.hh>

#pragma once

typedef std::pair<po::region_wloc,boost::variant<po::bound,po::bblock_loc,po::struct_loc>> row_t;

class LinearModel : public QAbstractListModel
{
	Q_OBJECT

public:
	LinearModel(po::dbase_loc db, QObject* p = nullptr);

	virtual int rowCount(const QModelIndex& parent = QModelIndex()) const;
	virtual QVariant data(const QModelIndex& parent = QModelIndex(), int role = Qt::DisplayRole) const;

	void postComment(int row, QString c);

protected:
	po::dbase_loc _dbase;
	std::list<std::pair<po::bound,po::region_wloc>> _projection;
	boost::icl::split_interval_map<int,row_t> _rows;
};

class Session : public QObject
{
	Q_OBJECT
	Q_PROPERTY(QString title READ title NOTIFY titleChanged)
	Q_PROPERTY(QObject* linear READ linear NOTIFY linearChanged)
	Q_PROPERTY(QStringList procedures READ procedures NOTIFY proceduresChanged)

public:
	Session(po::session, QObject *parent = nullptr);
	virtual ~Session(void);

	static Session* open(QString);
	static Session* create(QString);

	Q_INVOKABLE void postComment(int row, QString c);
	Q_INVOKABLE void disassemble(int row, int col);

	QString title(void) const { return QString::fromStdString(_session.dbase->title); }
	QObject* linear(void) const { return _linear; }
	const QStringList& procedures(void) const { return _procedures; }

signals:
	void titleChanged(void);
	void linearChanged(void);
	void proceduresChanged(void);

private:
	po::session _session;
	LinearModel* _linear;
	QStringList _procedures;
};

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
	struct data_visitor : public boost::static_visitor<std::tuple<QString,po::bound,std::list<po::bound>>>
	{
		data_visitor(int r, boost::icl::interval<int>::type iv, po::region_loc re);

		std::tuple<QString,po::bound,std::list<po::bound>> operator()(po::bound b) const;
		std::tuple<QString,po::bound,std::list<po::bound>> operator()(po::bblock_loc bb) const;
		std::tuple<QString,po::bound,std::list<po::bound>> operator()(po::struct_loc s) const;

	private:
		int row;
		boost::icl::interval<int>::type ival;
		po::region_loc reg;
	};

	int findTrack(po::bound b);

	po::dbase_loc _dbase;
	std::list<std::pair<po::bound,po::region_wloc>> _projection;
	boost::icl::split_interval_map<int,row_t> _rows;
	std::list<boost::icl::split_interval_set<po::offset>> _tracks;
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

#include "session.hh"

Session::Session(po::session sess, QObject *p)
: QObject(p), _session(sess), _linear(new QStringListModel(this))
{
	QStringList lst;

	lst.append("Test0");
	lst.append("Test1");
	lst.append("Test2");
	lst.append("Test3");
	lst.append("Test4");
	lst.append("Test5");
	lst.append("Test6");
	lst.append("Test7");
	lst.append("Test8");
	lst.append("Test9");

	_linear->setStringList(lst);
}

Session::~Session(void)
{}

Session* Session::open(QString s)
{
	return new Session(po::open(s.toStdString()));
}

Session* Session::create(QString s)
{
	return new Session(po::raw(s.toStdString()));
}

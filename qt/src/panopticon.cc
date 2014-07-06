#include "panopticon.hh"
#include "config.hh"

Panopticon* Panopticon::_instance = nullptr;

Panopticon::Panopticon(QObject* p) : QObject(p), _session(nullptr)
{}

Panopticon::~Panopticon(void)
{}

Panopticon& Panopticon::instance(void)
{
	if(!_instance)
		_instance = new Panopticon();
	return *_instance;
}

QObject* Panopticon::provider(QQmlEngine*, QJSEngine*)
{
	return &instance();
}

Session* Panopticon::openSession(const QString& path)
{
	qDebug() << "open:" << path;

	if(_session)
	{
		qDebug() << "Replace old session";
		Session* old = _session;
		_session = Session::open(path);

		emit sessionChanged();
		delete old;
	}
	else
	{
		_session = Session::open(path);
		emit sessionChanged();
	}

	return _session;
}

Session* Panopticon::createSession(const QString& path)
{
	qDebug() << "create:" << path;

	if(_session)
	{
		qDebug() << "Replace old session";
		Session* old = _session;
		_session = Session::create(path);

		emit sessionChanged();
		delete old;
	}
	else
	{
		_session = Session::create(path);
		emit sessionChanged();
	}

	return Session::create(path);
}

Session* Panopticon::session(void) const
{
	return _session;
}

QString Panopticon::buildDate(void) const
{
	return QString(QT_PANOPTICON_BUILD_DATE);
}

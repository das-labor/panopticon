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
	try
	{
		return createSession(Session::open(path));
	}
	catch(...)
	{
		return 0;
	}
}

Session* Panopticon::createRawSession(const QString& path)
{
	qDebug() << "create raw:" << path;
	try
	{
		return createSession(Session::createRaw(path));
	}
	catch(...)
	{
		return 0;
	}
}

Session* Panopticon::createAvrSession(const QString& path)
{
	qDebug() << "create AVR:" << path;
	try
	{
		return createSession(Session::createAvr(path));
	}
	catch(...)
	{
		return 0;
	}
}

Session* Panopticon::createSession(Session *s)
{
	if(_session)
	{
		qDebug() << "Replace old session";
		Session* old = _session;
		_session = s;

		emit sessionChanged();
		delete old;
	}
	else
	{
		_session = s;
		emit sessionChanged();
	}

	return _session;
}

Session* Panopticon::session(void) const
{
	return _session;
}

QString Panopticon::buildDate(void) const
{
	return QString(QT_PANOPTICON_BUILD_DATE);
}

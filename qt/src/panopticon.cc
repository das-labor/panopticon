#include "panopticon.hh"
#include "config.hh"

QObject* Panopticon::provider(QQmlEngine*, QJSEngine*)
{
	return new Panopticon();
}

Panopticon::Panopticon(QObject *p) : QObject(p) {}

Session* Panopticon::openSession(const QString& path) const
{
	qDebug() << "open:" << path;
	return new Session();
}

Session* Panopticon::newSession(const QString& path) const
{
	qDebug() << "new:" << path;
	return new Session();
}

QString Panopticon::buildDate(void) const
{
	return QString(QT_PANOPTICON_BUILD_DATE);
}

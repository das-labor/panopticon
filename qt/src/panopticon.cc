#include "panopticon.hh"
#include "config.hh"

Panopticon::Panopticon(int& argc, char *argv[], const std::string& root)
: QApplication(argc,argv), _engine(0)
{
	//qmlRegisterType<GraphScenePath>("Panopticon",1,0,"Path");
	//qmlRegisterType<GraphSceneItem>("Panopticon",1,0,"Graph");
	qmlRegisterType<Pen>("Panopticon",1,0,"Pen");
	qmlRegisterType<ElementSelectionObject>("Panopticon",1,0,"ElementSelection");
	qmlRegisterUncreatableType<Session>("Panopticon",1,0,"Session","Use Panopticon.newSession or Panopticon.openSession.");
	//qmlRegisterSingletonType<Panopticon>("Panopticon",1,0,"Panopticon",Panopticon::provider);
	qmlRegisterType<LinearView>("Panopticon",1,0,"LinearView");
	qmlRegisterType<Sugiyama>("Panopticon",1,0,"Sugiyama");

	setOrganizationName("Panopticon");
	setOrganizationDomain("panopticon.re");
	setApplicationName("QtPanopticon");

	_engine.load(QUrl(QString::fromStdString(root)));

	QListIterator<QObject*> iter(_engine.rootObjects());
	while(iter.hasNext())
	{
		QQuickWindow *window = qobject_cast<QQuickWindow *>(iter.next());

		if(window)
	  	window->show();
	}
}

Panopticon::~Panopticon(void)
{}

/*
QObject* Panopticon::provider(QQmlEngine*, QJSEngine*)
{
	return new Panopticon();
}

Session* Panopticon::openSession(const QString& path) const
{
	qDebug() << "open:" << path;
	return new Session();
}

Session* Panopticon::newSession(const QString& path) const
{
	qDebug() << "new:" << path;
	return new Session();
}*/

QString Panopticon::buildDate(void) const
{
	return QString(QT_PANOPTICON_BUILD_DATE);
}

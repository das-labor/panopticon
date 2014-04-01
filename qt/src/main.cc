#include <QtCore>
#include <QApplication>
#include <QQmlApplicationEngine>

#include "linearview.hh"
#include "session.hh"
#include "pen.hh"
#include "selection.hh"
#include "panopticon.hh"

using namespace std;

int main(int argc, char *argv[])
{
	//qmlRegisterType<GraphScenePath>("Panopticon",1,0,"Path");
	//qmlRegisterType<GraphSceneItem>("Panopticon",1,0,"Graph");
	qmlRegisterType<Pen>("Panopticon",1,0,"Pen");
	qmlRegisterType<ElementSelectionObject>("Panopticon",1,0,"ElementSelection");
	qmlRegisterUncreatableType<Session>("Panopticon",1,0,"Session","Use Panopticon.newSession or Panopticon.openSession.");
	qmlRegisterSingletonType<Panopticon>("Panopticon",1,0,"Panopticon",Panopticon::provider);
	qmlRegisterType<LinearView>("Panopticon",1,0,"LinearView");

	QApplication app(argc, argv);

	app.setOrganizationName("Panopticon");
	app.setOrganizationDomain("panopticon.re");
	app.setApplicationName("QtPanopticon");

	QQmlApplicationEngine engine(QUrl("qrc:/Window.qml"));

	QListIterator<QObject*> iter(engine.rootObjects());
	while(iter.hasNext())
	{
		QQuickWindow *window = qobject_cast<QQuickWindow *>(iter.next());

		if(window)
	  	window->show();
	}
	return app.exec();
}

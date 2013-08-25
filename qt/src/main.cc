#include <QtCore>
#include <QtDeclarative>

#include <graph.hh>
#include <interface.hh>
#include <window.hh>

using namespace std;

int main(int argc, char *argv[])
{
	qmlRegisterType<Path>("Panopticon",1,0,"Path");
	qmlRegisterType<Graph>("Panopticon",1,0,"Graph");
	qmlRegisterType<Pen>("Panopticon",1,0,"Pen");

	QApplication app(argc, argv);
	Window win;

	win.show();
	/*QScrollArea scroll;
	QDeclarativeView view(QUrl("qrc:/Procedure.qml"));

	scroll.setWidget(&view);
	scroll.setWidgetResizable(true);
	scroll.show();*/
	return app.exec();
}

#include <QtCore>
#include <QtDeclarative>

#include <scene.hh>
#include <interface.hh>
#include <linearscene.hh>
#include <window.hh>

using namespace std;

int main(int argc, char *argv[])
{
	qmlRegisterType<Path>("Panopticon",1,0,"Path");
	qmlRegisterType<Scene>("Panopticon",1,0,"Graph");
	qmlRegisterType<LinearScene>("Panopticon",1,0,"LinearScene");
	qmlRegisterType<Section>("Panopticon",1,0,"LinearSceneItem");
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

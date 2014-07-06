#include <QCommandLineParser>

#include "panopticon.hh"

using namespace std;

int main(int argc, char *argv[])
{
	QApplication app(argc,argv);

	qmlRegisterType<Pen>("Panopticon",1,0,"Pen");
	qmlRegisterType<ElementSelectionObject>("Panopticon",1,0,"ElementSelection");
	qmlRegisterUncreatableType<Session>("Panopticon",1,0,"Session","Use Panopticon.newSession or Panopticon.openSession.");
	qmlRegisterSingletonType<Panopticon>("Panopticon",1,0,"Panopticon",Panopticon::provider);
	qmlRegisterType<Sugiyama>("Panopticon",1,0,"Sugiyama");

	app.setOrganizationName("Panopticon");
	app.setOrganizationDomain("panopticon.re");
	app.setApplicationName("QtPanopticon");

	QCommandLineParser parser;

	parser.setApplicationDescription("A libre cross platform disassembler");
	parser.addHelpOption();
	parser.addVersionOption();

	QCommandLineOption openOpt(QStringList() << "o" << "open","Open previous session.","file.panop");
	parser.addOption(openOpt);

	QCommandLineOption newOpt(QStringList() << "n" << "new","Disassemble new file.","file");
	parser.addOption(newOpt);

	parser.process(app);

	if(parser.isSet(openOpt) && parser.isSet(newOpt))
		return 1;
	else
	{
		if(parser.isSet(openOpt))
			Panopticon::instance().openSession(parser.value(openOpt));
		else if(parser.isSet(newOpt))
			Panopticon::instance().createSession(parser.value(newOpt));
	}

	QQmlApplicationEngine engine;
	engine.load(QUrl(QString::fromStdString("qrc:/Window.qml")));

	QListIterator<QObject*> iter(engine.rootObjects());
	while(iter.hasNext())
	{
		QQuickWindow *window = qobject_cast<QQuickWindow *>(iter.next());

		if(window)
	  	window->show();
	}

	return app.exec();
}

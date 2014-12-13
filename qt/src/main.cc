/*
 * This file is part of Panopticon (http://panopticon.re).
 * Copyright (C) 2014 Kai Michaelis
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

#include <QCommandLineParser>

#include "panopticon.hh"

using namespace std;

int main(int argc, char *argv[])
{
	QApplication app(argc,argv);

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

	QCommandLineOption rawOpt(QStringList() << "n" << "raw","Open a plain file.","file");
	parser.addOption(rawOpt);

	QCommandLineOption avrOpt(QStringList() << "a" << "avr","Disassemble new AVR file.","file");
	parser.addOption(avrOpt);

	QCommandLineOption peOpt(QStringList() << "p" << "pe","Disassemble new PE (.exe) file.","file");
	parser.addOption(peOpt);

	parser.process(app);

	if(parser.isSet(openOpt) + parser.isSet(rawOpt) + parser.isSet(avrOpt) + parser.isSet(peOpt) > 1)
		return 1;
	else
	{
		if(parser.isSet(openOpt))
			Panopticon::instance().openSession(parser.value(openOpt));
		else if(parser.isSet(rawOpt))
			Panopticon::instance().createRawSession(parser.value(rawOpt));
		else if(parser.isSet(avrOpt))
			Panopticon::instance().createSession(new Session(po::raw_avr(parser.value(avrOpt).toStdString())));
		else if(parser.isSet(peOpt))
			Panopticon::instance().createSession(new Session(po::pe(parser.value(peOpt).toStdString())));
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

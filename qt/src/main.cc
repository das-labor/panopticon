/*
 * This file is part of Panopticon (http://panopticon.re).
 * Copyright (C) 2014 Panopticon authors
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

#include <panopticon/avr/avr.hh>

#include "panopticon.hh"

using namespace std;

void qtMessageHandler(QtMsgType type, QMessageLogContext const& ctx, QString const& msg)
{
	std::cerr << "Qt: " << msg.toLocal8Bit().data() << " (" << ctx.file << ":" << ctx.line << ", " << ctx.function << ")" << std::endl;
	if(type == QtFatalMsg)
	{
		std::cerr << "This is a fatal error. Terminating." << std::endl;
		std::exit(EXIT_FAILURE);
	}
}

int main(int argc, char *argv[])
{
	qInstallMessageHandler(qtMessageHandler);

	QApplication app(argc,argv);

	qmlRegisterUncreatableType<Session>("Panopticon",1,0,"Session","Use Panopticon.newSession or Panopticon.openSession.");
	qmlRegisterSingletonType<Panopticon>("Panopticon",1,0,"Panopticon",Panopticon::provider);
	qmlRegisterType<Sugiyama>("Panopticon",1,0,"Sugiyama");

	app.setOrganizationName("Panopticon");
	app.setOrganizationDomain("panopticon.re");
	app.setApplicationName("QtPanopticon");
	app.setApplicationVersion("0.10.0-dev");

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
	QCommandLineOption avrMcuOpt(QStringList() << "A" << "avr-mcu","Set the MCU to assume when disassembling. Default: mega88.","{ mega103, mega161, mega163, mega168, mega16, mega2561, mega3250, mega3290, mega32, mega48, mega64, mega8535, mega8, mega128, mega162, mega165, mega169, mega2560, mega323, mega325, mega329, mega406, mega649, mega8515, mega88 }");
	parser.addOption(avrMcuOpt);

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
		{
			if(!parser.isSet(avrMcuOpt) || parser.value(avrMcuOpt) == "mega88")
				Panopticon::instance().createSession(new Session(po::raw_avr(parser.value(avrOpt).toStdString(),po::avr_state::mega88())));
			else if(parser.value(avrMcuOpt) == "mega103")
				Panopticon::instance().createSession(new Session(po::raw_avr(parser.value(avrOpt).toStdString(),po::avr_state::mega103())));
			else if(parser.value(avrMcuOpt) == "mega161")
				Panopticon::instance().createSession(new Session(po::raw_avr(parser.value(avrOpt).toStdString(),po::avr_state::mega161())));
			else if(parser.value(avrMcuOpt) == "mega163")
				Panopticon::instance().createSession(new Session(po::raw_avr(parser.value(avrOpt).toStdString(),po::avr_state::mega163())));
			else if(parser.value(avrMcuOpt) == "mega168")
				Panopticon::instance().createSession(new Session(po::raw_avr(parser.value(avrOpt).toStdString(),po::avr_state::mega168())));
			else if(parser.value(avrMcuOpt) == "mega16")
				Panopticon::instance().createSession(new Session(po::raw_avr(parser.value(avrOpt).toStdString(),po::avr_state::mega16())));
			else if(parser.value(avrMcuOpt) == "mega2561")
				Panopticon::instance().createSession(new Session(po::raw_avr(parser.value(avrOpt).toStdString(),po::avr_state::mega2561())));
			else if(parser.value(avrMcuOpt) == "mega3250")
				Panopticon::instance().createSession(new Session(po::raw_avr(parser.value(avrOpt).toStdString(),po::avr_state::mega3250())));
			else if(parser.value(avrMcuOpt) == "mega3290")
				Panopticon::instance().createSession(new Session(po::raw_avr(parser.value(avrOpt).toStdString(),po::avr_state::mega3290())));
			else if(parser.value(avrMcuOpt) == "mega32")
				Panopticon::instance().createSession(new Session(po::raw_avr(parser.value(avrOpt).toStdString(),po::avr_state::mega32())));
			else if(parser.value(avrMcuOpt) == "mega48")
				Panopticon::instance().createSession(new Session(po::raw_avr(parser.value(avrOpt).toStdString(),po::avr_state::mega48())));
			else if(parser.value(avrMcuOpt) == "mega64")
				Panopticon::instance().createSession(new Session(po::raw_avr(parser.value(avrOpt).toStdString(),po::avr_state::mega64())));
			else if(parser.value(avrMcuOpt) == "mega8535")
				Panopticon::instance().createSession(new Session(po::raw_avr(parser.value(avrOpt).toStdString(),po::avr_state::mega8535())));
			else if(parser.value(avrMcuOpt) == "mega8")
				Panopticon::instance().createSession(new Session(po::raw_avr(parser.value(avrOpt).toStdString(),po::avr_state::mega8())));
			else if(parser.value(avrMcuOpt) == "mega128")
				Panopticon::instance().createSession(new Session(po::raw_avr(parser.value(avrOpt).toStdString(),po::avr_state::mega128())));
			else if(parser.value(avrMcuOpt) == "mega162")
				Panopticon::instance().createSession(new Session(po::raw_avr(parser.value(avrOpt).toStdString(),po::avr_state::mega162())));
			else if(parser.value(avrMcuOpt) == "mega165")
				Panopticon::instance().createSession(new Session(po::raw_avr(parser.value(avrOpt).toStdString(),po::avr_state::mega165())));
			else if(parser.value(avrMcuOpt) == "mega169")
				Panopticon::instance().createSession(new Session(po::raw_avr(parser.value(avrOpt).toStdString(),po::avr_state::mega169())));
			else if(parser.value(avrMcuOpt) == "mega2560")
				Panopticon::instance().createSession(new Session(po::raw_avr(parser.value(avrOpt).toStdString(),po::avr_state::mega2560())));
			else if(parser.value(avrMcuOpt) == "mega323")
				Panopticon::instance().createSession(new Session(po::raw_avr(parser.value(avrOpt).toStdString(),po::avr_state::mega323())));
			else if(parser.value(avrMcuOpt) == "mega325")
				Panopticon::instance().createSession(new Session(po::raw_avr(parser.value(avrOpt).toStdString(),po::avr_state::mega325())));
			else if(parser.value(avrMcuOpt) == "mega329")
				Panopticon::instance().createSession(new Session(po::raw_avr(parser.value(avrOpt).toStdString(),po::avr_state::mega329())));
			else if(parser.value(avrMcuOpt) == "mega406")
				Panopticon::instance().createSession(new Session(po::raw_avr(parser.value(avrOpt).toStdString(),po::avr_state::mega406())));
			else if(parser.value(avrMcuOpt) == "mega649")
				Panopticon::instance().createSession(new Session(po::raw_avr(parser.value(avrOpt).toStdString(),po::avr_state::mega649())));
			else if(parser.value(avrMcuOpt) == "mega8515")
				Panopticon::instance().createSession(new Session(po::raw_avr(parser.value(avrOpt).toStdString(),po::avr_state::mega8515())));
			else
			{
				std::cerr << "Unknown AVR MCU" << std::endl;
				return 1;
			}
		}
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
		{
			qDebug() << "show" << window;
			window->show();
		}
		else
		{
			qDebug() << iter.peekPrevious() << "is not a window";
		}
	}

	return app.exec();
}

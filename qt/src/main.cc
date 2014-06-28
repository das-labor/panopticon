#include <QCommandLineParser>

#include "panopticon.hh"

using namespace std;

int main(int argc, char *argv[])
{
	Panopticon app(argc,argv);
	QCommandLineParser parser;

	parser.setApplicationDescription("A libre cross platform disassembler");
	parser.addHelpOption();
	parser.addVersionOption();

	QCommandLineOption openOpt("o","Open previous session.");
	parser.addOption(openOpt);

	QCommandLineOption newOpt("n","Disassemble new file.");
	parser.addOption(newOpt);

	parser.process(app);

	if(parser.isSet(openOpt) ^ parser.isSet(newOpt))
		return app.exec();
	else
		return 1;
}

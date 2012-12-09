#include <QFile>
#include <QTimer>
#include <QCoreApplication>

#include <actions.hh>
#include <avr/avr.hh>

Disassemble::Disassemble(QString path, po::flow_ptr f, po::disassemble_cb sig, QObject *parent)
: QAction(parent), m_path(path), m_flowgraph(f), m_signal(sig)
{
	setText("Disassemble");
	connect(this,SIGNAL(triggered(bool)),this,SLOT(fire(bool)));
}

void Disassemble::fire(bool)
{
	assert(QFile::exists(m_path) && m_flowgraph);
	QTimer::singleShot(0,this,SLOT(disassemble()));
}

void Disassemble::disassemble(void)
{
	std::vector<uint16_t> bytes;
	QFile fd(m_path);

	assert(fd.open(QIODevice::ReadOnly));

	while(!fd.atEnd())
	{
		QByteArray d = fd.read(2);
		assert(d.size() == 2);
		bytes.push_back(*(uint16_t *)d.data());
		QCoreApplication::processEvents();
	}
	fd.close();

	po::avr::disassemble(bytes,0,m_flowgraph,m_signal);
}

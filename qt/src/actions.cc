#include <QFile>
#include <QTimer>
#include <QCoreApplication>

#include <actions.hh>
#include <avr/avr.hh>
#include <marshal.hh>

using namespace po;
using namespace std;

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

Open::Open(QString path, Window *w)
: QAction(w), m_path(path), m_window(w)
{
	assert(w);

	setText("Open");
	connect(this,SIGNAL(triggered(bool)),this,SLOT(fire(bool)));
}

void Open::fire(bool)
{
	assert(QFile::exists(m_path));
	
	rdf::storage store = rdf::storage::from_turtle(m_path.toStdString());
	rdf::stream s = store.select(nullptr,"type"_rdf,"Flowgraph"_po);

	if(!s.eof())
	{
		rdf::statement st;

		s >> st;
		try
		{
			m_window->setFlowgraph(flowgraph::unmarshal(st.subject(),store));
		}
		catch(marshal_exception &e)
		{
			cerr << "Caught exception:" << endl << e.what() << endl;
		}
	}
}

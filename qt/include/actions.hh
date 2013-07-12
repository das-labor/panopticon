#ifndef ACTIONS_HH
#define ACTIONS_HH

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Weffc++"
#include <QAction>
#include <flowgraph.hh>

class Disassemble;

class Disassemble : public QAction
{
	Q_OBJECT

public:
	Disassemble(QString path, po::flow_ptr f, po::disassemble_cb sig, QObject *parent = 0);

public slots:
	void disassemble(void);
	void fire(bool b);

private:
	QString m_path;
	po::flow_ptr m_flowgraph;
	po::disassemble_cb m_signal;
};

#endif

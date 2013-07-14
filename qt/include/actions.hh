#ifndef ACTIONS_HH
#define ACTIONS_HH

#include <QAction>
#include <flowgraph.hh>

class Disassemble;
class Open;

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

class Open : public QAction
{
	Q_OBJECT

public:
	Open(QString path, po::flow_ptr f, std::function<void(void)> cb, QObject *parent = 0);

public slots:
	void open(void);
	void fire(bool b);

private:
	QString m_path;
	po::flow_ptr m_flowgraph;
	std::function<void(void)> m_signal;
};

#endif

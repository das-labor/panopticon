#ifndef ACTIONS_HH
#define ACTIONS_HH

#include <QAction>
#include <flowgraph.hh>

class Disassemble;

#include <model.hh>

class Disassemble : public QAction
{
	Q_OBJECT

public:
	Disassemble(QString path, po::flow_ptr f, std::function<void(void)> sig, QObject *parent = 0);

public slots:
	void disassemble(void);
	void fire(bool b);

private:
	QString m_path;
	po::flow_ptr m_flowgraph;
	std::function<void(void)> m_signal;
};

#endif

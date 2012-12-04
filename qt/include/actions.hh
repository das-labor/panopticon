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
	Disassemble(QString path, Model &m, po::flow_ptr f, QObject *parent = 0);

public slots:
	void disassemble(void);
	void fire(bool b);

private:
	QString m_path;
	Model &m_model;
	po::flow_ptr m_flowgraph;
};

#endif

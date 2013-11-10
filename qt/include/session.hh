#include <QtQuick>
#include <source.hh>

#pragma once

class Session : public QObject
{
	Q_OBJECT

public:
	Session(QObject *parent = nullptr);
	virtual ~Session(void);

	po::graph<po::address_space,po::rrange>& graph(void);

private:
	po::graph<po::address_space,po::rrange> m_graph;
};

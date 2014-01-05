#include <QtQuick>
#include <panopticon/region.hh>

#pragma once

class Session : public QObject
{
	Q_OBJECT

public:
	Session(QObject *parent = nullptr);
	virtual ~Session(void);

	po::regions& graph(void);

private:
	po::regions _regions;
};

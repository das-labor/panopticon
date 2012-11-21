#ifndef BASICBLOCK_WIDGET_HH
#define BASICBLOCK_WIDGET_HH

#include <QGraphicsObject>
#include <QModelIndex>

class BasicBlockWidget : public QGraphicsObject
{
	Q_OBJECT

public:
	BasicBlockWidget(QModelIndex i, QGraphicsItem *parent = 0);
	
	virtual QRectF boundingRect(void) const;
	virtual void paint(QPainter *painter, const QStyleOptionGraphicsItem *option, QWidget *widget = 0);
};

class ControlTransferWidget : public QGraphicsObject
{
	Q_OBJECT

public:
	ControlTransferWidget(const BasicBlockWidget &from, const BasicBlockWidget &to, QGraphicsItem *parent = 0);
};

#endif

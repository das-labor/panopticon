#ifndef BASICBLOCK_WIDGET_HH
#define BASICBLOCK_WIDGET_HH

#include <QGraphicsObject>
#include <QGraphicsTextItem>
#include <QModelIndex>
#include <QAbstractItemModel>

class BasicBlockWidget : public QGraphicsObject
{
	Q_OBJECT

public:
	BasicBlockWidget(QModelIndex i, QGraphicsItem *parent = 0);
	
	virtual QRectF boundingRect(void) const;
	virtual void paint(QPainter *painter, const QStyleOptionGraphicsItem *option, QWidget *widget = 0);

private:
	QGraphicsTextItem m_text;
	const QAbstractItemModel *m_model;
	QPersistentModelIndex m_root;
};

class ControlTransferWidget : public QGraphicsObject
{
	Q_OBJECT

public:
	ControlTransferWidget(const BasicBlockWidget &from, const BasicBlockWidget &to, QGraphicsItem *parent = 0);
};

#endif

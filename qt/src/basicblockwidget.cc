#include <basicblockwidget.hh>
#include <QPainter>

BasicBlockWidget::BasicBlockWidget(QModelIndex i, QGraphicsItem *parent)
: QGraphicsObject(parent)
{
	return;
}

QRectF BasicBlockWidget::boundingRect(void) const
{
	return QRectF(0,0,100,100);
}

void BasicBlockWidget::paint(QPainter *painter, const QStyleOptionGraphicsItem *option, QWidget *widget)
{
	painter->fillRect(0,0,100,100,Qt::red);
}

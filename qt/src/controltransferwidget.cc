#include <QTransform>

#include <controltransferwidget.hh>
#include <model.hh>

ControlTransferWidget::ControlTransferWidget(QModelIndex i, BasicBlockWidget *from, BasicBlockWidget *to, QGraphicsItem *parent)
: QGraphicsItem(parent), m_from(from), m_to(to), 
												 m_text(i.sibling(i.row(),Model::ValuesColumn).data().toString(),this), 
												 m_rect(QRectF(),this),
												 m_path(QPainterPath(),this)
{
	setPath(QPainterPath());
	m_path.setZValue(-2);
	
	m_text.setFont(QFont("Monospace",8));
	
	m_rect.setBrush(QBrush(Qt::red));
	m_rect.setZValue(-1);
	m_rect.setRect(m_text.boundingRect().adjusted(0,0,4,4));
}

void ControlTransferWidget::setPath(QPainterPath pp)
{
	if(false && !pp.isEmpty())
	{
		QPolygonF head;
		QTransform t;
		QPointF ptn = pp.pointAtPercent(1);

		t.rotate(90 - m_path.path().angleAtPercent(1));
		t.translate(ptn.x(),ptn.y());
		head << QPointF(0,0) << QPointF(3*-1.3,3*3) << QPointF(0,3*2.5) << QPointF(3*1.3,3*3) << QPointF(0,0);

		pp.addPolygon(t.map(head));
	}
		
	m_path.setPath(pp);
	m_path.setPos(0,0);
	m_text.setPos(path().pointAtPercent(0.5) - QPointF(m_text.boundingRect().width() / 2.0,m_text.boundingRect().height() / 2.0));
	m_rect.setPos(m_text.pos() - QPointF(2,2));
}

QGraphicsObject *ControlTransferWidget::from(void)
{
	return m_from;
}

QGraphicsObject *ControlTransferWidget::to(void)
{
	return m_to;
}

QPainterPath ControlTransferWidget::path(void) const
{
	return m_path.path();
}

QRectF ControlTransferWidget::boundingRect(void) const
{
	return m_path.boundingRect() | m_text.boundingRect() |  m_rect.boundingRect() |  m_to->boundingRect() |  m_from->boundingRect();
}

void ControlTransferWidget::paint(QPainter *painter, const QStyleOptionGraphicsItem *option, QWidget *widget)
{
	return;
}

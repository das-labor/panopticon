#include <sstream>

#include <QTransform>
#include <QPainter>

#include <controltransferwidget.hh>

ControlTransferWidget::ControlTransferWidget(po::guard g, BasicBlockWidget *from, BasicBlockWidget *to, QGraphicsItem *parent)
: QGraphicsItem(parent), m_from(from), m_to(to),
												 m_text("",this),
												 m_rect(QRectF(),this),
												 m_path(QPainterPath(),this)
{
	setPath(QPainterPath());
	m_path.setZValue(-2);
	m_path.setPen(QPen(Qt::red,2));
	m_head << QPointF(0,0) << QPointF(3*-1.3,3*3) << QPointF(0,3*2.5) << QPointF(3*1.3,3*3) << QPointF(0,0);

	if(g.relations.empty())
	{
		m_text.hide();
		m_rect.hide();
	}
	else
	{
		std::stringstream ss;

		ss << g;
		m_text.setText(QString::fromUtf8(ss.str().c_str()));
		m_text.setFont(QFont("Monospace",8));
		m_rect.setBrush(QBrush(Qt::red));
		m_rect.setZValue(-1);
		m_rect.setRect(m_text.boundingRect().adjusted(0,0,4,4));
	}
}

void ControlTransferWidget::setPath(QPainterPath pp)
{
	m_path.setPath(pp);
	m_path.setPos(0,0);

	if(m_text.isVisible())
	{
		m_text.setPos(path().pointAtPercent(0.5) - QPointF(m_text.boundingRect().width() / 2.0,m_text.boundingRect().height() / 2.0));
		m_rect.setPos(m_text.pos() - QPointF(2,2));
	}
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
	qreal p = std::max(0.0,path().percentAtLength(path().length() - 3*2.5));

	painter->save();
	painter->setPen(QPen(Qt::red,2,Qt::SolidLine,Qt::SquareCap,Qt::MiterJoin));
	painter->setBrush(QBrush(Qt::red));
	painter->translate(path().pointAtPercent(1));
	painter->rotate(90 - path().angleAtPercent(p));
	painter->translate(QPointF(0,-0.55));
	painter->drawConvexPolygon(m_head);
	painter->restore();
}

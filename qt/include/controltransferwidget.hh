#ifndef CONTROLTRANSFERWIDGET_HH
#define CONTROLTRANSFERWIDGET_HH

#include <QGraphicsSimpleTextItem>
#include <QGraphicsRectItem>
#include <QGraphicsPathItem>
#include <QGraphicsObject>
#include <QPolygonF>

#include <basic_block.hh>

#include <basicblockwidget.hh>
#include <graph.hh>

class ControlTransferWidget : public QGraphicsItem, public Arrow
{
public:
	ControlTransferWidget(po::guard_ptr g, BasicBlockWidget *from, BasicBlockWidget *to, QGraphicsItem *parent = 0);
	
	virtual QGraphicsObject *from(void);
	virtual QGraphicsObject *to(void);
	virtual QPainterPath path(void) const;
	virtual void setPath(QPainterPath pp);
	
	virtual QRectF boundingRect(void) const;
	virtual void paint(QPainter *painter, const QStyleOptionGraphicsItem *option, QWidget *widget = 0);

private:
	BasicBlockWidget *m_from;
	BasicBlockWidget *m_to;
	QGraphicsSimpleTextItem m_text;
	QGraphicsRectItem m_rect;
	QGraphicsPathItem m_path;
	QPolygonF m_head;
};

#endif

#ifndef FLOWGRAPHWIDGET_HH
#define FLOWGRAPHWIDGET_HH

#include <graphwidget.hh>

#include <QGraphicsObject>
#include <QGraphicsTextItem>
#include <QGraphicsRectItem>

class FlowgraphNode : public QGraphicsObject
{
	Q_OBJECT

public:
	FlowgraphNode(QString name, QPoint ptn = QPoint(0,0));

	virtual QRectF boundingRect(void) const;
	virtual void paint(QPainter *painter, const QStyleOptionGraphicsItem *option, QWidget *widget = 0);
	
	void setTitle(QString s);

protected:
	virtual QVariant itemChange(GraphicsItemChange change, const QVariant &value);

private:
	QGraphicsTextItem m_text;
	QGraphicsRectItem m_rect;
};

class ProcedureCallWidget : public QGraphicsItem, public Arrow
{
public:
	ProcedureCallWidget(FlowgraphNode *from, FlowgraphNode *to, QGraphicsItem *parent = 0);
	
	virtual QGraphicsObject *from(void);
	virtual QGraphicsObject *to(void);
	virtual QPainterPath path(void) const;
	virtual void setPath(QPainterPath pp);

	void setHighlighted(bool l);
	
	virtual QRectF boundingRect(void) const;
	virtual void paint(QPainter *painter, const QStyleOptionGraphicsItem *option, QWidget *widget = 0);

private:
	FlowgraphNode *m_from;
	FlowgraphNode *m_to;
	QGraphicsPathItem m_path;
	bool m_highlighted;
};

class FlowgraphWidget : public GraphWidget
{
	Q_OBJECT

public:
	FlowgraphWidget(QAbstractItemModel *m, QModelIndex i, QItemSelectionModel *s, QWidget *parent = 0);
	virtual ~FlowgraphWidget(void);

signals:
	void activated(const QModelIndex &idx);

protected:
	virtual QPointF populate(void);
	virtual void mouseDoubleClickEvent(QMouseEvent *event);

private slots:
	void sceneSelectionChanged(void);
	void modelSelectionChanged(const QItemSelection &selected, const QItemSelection &deselected);
	void dataChanged(const QModelIndex &topLeft, const QModelIndex &bottomRight);
	void sceneRectChanged(const QRectF &r);

private:
	std::map<ptrdiff_t,FlowgraphNode *> m_uid2procedure;
	std::map<FlowgraphNode *,int> m_procedure2row;
	QItemSelectionModel *m_selection;
};

#endif

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
	FlowgraphWidget(po::flow_ptr f, QItemSelectionModel *s, QWidget *parent = 0);
	virtual ~FlowgraphWidget(void);

public slots:
	void snapshot(void);

protected:
	virtual void mouseDoubleClickEvent(QMouseEvent *event);

private slots:
	void sceneSelectionChanged(void);
	void modelSelectionChanged(const QItemSelection &selected, const QItemSelection &deselected);
	void sceneRectChanged(const QRectF &r);

private:
	po::flow_ptr m_flowgraph;
	QItemSelectionModel *m_selection;
	std::map<po::proc_ptr,FlowgraphNode *> m_procedureToNode;
};

#endif

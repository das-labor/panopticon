#include <QDebug>
#include <QCoreApplication>
#include <QScrollBar>

#include <graph.hh>

Graph::Graph(QAbstractItemModel *m, QModelIndex i, QWidget *parent)
: QGraphicsView(parent), m_model(m), m_root(i)
{
	setScene(&m_scene);
	setRenderHints(QPainter::Antialiasing | QPainter::SmoothPixmapTransform | QPainter::TextAntialiasing | QPainter::HighQualityAntialiasing);
	setDragMode(QGraphicsView::RubberBandDrag);//QGraphicsView::ScrollHandDrag);

	connect(&m_scene,SIGNAL(sceneRectChanged(const QRectF&)),this,SLOT(sceneRectChanged(const QRectF&)));
}

Graph::~Graph(void)
{
	return;
}

void Graph::sceneRectChanged(const QRectF &r)
{
	fitInView(r,Qt::KeepAspectRatio);
}

void Graph::wheelEvent(QWheelEvent *event)
{
	double fac = (double)(event->delta()) / 150.f;
	fac = fac > 0.0f ? 1 / fac : -fac;
	scale(fac,fac);
}

void Graph::mouseMoveEvent(QMouseEvent *event)
{
	if(event->buttons() & Qt::MiddleButton)
	{
		QPointF p = m_lastDragPos - event->pos();

		horizontalScrollBar()->setValue(horizontalScrollBar()->value() + p.x());
		verticalScrollBar()->setValue(verticalScrollBar()->value() + p.y());
		m_lastDragPos = event->pos();
	}
	else
		QGraphicsView::mouseMoveEvent(event);
}

void Graph::mousePressEvent(QMouseEvent *event)
{
	if(event->button() == Qt::MiddleButton)
	{
		m_lastDragPos = event->pos();
		viewport()->setCursor(Qt::ClosedHandCursor);
	}
	else
		QGraphicsView::mousePressEvent(event);
}

void Graph::mouseReleaseEvent(QMouseEvent *event)
{	
	if(event->button() == Qt::MiddleButton)
		viewport()->setCursor(Qt::ArrowCursor);
	else
		QGraphicsView::mouseReleaseEvent(event);
}

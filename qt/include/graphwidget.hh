#ifndef VIEWPORT_HH
#define VIEWPORT_HH

#include <QGraphicsView>
#include <QWheelEvent>
#include <QItemSelectionModel>

#include <graph.hh>
#include <model.hh>

class GraphWidget : public QGraphicsView
{
	Q_OBJECT

public:
	GraphWidget(QAbstractItemModel *m, QModelIndex i, QWidget *parent = 0);
	virtual ~GraphWidget(void);

	void setRootIndex(const QModelIndex &i);

protected:
	virtual void wheelEvent(QWheelEvent *event);
	virtual void mouseMoveEvent(QMouseEvent *event);
	virtual void mousePressEvent(QMouseEvent *event);
	virtual void mouseReleaseEvent(QMouseEvent *event);

	virtual QPointF populate(void) = 0;

	QAbstractItemModel *m_model;
	Graph m_scene;
	QPersistentModelIndex m_root;
	QPointF m_lastDragPos;
};

#endif

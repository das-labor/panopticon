#ifndef VIEWPORT_HH
#define VIEWPORT_HH

#include <QGraphicsView>
#include <QWheelEvent>
#include <QItemSelectionModel>

#include <scene.hh>
#include <model.hh>

class Graph : public QGraphicsView
{
	Q_OBJECT

public:
	Graph(QAbstractItemModel *m, QModelIndex i, QWidget *parent = 0);
	virtual ~Graph(void);

	void setRootIndex(const QModelIndex &i);

protected:
	virtual void wheelEvent(QWheelEvent *event);
	virtual void mouseMoveEvent(QMouseEvent *event);
	virtual void mousePressEvent(QMouseEvent *event);
	virtual void mouseReleaseEvent(QMouseEvent *event);

	virtual void populate(void) = 0;

	QAbstractItemModel *m_model;
	Scene m_scene;
	QPersistentModelIndex m_root;
	QPointF m_lastDragPos;

private slots:
	void sceneRectChanged(const QRectF &r);
};

#endif

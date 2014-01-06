#include <QtQuick>

#include "pen.hh"

#pragma once

class Polygon : public QQuickItem
{
	Q_OBJECT
	Q_PROPERTY(Pen *pen READ pen)
	Q_PROPERTY(QColor background READ background WRITE setBackground NOTIFY backgroundChanged)
	Q_PROPERTY(QVariantList points READ points WRITE setPoints NOTIFY pointsChanged)

public:
	Polygon(void);

	Pen *pen(void);
	QColor background(void) const;
	QVariantList points(void);

	void setBackground(const QColor &);
	void setPoints(const QVariantList &);

signals:
	void backgroundChanged(void);
	void pointsChanged(void);

protected:
	virtual QSGNode *updatePaintNode(QSGNode *oldNode, UpdatePaintNodeData *updatePaintNodeData);

	Pen _pen;
	QColor _background;
	QVariantList _points;
};

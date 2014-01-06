#include "polygon.hh"

Polygon::Polygon(void)
: QQuickItem(), _pen(), _background(Qt::white), _points()
{
	setFlag(QQuickItem::ItemHasContents);
}

Pen *Polygon::pen(void) { return &_pen; }
QColor Polygon::background(void) const { return _background; }
QVariantList Polygon::points(void) { return _points; }
/*
void Polygon::setPen(const Pen &p)
{
	if(_pen != p)
	{
		_pen = p;
		emit penChanged();
		update();
	}
}*/

void Polygon::setBackground(const QColor &c)
{
	if(_background != c)
	{
		_background = c;
		emit backgroundChanged();
		update();
	}
}

void Polygon::setPoints(const QVariantList &v)
{
	if(_points != v)
	{
		_points = v;
		emit pointsChanged();
		update();
	}
}

QSGNode *Polygon::updatePaintNode(QSGNode *oldNode, UpdatePaintNodeData *updatePaintNodeData)
{
	if(oldNode)
		delete oldNode;

	QSGGeometry *geometry = new QSGGeometry(QSGGeometry::defaultAttributes_Point2D(), _points.size());
	//geometry->setDrawingMode(GL_LINES);
	geometry->setLineWidth(_pen.widthF());

	auto i = geometry->vertexDataAsPoint2D();
	QListIterator<QVariant> j(_points);

	while(j.hasNext())
	{
		QPointF ptn = j.next().toPointF();

		i->set(ptn.x(),ptn.y());
		++i;
	}

	QSGFlatColorMaterial *material = new QSGFlatColorMaterial;
	material->setColor(_pen.color());

	QSGGeometryNode *node = new QSGGeometryNode;
	node->setGeometry(geometry);
	node->setFlag(QSGNode::OwnsGeometry);
	node->setMaterial(material);
	node->setFlag(QSGNode::OwnsMaterial);

	return node;
}

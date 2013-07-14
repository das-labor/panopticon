#ifndef PATH_HH
#define PATH_HH

#include <QtDeclarative>
#include <QPainter>

class Pen : public QObject, public QPen
{
	Q_OBJECT
	Q_PROPERTY(QColor color READ color WRITE setColor)
	Q_PROPERTY(qreal width READ width WRITE setWidth)
	Q_PROPERTY(PenStyle style READ style WRITE setStyle)
	Q_PROPERTY(CapStyle capStyle READ capStyle WRITE setCapStyle)
	Q_PROPERTY(JoinStyle joinStyle READ joinStyle WRITE setJoinStyle)
	Q_ENUMS(PenStyle)
	Q_ENUMS(CapStyle)
	Q_ENUMS(JoinStyle)

public:
	enum PenStyle
	{
		NoPen =	0,
		SolidLine = 1,
		DashLine = 2,
		DotLine = 3,
		DashDotLine = 4,
		DashDotDotLine = 5,
		CustomDashLine = 6,
	};

	enum CapStyle
	{
		FlatCap = 0x00,
		SquareCap = 0x10,
		RoundCap = 0x20,
	};

	enum JoinStyle
	{
		MiterJoin = 0x00,
		BevelJoin = 0x40,
		RoundJoin = 0x80,
		SvgMiterJoin = 0x100,
	};

	Pen(void) {}

	QColor color(void) const { return QPen::color(); }
	qreal width(void) const { return QPen::widthF(); }
	PenStyle style(void) const { return static_cast<PenStyle>(QPen::style()); }
	CapStyle capStyle(void) const { return static_cast<CapStyle>(QPen::capStyle()); }
	JoinStyle joinStyle(void) const { return static_cast<JoinStyle>(QPen::joinStyle()); }

	void setColor(const QColor &x) { QPen::setColor(x); emit changed(); }
	void setWidth(qreal x) { QPen::setWidthF(x); }
	void setStyle(PenStyle x) { QPen::setStyle(static_cast<Qt::PenStyle>(x)); emit changed(); }
	void setCapStyle(CapStyle x) { QPen::setCapStyle(static_cast<Qt::PenCapStyle>(x)); emit changed(); }
	void setJoinStyle(JoinStyle x) { QPen::setJoinStyle(static_cast<Qt::PenJoinStyle>(x)); emit changed(); }

signals:
	void changed(void);
};

class Path : public QDeclarativeItem
{
	Q_OBJECT
	Q_PROPERTY(QDeclarativeItem* from READ from WRITE setFrom)
	Q_PROPERTY(QDeclarativeItem* to READ to WRITE setTo)
	Q_PROPERTY(bool direct READ isDirect WRITE setDirect)
	Q_PROPERTY(Pen *pen READ pen)
	Q_PROPERTY(QDeclarativeItem* head READ head WRITE setHead)
	Q_PROPERTY(QDeclarativeItem* tail READ tail WRITE setTail)

public:
	Path(QDeclarativeItem *from = 0, QDeclarativeItem *to = 0,QDeclarativeItem *parent = 0);

	void setPath(const QPainterPath &pp);
	void setDirect(bool b);
	void setPen(const QPen &p);
	void setFrom(QDeclarativeItem *obj);
	void setTo(QDeclarativeItem *obj);
	void setHead(QDeclarativeItem *obj);
	void setTail(QDeclarativeItem *obj);

	virtual QRectF boundingRect() const;
	virtual void paint(QPainter *painter, const QStyleOptionGraphicsItem *option, QWidget *widget);

	QDeclarativeItem *from(void) const;
	QDeclarativeItem *to(void) const;
	QDeclarativeItem *head(void) const;
	QDeclarativeItem *tail(void) const;
	bool isDirect(void) const;
	Pen *pen(void);

public slots:
	void updateGeometry(void);
	void update(void);

signals:
	void nodesChanged(void);

protected:
	virtual void hoverMoveEvent(QGraphicsSceneHoverEvent *event);

private:
	QLineF contactVector(QDeclarativeItem *itm) const;
	qreal approximateDistance(const QPointF &pnt) const;
	void positionEnds(void);

	QDeclarativeItem *m_from;
	QDeclarativeItem *m_to;
	QDeclarativeItem *m_head;
	QDeclarativeItem *m_tail;
	QPainterPath m_path;
	Pen m_pen;
	bool m_direct;
	QRectF m_boundingRect;
	QPointF m_fromCenter, m_toCenter;
};

#endif

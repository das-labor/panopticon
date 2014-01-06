#include <QQmlListProperty>
#include <QList>
#include <QDebug>

#include <QtQml>
#include <QtQuick>
#include <QPainter>
#include <QPen>

#pragma once

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
	Pen(const Pen &p) : QObject(p.parent()), QPen(p) {}
	virtual ~Pen(void) {}

	Pen& operator=(const Pen &p) { QPen::operator=(p); return *this; }

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

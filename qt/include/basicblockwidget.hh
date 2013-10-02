#ifndef BASICBLOCK_WIDGET_HH
#define BASICBLOCK_WIDGET_HH

#include <QGraphicsObject>
#include <QGraphicsSimpleTextItem>
#include <QGraphicsTextItem>

#include <flowgraph.hh>
#include <procedure.hh>
#include <basic_block.hh>
#include <mnemonic.hh>

class BasicBlockWidget;
class MnemonicWidget;

class BasicBlockWidget : public QGraphicsObject
{
	Q_OBJECT

public:
	BasicBlockWidget(po::flow_ptr flow, po::proc_ptr proc, po::bblock_ptr bb, QGraphicsItem *parent = 0);

	virtual QRectF boundingRect(void) const;
	virtual void paint(QPainter *painter, const QStyleOptionGraphicsItem *option, QWidget *widget = 0);

protected:
	virtual void mousePressEvent(QGraphicsSceneMouseEvent *event);
//	virtual void mouseMoveEvent(QGraphicsSceneMouseEvent *event);

private:
	QVector<MnemonicWidget *> m_mnemonics;
	po::bblock_ptr m_basic_block;
	QGraphicsSimpleTextItem m_instructions;
};

class MnemonicWidget : public QGraphicsItem
{
public:
	MnemonicWidget(po::flow_ptr flow, po::proc_ptr proc, const po::mnemonic &mne, QGraphicsItem *parent = 0);

	void setIdent(double s);
	double ident(void) const;

	virtual QRectF boundingRect(void) const;
	virtual void paint(QPainter *painter, const QStyleOptionGraphicsItem *option, QWidget *widget = 0);

private:
	QGraphicsSimpleTextItem m_mnemonic;
	QVector<QGraphicsItem *> m_operands;
	double m_ident;
};

class OperandWidget : public QGraphicsTextItem
{
public:
	OperandWidget(po::flow_ptr flow, po::proc_ptr proc, po::rvalue v, const po::mnemonic::token &tok, QGraphicsItem *parent = 0);

	virtual void paint(QPainter *painter, const QStyleOptionGraphicsItem *option, QWidget *widget);

protected:
	virtual void hoverEnterEvent(QGraphicsSceneHoverEvent *event);
	virtual void hoverLeaveEvent(QGraphicsSceneHoverEvent *event);

private:
	bool m_marked;
};

#endif

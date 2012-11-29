#include <basicblockwidget.hh>
#include <QPainter>
#include <QTextDocument>
#include <model.hh>

BasicBlockWidget::BasicBlockWidget(QModelIndex i, QGraphicsItem *parent)
: QGraphicsObject(parent), m_model(i.model()), m_root(i)
{
	int row = 0;
	QModelIndex mne_idx = m_root.sibling(m_root.row(),Model::MnemonicsColumn);
	double y = 0, ident = 0;
	QFontMetrics f(QFont("Monospace",11));

	while(row < m_model->rowCount(mne_idx))
	{
		QModelIndex mne = mne_idx.child(row,Model::OpcodeColumn);
		
		if(!mne.data().toString().startsWith("internal"))
		{
			m_mnemonics.append(new MnemonicWidget(mne,this));
			m_mnemonics.last()->setPos(0,y);
			if(!row)
				m_mnemonics.last()->setSelected(true);
			y += f.lineSpacing()*1.25;
			ident = std::max(ident,m_mnemonics.last()->ident());
		}
		++row;
	}

	QVectorIterator<MnemonicWidget *> j(m_mnemonics);
	while(j.hasNext())
	{
		MnemonicWidget *s = j.next();
		s->setIdent(ident);
	}
}

QRectF BasicBlockWidget::boundingRect(void) const
{
	QRectF ret;
	QVectorIterator<MnemonicWidget *> j(m_mnemonics);
	
	while(j.hasNext())
	{
		MnemonicWidget *s = j.next();
		ret = ret.united(s->boundingRect().translated(s->pos()));
	}

	return ret.adjusted(-5,-5,8,8);
}

void BasicBlockWidget::paint(QPainter *painter, const QStyleOptionGraphicsItem *option, QWidget *widget)
{
	painter->drawRect(boundingRect());
}

MnemonicWidget::MnemonicWidget(QModelIndex i, QGraphicsItem *parent)
: QGraphicsItem(parent), m_mnemonic(this)
{
	QModelIndex opcode = i.sibling(i.row(),Model::OpcodeColumn);
	QModelIndex ops = i.sibling(i.row(),Model::OperandsColumn);
	int op_row = 0;
	bool left_used = false;
	std::function<void(QString)> add = [&](QString str)
	{
		QGraphicsSimpleTextItem *a = new QGraphicsSimpleTextItem(this);
		m_operands.append(a);
		a->setFont(QFont("Monospace",11));
		a->setText(str);
	};

	m_mnemonic.setFont(QFont("Monospace",11));
	m_mnemonic.setText(opcode.data().toString());

	while(op_row < ops.model()->rowCount(ops))
	{
		QModelIndex d = ops.child(op_row,Model::DecorationColumn);
		QStringList deco = d.data().toStringList();

		assert(deco.size() == 2);
		if(deco[0].size() && !left_used)
			add(deco[0]);

		m_operands.append(new OperandWidget(d,this));
		
		if(deco[1].size())
		{
			add(deco[1]);
			left_used = true;
		}

		++op_row;
	}
	
	setIdent(m_mnemonic.boundingRect().width() + 10);
	setFlag(QGraphicsItem::ItemIsSelectable);
}

void MnemonicWidget::setIdent(double i)
{
	m_ident = i;
	m_mnemonic.setPos(0,0);

	double x = m_ident;
	QVectorIterator<QGraphicsItem *> j(m_operands);
	while(j.hasNext())
	{
		QGraphicsItem *s = j.next();
		s->setPos(x,0);
		x += s->boundingRect().width();
	}
}

double MnemonicWidget::ident(void) const
{
	return m_ident;
}

QRectF MnemonicWidget::boundingRect(void) const
{
	QRectF ret = m_mnemonic.boundingRect().translated(m_mnemonic.pos());
	QVectorIterator<QGraphicsItem *> j(m_operands);
	
	while(j.hasNext())
	{
		QGraphicsItem *s = j.next();
		ret = ret.united(s->boundingRect().translated(s->pos()));
	}

	return ret;
}

void MnemonicWidget::paint(QPainter *painter, const QStyleOptionGraphicsItem *option, QWidget *widget)
{	
	if(isSelected())
	{
		painter->save();
		painter->setPen(QPen(Qt::blue,1));
		painter->setBrush(QBrush(QColor(0,0,255,60)));
	//	painter->fillRect(boundingRect());
		painter->drawRect(boundingRect());
		painter->restore();
	}
	return;
}

OperandWidget::OperandWidget(QModelIndex i, QGraphicsItem *parent)
: QGraphicsTextItem(parent), m_marked(isUnderMouse())
{
	QModelIndex value = i.sibling(i.row(),Model::ValueColumn);
	QModelIndex sscp = i.sibling(i.row(),Model::SscpColumn);

	document()->setDocumentMargin(0);

	if(sscp.data().toString().size())
		setHtml(value.data().toString() + " <i>(" + sscp.data().toString() + ")</i>");
	else
		setPlainText(value.data().toString());

	setFont(QFont("Monospace",11));
	setAcceptHoverEvents(true);
}

void OperandWidget::hoverEnterEvent(QGraphicsSceneHoverEvent *event)
{
	m_marked = true;
	update();
}

void OperandWidget::hoverLeaveEvent(QGraphicsSceneHoverEvent *event)
{
	m_marked = false;
	update();
}

void OperandWidget::paint(QPainter *painter, const QStyleOptionGraphicsItem *option, QWidget *widget)
{
	QGraphicsTextItem::paint(painter,option,widget);
	if(m_marked)
	{
		painter->save();
		painter->setPen(QPen(Qt::transparent,0));
		painter->setBrush(QBrush(QColor(0,120,120,60)));
	//	painter->fillRect(boundingRect());
		painter->drawRect(boundingRect());
		painter->restore();
	}
}



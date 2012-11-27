#include <basicblockwidget.hh>
#include <QPainter>
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
			y += f.lineSpacing();
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
	int op_idx = 0;
	QString op_str = ops.data().toString();

	m_mnemonic.setFont(QFont("Monospace",11));
	m_mnemonic.setText(opcode.data().toString());

	while(op_row < ops.model()->rowCount(ops))
	{
		QModelIndex op = ops.child(op_row,Model::PositionColumn);
		QPoint ptn = op.data().toPoint();

		assert(ptn.x() >= op_idx && ptn.x() <= ptn.y() && ptn.x() < op_str.length());
		if(ptn.x() > op_idx)
			qDebug() << op_str.left(ptn.x()).right(ptn.x() - op_idx);
		qDebug() << op_str.left(ptn.y()).right(ptn.y() - ptn.x());

		op_idx = ptn.y();
		++op_row;
	}

	if(op_idx < op_str.length())
	qDebug() << op_str.right(op_str.length() - op_idx);
	
	m_operands.append(new QGraphicsSimpleTextItem(this));
	m_operands.last()->setFont(QFont("Monospace",11));
	m_operands.last()->setText(ops.data().toString());
	
	setIdent(m_mnemonic.boundingRect().width() + 10);
}

void MnemonicWidget::setIdent(double i)
{
	m_ident = i;
	m_mnemonic.setPos(0,0);

	double x = m_ident;
	QVectorIterator<QGraphicsSimpleTextItem *> j(m_operands);
	while(j.hasNext())
	{
		QGraphicsSimpleTextItem *s = j.next();
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
	QVectorIterator<QGraphicsSimpleTextItem *> j(m_operands);
	
	while(j.hasNext())
	{
		QGraphicsSimpleTextItem *s = j.next();
		ret = ret.united(s->boundingRect().translated(s->pos()));
	}

	return ret;
}

void MnemonicWidget::paint(QPainter *painter, const QStyleOptionGraphicsItem *option, QWidget *widget)
{
	return;
}

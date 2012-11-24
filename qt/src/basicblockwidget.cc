#include <basicblockwidget.hh>
#include <QPainter>
#include <model.hh>

BasicBlockWidget::BasicBlockWidget(QModelIndex i, QGraphicsItem *parent)
: QGraphicsObject(parent), m_text("",this), m_model(i.model()), m_root(i)
{
	int row = 0;
	QModelIndex mne_idx = m_root.sibling(m_root.row(),Model::MnemonicsColumn);

	m_text.setFont(QFont("Monospace",9));

	while(row < m_model->rowCount(mne_idx))
	{
		QModelIndex mne = mne_idx.child(row,Model::OpcodeColumn);
		QString line =  mne.data().toString() + "\t";
		QModelIndex op_idx = mne_idx.child(row,Model::OperandsColumn);
		int op_row = 0;

		/*while(op_row < m_model->rowCount(op_idx))
		{
			QModelIndex op = op_idx.child(op_row,Model::ValueColumn);
			
			line += op.data().toString() + " ";
			++op_row;
		}*/

		line += op_idx.data().toString();

		m_text.setPlainText(m_text.toPlainText() + line);
		++row;

		if(row < m_model->rowCount(mne_idx))
			m_text.setPlainText(m_text.toPlainText() + "\n");
	}

	m_text.adjustSize();
}

QRectF BasicBlockWidget::boundingRect(void) const
{
	QRectF bb = m_text.boundingRect();

	return bb;
}

void BasicBlockWidget::paint(QPainter *painter, const QStyleOptionGraphicsItem *option, QWidget *widget)
{
	painter->drawRect(boundingRect());
}

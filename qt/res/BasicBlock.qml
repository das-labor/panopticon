import QtQuick 1.0

Rectangle
{
	width: 120; height: 120
	color: "#00000000"

	Rectangle
	{
		id: rect
		width: 120; height: 120;
		color: "green"
	}

	states:
	[
		State
		{
			name: ""
			PropertyChanges { target: rect; color: "green" }
		},
		State
		{
			name: "grabbed"
			PropertyChanges { target: rect; color: "red" }
		},
		State
		{
			name: "hidden"
			PropertyChanges { target: rect; visible: false }
		}
	]
	transitions: [ Transition { to: "*"; ColorAnimation { target: rect; duration: 500 } } ]
}

/*
BasicBlockWidget::BasicBlockWidget(po::flow_ptr flow, po::proc_ptr proc, po::bblock_ptr bb, QGraphicsItem *parent)
: QGraphicsObject(parent), m_basic_block(bb), m_instructions(this)
{
	double y = 8, ident = 0;
	QFontMetrics f(QFont("Monospace",11));

	for(const po::mnemonic &mne: bb->mnemonics())
	{
		if(!QString::fromStdString(mne.opcode).startsWith("internal"))
		{
			m_mnemonics.append(new MnemonicWidget(flow,proc,mne,this));
			m_mnemonics.last()->setPos(8,y);
			//if(!row)
			//	m_mnemonics.last()->setSelected(true);
			y += f.lineSpacing()*1.25;
			ident = std::max(ident,m_mnemonics.last()->ident());
		}
	}

	QVectorIterator<MnemonicWidget *> j(m_mnemonics);
	while(j.hasNext())
	{
		MnemonicWidget *s = j.next();
		s->setIdent(ident);
	}

	//const std::map<po::proc_ptr,std::shared_ptr<std::map<po::rvalue,po::sscp_lattice>>> &sscp = flow->simple_sparse_constprop;
	execute2(bb,[&](const po::instr &i)
	{
		std::stringstream ss;

		ss << i;
		//if(sscp.count(proc) && sscp.at(proc)->count(i.left))
		//	ss << " | " <<  sscp.at(proc)->at(i.left);

		m_instructions.setText(m_instructions.text() + (m_instructions.text().size() ? "\n" : "") + QString::fromUtf8(ss.str().c_str()));
	});

	m_instructions.hide();
	m_instructions.setZValue(1);
	m_instructions.setBrush(QBrush(Qt::blue));
	m_instructions.setPos(QPoint(-m_instructions.boundingRect().width(),0));

	setCursor(QCursor(Qt::ArrowCursor));
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

	if(m_instructions.isVisible())
		ret = ret.united(m_instructions.boundingRect().translated(m_instructions.pos()));

	return QRectF(QPointF(0,0),ret.bottomRight() + QPoint(8,8));
}

void BasicBlockWidget::paint(QPainter *painter, const QStyleOptionGraphicsItem *option, QWidget *widget)
{
	painter->save();
	painter->setPen(QPen(Qt::green,1));
	painter->drawRect(boundingRect().adjusted(2,2,-4,-4));
	painter->restore();
}

void BasicBlockWidget::mousePressEvent(QGraphicsSceneMouseEvent *event)
{
	prepareGeometryChange();

	if(m_instructions.isVisible())
		m_instructions.hide();
	else
		m_instructions.show();
}

MnemonicWidget::MnemonicWidget(po::flow_ptr flow, po::proc_ptr proc, const po::mnemonic &mne, QGraphicsItem *parent)
: QGraphicsItem(parent), m_mnemonic(this)
{
	unsigned int ops = 0;

	for(const po::mnemonic::token &tok: mne.format)
	{
		if(tok.is_literal)
		{
			QGraphicsSimpleTextItem *a = new QGraphicsSimpleTextItem(this);
			m_operands.append(a);
			a->setFont(QFont("Monospace",11));
			a->setText(QString::fromStdString(tok.alias));
			//a->hide();
		}
		else
		{
			assert(ops < mne.operands.size());
			m_operands.append(new OperandWidget(flow,proc,mne.operands[ops++],tok,this));
		}
	}

	m_mnemonic.setFont(QFont("Monospace",11));
	m_mnemonic.setText(QString::fromStdString(mne.opcode));
	//m_mnemonic.hide();

	setIdent(m_mnemonic.boundingRect().width() + 10);
	//setFlag(QGraphicsItem::ItemIsSelectable);
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
		//painter->fillRect(boundingRect());
		painter->drawRect(boundingRect());
		painter->restore();
	}
	return;
}

OperandWidget::OperandWidget(po::flow_ptr flow, po::proc_ptr proc, po::rvalue v, const po::mnemonic::token &tok, QGraphicsItem *parent)
: QGraphicsTextItem(parent), m_marked(isUnderMouse())
{
	document()->setDocumentMargin(0);

	if(tok.alias.empty())
	{
		if(v.is_variable())
			setPlainText(QString::fromStdString(v.to_variable().name()));
		else if(v.is_constant())
			setPlainText(QString::fromStdString(std::to_string(format_constant(tok,v.to_constant().content()))));
		else
		{
			std::stringstream ss;
			ss << v;
			setPlainText(QString::fromUtf8(ss.str().c_str()));
		}
	}
	else
		setPlainText(QString::fromStdString(tok.alias));

	/// @todo
	const std::map<po::proc_ptr,std::shared_ptr<std::map<po::rvalue,po::sscp_lattice>>> &sscp = flow->simple_sparse_constprop;
	if(sscp.count(proc) && sscp.at(proc)->count(v) && sscp.at(proc)->at(v).type == po::sscp_lattice::Const)
		setHtml(toPlainText() + " <i>(" + QString::fromStdString(std::to_string(format_constant(tok,sscp.at(proc)->at(v).value))) + ")</i>");

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
}*/

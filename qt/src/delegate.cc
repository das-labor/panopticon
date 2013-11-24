#include <delegate.hh>

Delegate::Delegate(const po::address_space &as, const po::rrange &r, QObject *p)
: QObject(p), m_space(as), m_range(r)
{}

Delegate::~Delegate(void) {}

const po::address_space &Delegate::space(void) const { return m_space; }
const po::rrange &Delegate::range(void) const { return m_range; }

/*
const boost::optional<ElementSelection> &Delegate::cursor(void)
{
	if(database()->cursor() && section()->position().includes(*database()->cursor()))
	{
		ByteSelection *sel = new ByteSelection(*database()->cursor());
		boost::optional<ElementSelection> ret = elementSelection(sel);

		delete sel;
		return ret;
	}
	else
		return 0;
}

const boost::optional<ElementSelection> &Delegate::mouse(void)
{
	if(database()->mouse() && section()->position().includes(*database()->mouse()))
	{
		ByteSelection *sel = new ByteSelection(*database()->mouse());
		boost::optional<ElementSelection> ret = elementSelection(sel);

		delete sel;
		return ret;
	}
	else
		return 0;
}

boost::optional<ElementSelection> Delegate::elementSelection(const po::range &sel)
{
	if(!sel)
		return 0;

	quint64 line = 0;
	quint64 anc_l = 0, cur_l = 0;
	bool anc_set = false, cur_set = false, anc_freeze = false, cur_freeze = false;

	MetadataSet::Zipper z = spans().zipper();
	while(z.hasNext() && !(anc_freeze && cur_freeze))
	{
		SpanRef span = dynamic_pointer_cast<Span,Metadata>(z.next());

		if(sel->includes(span->position()))
		{
			if(!anc_freeze && sel->anchorByte() < sel->cursorByte())
			{
				anc_l = line;
				anc_set = anc_freeze = true;
			}
			else if(!cur_freeze && sel->anchorByte() >= sel->cursorByte())
			{
				cur_l = line;
				cur_set = cur_freeze = true;
			}
		}

		if(!anc_freeze && sel->anchorByte() >= span->position().firstByte() && sel->anchorByte() <= span->position().lastByte())
		{
			anc_l = line;
			anc_set = true;
		}

		if(!cur_freeze && sel->cursorByte() >= span->position().firstByte() && sel->cursorByte() <= span->position().lastByte())
		{
			cur_l = line;
			cur_set = true;
		}

		line++;
	}

	if(anc_set && cur_set)
		return new ElementSelection(anc_l,0,cur_l,0);
	else
		return 0;
}

po::rrange Delegate::byteSelection(const boost::optional<ElementSelection> &sel)
{
	if(sel)
	{
		if(spans().size() <= std::max(sel->anchorLine(),sel->cursorLine()))
			return 0;

		const ByteSelection &anc(spans().zipper(sel->anchorLine()+1).current()->position());
		const ByteSelection &cur(spans().zipper(sel->cursorLine()+1).current()->position());

		if(sel->anchorLine() > sel->cursorLine())
			return new ByteSelection(anc.lastByte(),cur.firstByte());
		else
			return new ByteSelection(anc.firstByte(),cur.lastByte());
	}
	else
		return 0;
}*/

TestDelegateContext::TestDelegateContext(QObject *parent)
: QObject(parent), m_address(), m_data(), m_row(-1)
{}

TestDelegateContext::TestDelegateContext(const QString &a, const QVariantList &d, int r, QObject *p)
: QObject(p), m_address(a), m_data(d), m_row(r)
{}

QString TestDelegateContext::address(void) const		{ return m_address; }
QVariantList TestDelegateContext::data(void) const	{ return m_data; }
int TestDelegateContext::row(void) const						{ return m_row; }

TestDelegate::TestDelegate(const po::address_space &as, const po::rrange &r, unsigned int w, QQmlEngine *e, QQuickItem *p)
: Delegate(as,r,p), m_width(w), m_engine(e),
	m_rowComponent(m_engine,QUrl("qrc:/Test.qml")),
	m_headComponent(m_engine,QUrl("qrc:/Block.qml")),
	m_cursorComponent(m_engine,QUrl("qrc:/Cursor.qml")),
	m_cursor(boost::none), m_visibleRows(), m_overlay(0)
{
	assert(w);
	qDebug() << m_rowComponent.errors();
	qDebug() << m_headComponent.errors();
	qDebug() << m_cursorComponent.errors();

	m_overlay = qobject_cast<QQuickItem*>(m_cursorComponent.create());
	assert(m_overlay);
	m_overlay->setParent(this);
	m_overlay->setParentItem(p);
	m_overlay->setVisible(false);
}

TestDelegate::~TestDelegate(void) {}

unsigned int TestDelegate::rowCount(void) const
{
	size_t l = boost::icl::length(range());
	return l / m_width + !!(l % m_width);
}

QQuickItem *TestDelegate::createRow(unsigned int l)
{
	auto ctx = new QQmlContext(m_engine->rootContext());
	unsigned int i = 0, w = (l == rowCount() - 1 && boost::icl::length(range()) % m_width ? boost::icl::length(range()) % m_width : m_width);
	QVariantList _data;

	assert(l < rowCount());

	while(i < w)
	{
		_data.append(QVariant::fromValue(QString("??")));//new Element("??",m_cursor && m_cursor->includes(ElementSelection(l,i,l,i)))));
		i++;
	}

	ctx->setContextProperty("testDelegateContext",new TestDelegateContext(QString("%1").arg(l * m_width),_data,l,ctx));

	auto ret = qobject_cast<QQuickItem*>(m_rowComponent.create(ctx));

	assert(ret);
	connect(ret,SIGNAL(elementEntered(int,int)),this,SLOT(elementEntered(int,int)));
	connect(ret,SIGNAL(elementClicked(int,int)),this,SLOT(elementClicked(int,int)));
	ctx->setParent(ret);
	ret->setParent(this);
	ret->setParentItem(m_overlay->parentItem());

	m_visibleRows.insert(std::make_pair(l,ret));
	reattachCursor();
	return ret;
}

void TestDelegate::deleteRow(QQuickItem *i)
{
	assert(i);
	auto j = std::find_if(m_visibleRows.begin(),m_visibleRows.end(),[&](std::pair<int,QQuickItem*> p) { return p.second == i; });

	assert(j != m_visibleRows.end());
	m_visibleRows.erase(j);
	reattachCursor();
	i->deleteLater();
}

QQuickItem *TestDelegate::createHead(void)
{
	auto ret = qobject_cast<QQuickItem*>(m_headComponent.create());

	ret->setParent(this);
	return ret;
}

void TestDelegate::deleteHead(QQuickItem *i)
{
	assert(i);
	i->deleteLater();
}

void TestDelegate::reattachCursor(void)
{
	if(m_cursor)
	{
		auto fi = m_visibleRows.lower_bound(m_cursor->firstLine());
		auto li = m_visibleRows.lower_bound(m_cursor->lastLine());

		if(fi != m_visibleRows.end() && fi->first >= m_cursor->firstLine() && fi->first <= m_cursor->lastLine())
		{
			if(li == m_visibleRows.end() || li->first > m_cursor->lastLine())
				--li;
			if(li->first >= m_cursor->firstLine() && li->first <= m_cursor->lastLine())
			{
				QVariant ret, first = QVariant::fromValue(fi->second), last = QVariant::fromValue(li->second);
				QMetaObject::invokeMethod(m_overlay,"attach",Q_RETURN_ARG(QVariant,ret),Q_ARG(QVariant,first),Q_ARG(QVariant,last));
				m_overlay->setVisible(true);

				return;
			}
		}
	}

	m_overlay->setVisible(false);
}

void TestDelegate::elementClicked(int col, int row)
{
	if(m_cursor)
		setCursor(boost::none);
	else
		setCursor(ElementSelection(row,col,row,col));
}

void TestDelegate::elementEntered(int col, int row)
{
	if(m_cursor)
	{
		ElementSelection sel = *m_cursor;
		sel.setCursor(row,col);
		setCursor(sel);
	}
	else
		setCursor(ElementSelection(row,col,row,col));
}

void TestDelegate::setCursor(const boost::optional<ElementSelection> &sel)
{
	if(sel != m_cursor)
	{
		auto old = m_cursor;

		m_cursor = sel;
		reattachCursor();

	/*
		// one if NULL other !NULL: redraw only !NULL one
		if((sel && !old) || (!sel && old))
		{
			auto t = sel ? sel : old;
			emit modified(ElementSelection(t->firstLine(),0,t->lastLine(),m_width-1));
		}
		// both selection smth: redraw difference
		else
		{
			// moved only cursor
			if(sel->anchorLine() == old->anchorLine())
			{
				emit modified(ElementSelection(std::min(sel->cursorLine(),old->cursorLine()),0,
																			 std::max(sel->cursorLine(),old->cursorLine()),m_width-1));
			}
			// give up and repaint everything
			else
			{
				emit modified(ElementSelection(std::min(sel->firstLine(),old->firstLine()),0,
																			 std::max(sel->lastLine(),old->lastLine()),m_width-1));
			}
		}*/
	}
}
/*
Element::Element(void)
: QObject(), m_data(""), m_selected(false)
{}

Element::Element(const QString &h, bool sel)
: QObject(), m_data(h), m_selected(sel) {}

Element::~Element(void)
{}

QString Element::data(void) const { return m_data; }
bool Element::selected(void) const { return m_selected; }*/

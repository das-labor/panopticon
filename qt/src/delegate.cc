#include <delegate.hh>
#include <graph.hh>

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
	m_overlays(), m_visibleRows(),
	m_cursor(boost::none), m_cursorOverlay(0)
{
	assert(w);
	qDebug() << m_rowComponent.errors();
	qDebug() << m_headComponent.errors();
	qDebug() << m_cursorComponent.errors();
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
		_data.append(QVariant::fromValue(QString("??")));
		i++;
	}

	ctx->setContextProperty("testDelegateContext",new TestDelegateContext(QString("%1").arg(l * m_width),_data,l,ctx));

	auto ret = qobject_cast<QQuickItem*>(m_rowComponent.create(ctx));

	assert(ret);
	connect(ret,SIGNAL(elementEntered(int,int)),this,SLOT(elementEntered(int,int)));
	connect(ret,SIGNAL(elementClicked(int,int)),this,SLOT(elementClicked(int,int)));
	ctx->setParent(ret);
	ret->setParent(this);
	ret->setParentItem(qobject_cast<QQuickItem*>(parent()));

	m_visibleRows.insert(std::make_pair(l,ret));
	updateOverlays(ElementSelection(l,0,l,m_width-1));

	return ret;
}

void TestDelegate::deleteRow(QQuickItem *i)
{
	assert(i);
	auto j = std::find_if(m_visibleRows.begin(),m_visibleRows.end(),[&](std::pair<int,QQuickItem*> p) { return p.second == i; });

	assert(j != m_visibleRows.end());
	int l = j->first;

	m_visibleRows.erase(j);
	updateOverlays(ElementSelection(l,0,l,m_width-1));
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

QQuickItem *TestDelegate::createOverlay(const ElementSelection &sel)
{
	QQuickItem *ov = qobject_cast<QQuickItem*>(m_cursorComponent.create());

	assert(ov);
	ov->setParent(this);
	ov->setParentItem(qobject_cast<QQuickItem*>(parent()));
	ov->setVisible(false);

	m_overlays.insert(std::make_pair(sel,ov));
	updateOverlays(sel);

	return ov;
}

void TestDelegate::deleteOverlay(QQuickItem *ov)
{
	assert(ov);
	auto i = std::find_if(m_overlays.begin(),m_overlays.end(),[&](const std::pair<ElementSelection,QQuickItem*> &p) { return p.second == ov; });
	assert(i != m_overlays.end());
	auto key = i->first;

	m_overlays.erase(i);
	updateOverlays(key);
	ov->deleteLater();
}

void TestDelegate::updateOverlays(const ElementSelection &sel)
{
	for(auto i: m_overlays)
	{
		if(!i.first.disjoint(sel))
		{
			auto fi = m_visibleRows.lower_bound(i.first.firstLine());
			auto li = m_visibleRows.lower_bound(i.first.lastLine());

			if(fi != m_visibleRows.end() && fi->first >= i.first.firstLine() && fi->first <= i.first.lastLine())
			{
				if(li == m_visibleRows.end() || li->first > i.first.lastLine())
					--li;
				if(li->first >= i.first.firstLine() && li->first <= i.first.lastLine())
				{
					QVariant ret, first = QVariant::fromValue(fi->second), last = QVariant::fromValue(li->second);
					QMetaObject::invokeMethod(i.second,"attach",Q_RETURN_ARG(QVariant,ret),Q_ARG(QVariant,first),Q_ARG(QVariant,last));
					i.second->setVisible(true);

					continue;
				}
			}

			i.second->setVisible(false);
		}
	}
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

		if(m_cursorOverlay)
			deleteOverlay(m_cursorOverlay);

		m_cursor = sel;

		if(m_cursor)
			m_cursorOverlay = createOverlay(*m_cursor);
		else
			m_cursorOverlay = 0;

		// one if NULL other !NULL: redraw only !NULL one
		if((sel && !old) || (!sel && old))
		{
			auto t = sel ? sel : old;
			updateOverlays(ElementSelection(t->firstLine(),0,t->lastLine(),m_width-1));
		}
		// both selection smth: redraw difference
		else
		{
				updateOverlays(ElementSelection(std::min(sel->firstLine(),old->firstLine()),0,
																				std::max(sel->lastLine(),old->lastLine()),m_width-1));
		}
	}
}

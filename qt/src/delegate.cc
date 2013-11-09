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

TestDelegate::TestDelegate(const po::address_space &as, const po::rrange &r, unsigned int w)
: Delegate(as,r), m_width(w)
{
	assert(w);
}

TestDelegate::~TestDelegate(void) {}

unsigned int TestDelegate::lines(void) const
{
	size_t l = boost::icl::length(range());
	return l / m_width + !!(l % m_width);
}

unsigned int TestDelegate::width(unsigned int l) const
{
	return m_width;
}

QVariant TestDelegate::line(unsigned int l) const
{
	QList<QVariant> ret;
	unsigned int i = 0, w = (l == lines() - 1 && boost::icl::length(range()) % m_width ? boost::icl::length(range()) % m_width : m_width);

	assert(l < lines());

	while(i < w)
	{
		ret.append(QVariant::fromValue(new Element("??",m_cursor && m_cursor->includes(ElementSelection(l,i,l,i)))));
		i++;
	}
	return QVariant::fromValue(ret);
}

void TestDelegate::setCursor(const boost::optional<ElementSelection> &sel)
{
	if(sel != m_cursor)
	{
		auto old = m_cursor;

		m_cursor = sel;

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
		}
	}
}

Element::Element(void)
: QObject(), m_data(""), m_selected(false)
{}

Element::Element(const QString &h, bool sel)
: QObject(), m_data(h), m_selected(sel) {}

Element::~Element(void)
{}

QString Element::data(void) const { return m_data; }
bool Element::selected(void) const { return m_selected; }

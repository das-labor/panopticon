#include <cassert>
#include <linearview.hh>

Header::Header(void) : QObject(), m_name(""), m_collapsed(false), m_id(-1) {}
Header::Header(const QString &h, bool col, int id) : QObject(), m_name(h), m_collapsed(col), m_id(id) {}
Header::~Header(void) {}

QString Header::name(void) const { return m_name; }
bool Header::collapsed(void) const { return m_collapsed; }
int Header::id(void) const { return m_id; }

LinearViewBlock::LinearViewBlock(void)
: type(Data), delegate(), id(-1)
{}

LinearViewBlock::LinearViewBlock(LinearViewBlock::Type t, QSharedPointer<Delegate> d, int i)
: type(t), delegate(d), id(i)
{}

LinearViewBlock::LinearViewBlock(const LinearViewBlock &o)
: type(o.type), delegate(o.delegate), id(o.id)
{}

bool LinearViewBlock::operator==(const LinearViewBlock &r) const
{
	return type == r.type && delegate == r.delegate && id == r.id;
}

LinearViewBlock &LinearViewBlock::operator+=(const LinearViewBlock &r)
{
	return *this;
}

LinearViewContext::LinearViewContext(QObject *parent)
: QObject(parent), m_columnWidth(0)
{}

qreal LinearViewContext::columnWidth(void) const
{
	return m_columnWidth;
}

void LinearViewContext::setColumnWidth(qreal r)
{
	if(r != m_columnWidth)
	{
		m_columnWidth = r;
		emit columnWidthChanged();
	}
}

LinearView::LinearView(QQuickItem *parent)
: QQuickItem(parent), m_engine(), m_context(), m_session(nullptr), m_visibleRows(), m_visibleTopRow(0)
{
	setFlags(QQuickItem::ItemHasContents);
	setAcceptedMouseButtons(Qt::LeftButton);

	m_engine.rootContext()->setContextProperty("linearViewContext",&m_context);

	addRows();
	setClip(true);
}

LinearView::~LinearView(void)
{}

Session *LinearView::session(void)
{
	return m_session;
}

void LinearView::setSession(Session *s)
{
	if(s != m_session)
	{
		m_session = s;

		int id = 0;
		int i = 0;

		m_availableBlocks.clear();
		m_hiddenBlocks.clear();
		//m_firstRow = m_lastRow = m_firstColumn = m_lastColumn = -1;

		for(auto p: po::projection(m_session->graph().get_node(po::root(m_session->graph())),m_session->graph()))
		{
			QSharedPointer<Delegate> delegate(new TestDelegate(p.second,p.first,10,&m_engine,this));
			auto len = delegate->rowCount();

			m_availableBlocks.add(std::make_pair(decltype(m_availableBlocks)::interval_type::right_open(i,i + 1),LinearViewBlock(LinearViewBlock::Header,delegate,id)));
			m_availableBlocks.add(std::make_pair(decltype(m_availableBlocks)::interval_type::right_open(i + 1,i + 1 + len),LinearViewBlock(LinearViewBlock::Data,delegate,id)));

			connect(delegate.data(),SIGNAL(modified()),this,SLOT(delegateModified()));

			i += len + 1;
			id += 1;
		}

		emit sessionChanged();
	}
}

QQuickItem *LinearView::createRow(int idx)
{
	auto iter = m_availableBlocks.find(idx);
	QQuickItem *ret = nullptr;

	if(iter == m_availableBlocks.end())
		return nullptr;

	if(iter->second.type == LinearViewBlock::Data)
		ret = iter->second.delegate->createRow(idx - boost::icl::first(iter->first));
	else
		ret = iter->second.delegate->createHead();

	if(ret)
	{
		ret->setParentItem(this);
		ret->setX(0);
		connect(ret,SIGNAL(heightChanged()),this,SLOT(rowHeightChanged()));
	}

	return ret;
}

void LinearView::delegateModified(void)
{
	auto del = qobject_cast<Delegate*>(sender());

	assert(del);
	auto i = m_visibleRows.begin();
	qreal offset = (*i)->y();

	while(i != m_visibleRows.end())
	{
		auto row = std::distance(m_visibleRows.begin(),i) + m_visibleTopRow;
		auto j = m_availableBlocks.find(row);

		assert(j != m_availableBlocks.end());
		if(j->second.type == LinearViewBlock::Header)
			j->second.delegate->deleteHead(*i);
		else
			j->second.delegate->deleteRow(*i);

		++i;
	}

	auto bak = m_availableBlocks;
	int id = 0, k = 0;

	m_visibleRows.clear();
	m_availableBlocks.clear();
	for(auto j: bak)
	{
		if(j.second.type == LinearViewBlock::Header)
		{
			auto len = j.second.delegate->rowCount();

			m_availableBlocks.add(std::make_pair(decltype(m_availableBlocks)::interval_type::right_open(k,k + 1),LinearViewBlock(LinearViewBlock::Header,j.second.delegate,id)));
			m_availableBlocks.add(std::make_pair(decltype(m_availableBlocks)::interval_type::right_open(k + 1,k + 1 + len),LinearViewBlock(LinearViewBlock::Data,j.second.delegate,id)));

			k += len + 1;
			id += 1;
		}
	}

	scrollViewport(0);
	scrollViewport(offset);

}

void LinearView::addRows(bool up)
{
	assert(!(m_visibleRows.empty() && up));

	qreal yy = (m_visibleRows.empty() ? 0 : (up ? m_visibleRows.front()->y() : m_visibleRows.back()->y() + m_visibleRows.back()->height()));
	int idx = m_visibleTopRow + (up ? -1 : m_visibleRows.size());

	while(yy >= y() && yy < y() + height())
	{
		QQuickItem *itm = createRow(idx);

		if(!itm)
			return;

		if(up)
		{
			m_visibleRows.push_front(itm);
			--idx;
			--m_visibleTopRow;
			itm->setY(yy - itm->height());
			yy += 1 - itm->height();
		}
		else
		{
			m_visibleRows.push_back(itm);
			++idx;
			itm->setY(yy);
			yy += itm->height();
		}
	}
}

void LinearView::rowHeightChanged(void)
{
	QQuickItem *prev = 0;
	std::for_each(std::find(m_visibleRows.begin(),m_visibleRows.end(),sender()),m_visibleRows.end(),[&](QQuickItem *itm)
	{
		if(prev)
			itm->setY(prev->y() + prev->height());
		prev = itm;
	});
}

void LinearView::wheelEvent(QWheelEvent *event)
{
	scrollViewport(event->angleDelta().y() / 8);
}

void LinearView::mouseMoveEvent(QMouseEvent *event)
{
	if(event->buttons() & Qt::LeftButton)
	{
		QPointF ptn = event->localPos();
		auto i = std::find_if(m_visibleRows.begin(),m_visibleRows.end(),[&](QQuickItem *j)
		{
			QRectF bb(j->x(),j->y(),j->width(),j->height());
			return bb.contains(ptn);
		});

		if(i != m_visibleRows.end())
		{
			QQuickItem *itm = *i;

			QVariant ret;
			QMetaObject::invokeMethod(itm,"mouseMoved",Q_RETURN_ARG(QVariant,ret),Q_ARG(QVariant,ptn.x() - itm->x()),Q_ARG(QVariant,ptn.y() - itm->y()));
			event->accept();
		}
	}
}

void LinearView::mousePressEvent(QMouseEvent *event)
{
	if(event->buttons() & Qt::LeftButton)
	{
		QPointF ptn = event->localPos();
		auto i = std::find_if(m_visibleRows.begin(),m_visibleRows.end(),[&](QQuickItem *j)
		{
			QRectF bb(j->x(),j->y(),j->width(),j->height());
			return bb.contains(ptn);
		});

		if(i != m_visibleRows.end())
		{
			QQuickItem *itm = *i;

			QVariant ret;
			QMetaObject::invokeMethod(itm,"mousePressed",Q_RETURN_ARG(QVariant,ret),Q_ARG(QVariant,ptn.x() - itm->x()),Q_ARG(QVariant,ptn.y() - itm->y()));
			event->accept();
		}
	}
}

void LinearView::geometryChanged(const QRectF&, const QRectF&)
{
	scrollViewport(0);
}

unsigned int LinearView::rowCount(void) const
{
	return boost::icl::length(m_availableBlocks);
}

void LinearView::scrollViewport(qreal d)
{
	QRectF bb(x(),y(),width(),height());

	if(d &&
		 !m_visibleTopRow &&
		 m_visibleRows.size() >= rowCount() &&
		 m_visibleRows.front()->y() >= bb.top() &&
		 m_visibleRows.back()->y() + m_visibleRows.back()->height() <= bb.bottom())
		return;

	if(d)
	{
		// move elements
		for(auto i: m_visibleRows)
			i->setY(i->y() + d);
	}

	// delete elements out of sight
	auto i = m_visibleRows.begin();
	while(i != m_visibleRows.end())
	{
		if((bb & QRectF((*i)->x(),(*i)->y(),(*i)->width(),(*i)->height())).isNull())
		{
			auto row = std::distance(m_visibleRows.begin(),i) + m_visibleTopRow;
			auto j = m_availableBlocks.find(row);

			assert(j != m_availableBlocks.end());
			if(j->second.type == LinearViewBlock::Header)
				j->second.delegate->deleteHead(*i);
			else
				j->second.delegate->deleteRow(*i);

			if(i == m_visibleRows.begin())
				++m_visibleTopRow;
			i = m_visibleRows.erase(i);
		}
		else
		{
			++i;
		}
	}

	// add new elements
	if(m_visibleRows.empty())
		addRows(false);
	else
	{
		if(m_visibleRows.front()->y() > y())
			addRows(true);
		if(m_visibleRows.back()->y() < y() + height())
			addRows(false);

		// Make sure we don't scroll past the first/last element
		if(d)
		{
			if(!m_visibleTopRow)
			{
				qreal delta = m_visibleRows.front()->y() - bb.top();

				if(delta > 0)
				{
					scrollViewport(-delta);
					return;
				}
			}

			if(m_visibleTopRow + m_visibleRows.size() >= rowCount())
			{
				qreal delta = bb.bottom() - (m_visibleRows.back()->y() + m_visibleRows.back()->height());

				if(delta > 0)
					scrollViewport(delta);
			}
		}
	}
}

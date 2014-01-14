#include <cassert>
#include "linearview.hh"

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

std::shared_ptr<Delegate> operator+=(std::shared_ptr<Delegate> &a, const std::shared_ptr<Delegate> &b)
{
	return (a = b);
}

LinearView::LinearView(QQuickItem *parent)
: QQuickItem(parent), _engine(), _context(), _session(nullptr), _delegates(), _globalRowIndex(0), _yOffset(0), _visibleRows(), _references()
{
	setFlags(QQuickItem::ItemHasContents);
	setAcceptedMouseButtons(Qt::LeftButton);

	_engine.rootContext()->setContextProperty("linearViewContext",&_context);
	scrollViewport();
	setClip(true);
}

LinearView::~LinearView(void)
{}

Session *LinearView::session(void)
{
	return _session;
}

void LinearView::setSession(Session *s)
{
	if(s != _session)
	{
		_session = s;
		_references.clear();
		_delegates.clear();

		rowIndex gri = 0;
		for(auto p: po::projection(_session->graph()))
		{
			std::shared_ptr<Delegate> del = std::make_shared<TestDelegate>(p.second,10,&_engine,this);
			auto len = del->rowCount();

			_delegates += std::make_pair(decltype(_delegates)::interval_type::right_open(gri,gri+len),del);
			connect(del.get(),SIGNAL(modified()),this,SLOT(delegateModified()));
			gri += len;
		}

		emit sessionChanged();
	}
}
/*
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
}*/

void LinearView::delegateModified(void)
{
	/*
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
*/
	qWarning() << "LinearView::delegateModified not implemented";
}
/*
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
}*/

void LinearView::rowHeightChanged(void)
{/*
	QQuickItem *prev = 0;
	std::for_each(std::find(m_visibleRows.begin(),m_visibleRows.end(),sender()),m_visibleRows.end(),[&](QQuickItem *itm)
	{
		if(prev)
			itm->setY(prev->y() + prev->height());
		prev = itm;
	});*/
	qWarning() << "LinearView::rowHeightChanged not implemented";
}

void LinearView::wheelEvent(QWheelEvent *event)
{
	scrollViewport(event->angleDelta().y() / 8);
}

void LinearView::mouseMoveEvent(QMouseEvent *event)
{
	/*if(event->buttons() & Qt::LeftButton)
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
	}*/
	qWarning() << "LinearView::mouseMoveEvent not implemented";
}

void LinearView::mousePressEvent(QMouseEvent *event)
{
	/*if(event->buttons() & Qt::LeftButton)
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
	}*/
	qWarning() << "LinearView::mousePressEvent not implemented";
}

void LinearView::geometryChanged(const QRectF&, const QRectF&)
{
	scrollViewport();
}

/*
unsigned int LinearView::rowCount(void) const
{
	return boost::icl::length(m_availableBlocks);
}*/

void LinearView::scrollViewport(float delta)
{
	QRectF bb(x(),y(),width(),height());
	int y;
	rowIndex i = _globalRowIndex, firstVisibleRowIndex = std::numeric_limits<rowIndex>::max();
	auto j = _visibleRows.begin();

	while(j != _visibleRows.end())
	{
		const std::pair<rowIndex,QQuickItem*> &p = *j;
		const QRectF &itemBB = p.second->boundingRect();

		if(bb.intersects(QRectF(mapFromItem(p.second,itemBB.topLeft() + QPointF(0,delta)),itemBB.size())))
		{
			firstVisibleRowIndex = std::min(firstVisibleRowIndex,p.first);
			p.second->setY(p.second->y() + delta);
			++j;
		}
		else
		{
			std::shared_ptr<Delegate> del = _delegates.find(p.first)->second;

			del->deleteRow(p.second);
			j = _visibleRows.erase(j);
		}
	}

	if(firstVisibleRowIndex == std::numeric_limits<rowIndex>::max())
	{
		i = 0;
		float y = 0;

		while(i < boost::icl::length(_delegates))
		{
			auto j = _delegates.find(i);

			if(j != _delegates.end())
			{
				rowIndex l = i - boost::icl::first(j->first);
				qDebug() << j->second.get() << l << i;
				QQuickItem *itm = j->second->createRow(l);

				if(itm)
				{
					itm->setParentItem(this);
					itm->setX(0);
					itm->setY(y);
					
					connect(itm,SIGNAL(heightChanged()),this,SLOT(rowHeightChanged()));
					assert(_visibleRows.emplace(i,itm).second);
					y += itm->height();
				}

				++i;
			}
			else
				break;
		}
	}

	qDebug() << "done";





/*

	tie(_yOffset,_globalRowIndex) = newGlobalRowIndex(delta);

	y = _yOffset;
	gri = _globalRowIndex;

		while(*/
/*
	if(d &&
		 !m_visibleTopRow &&
		 m_visibleRfirstVisibleRowIndex = std::numeric_limits<rowIndex>::max();ows.size() >= rowCount() &&
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
	}*/
}

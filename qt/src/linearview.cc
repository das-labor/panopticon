#include <cassert>
#include "linearview.hh"

using namespace std;

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
		_ordinals.clear();

		rowIndex gri = 0;
		size_t ord = 0;

		for(auto p: po::projection(_session->graph()))
		{
			std::shared_ptr<Delegate> del = std::make_shared<BinaryDelegate>(p.second,16,&_engine,this);
			auto len = del->rowCount();

			_delegates += std::make_pair(decltype(_delegates)::interval_type::right_open(gri,gri+len),del);
			connect(del.get(),SIGNAL(modified()),this,SLOT(delegateModified()));
			connect(del.get(),SIGNAL(selected(boost::optional<po::offset>,bool)),this,SLOT(selected(boost::optional<po::offset>,bool)));
			gri += len;

			assert(_ordinals.emplace(del.get(),ord++).second);
		}

		emit sessionChanged();
		scrollViewport();
	}
}

void LinearView::selected(boost::optional<po::offset> cur, bool start_new)
{
	Delegate *del = qobject_cast<Delegate*>(sender());
	assert(del);
	size_t regIdx = regionIndex(del);
	boost::optional<DelegateSelection> sel = boost::none;

	if(cur)
	{
		if(start_new || !_cursor)
			sel = DelegateSelection(DelegatePoint(regIdx,*cur),DelegatePoint(regIdx,*cur));
		else
			sel = DelegateSelection(_cursor->anchor(),DelegatePoint(regIdx,*cur));
	}

	if(sel != _cursor)
	{
		size_t topReg = std::min(sel ? sel->minimum().first : std::numeric_limits<size_t>::max(),_cursor ? _cursor->minimum().first : std::numeric_limits<size_t>::max()),
					 botReg = std::max(sel ? sel->maximum().first : std::numeric_limits<size_t>::min(),_cursor ? _cursor->maximum().first : std::numeric_limits<size_t>::min());
		auto i = std::next(_delegates.begin(),topReg);

		while(i != std::next(_delegates.begin(),botReg + 1))
		{
			Delegate *cur_del = i->second.get();
			regIdx = regionIndex(cur_del);
			const po::offset null = 0;

			// in new selection?
			if(sel && regIdx >= sel->minimum().first && regIdx <= sel->maximum().first)
			{
				if(regIdx == sel->minimum().first && regIdx == sel->maximum().first)
					i->second->select(boost::make_optional(std::make_pair(sel->anchor().second,sel->cursor().second)));
				else if(regIdx == sel->minimum().first)
				{
					if(regIdx == sel->anchor().first)
						i->second->select(boost::make_optional(std::make_pair(sel->anchor().second,cur_del->region()->size() - 1)));
					else
						i->second->select(boost::make_optional(std::make_pair(cur_del->region()->size() - 1,sel->cursor().second)));
				}
				else if(regIdx == sel->maximum().first)
				{
					if(regIdx == sel->anchor().first)
						i->second->select(boost::make_optional(std::make_pair(sel->anchor().second,null)));
					else
						i->second->select(boost::make_optional(std::make_pair(null,sel->cursor().second)));
				}
				else
				{
					if(sel->anchor().first < sel->cursor().first)
						i->second->select(boost::make_optional(std::make_pair(null,cur_del->region()->size() - 1)));
					else
						i->second->select(boost::make_optional(std::make_pair(cur_del->region()->size() - 1,null)));
				}
			}
			else
				i->second->select(boost::none);

			++i;
		}

		_cursor = sel;
	}
}

size_t LinearView::regionIndex(const Delegate *del) const
{
	assert(del);
	auto i = _ordinals.find(del);

	assert(i != _ordinals.end());
	return i->second;
}

void LinearView::delegateModified(void)
{
	Delegate *del = qobject_cast<Delegate*>(sender());

	assert(del);
	auto i = std::find_if(_delegates.begin(),_delegates.end(),[&](const std::pair<decltype(_delegates)::interval_type,std::shared_ptr<Delegate>> &p)
			{ return p.second.get() == del; });
	auto a = _visibleRows.lower_bound(boost::icl::first(i->first));
	auto b = _visibleRows.upper_bound(boost::icl::last(i->first));

	while(a != b)
	{
		del->deleteRow(a->second);
		a = _visibleRows.erase(a);
	}

	rowHeightChanged();
}



void LinearView::rowHeightChanged(void)
{
	QQuickItem *prev = 0;
	Delegate *del = qobject_cast<Delegate*>(sender());

	assert(del);
	auto i = std::find_if(_delegates.begin(),_delegates.end(),[&](const std::pair<decltype(_delegates)::interval_type,std::shared_ptr<Delegate>> &p)
			{ return p.second.get() == del; });
	auto a = _visibleRows.lower_bound(boost::icl::first(i->first));
	auto b = _visibleRows.upper_bound(boost::icl::last(i->first));

	std::for_each(a,b,[&](const std::pair<rowIndex,QQuickItem*> &p)
	{
		if(prev)
			p.second->setY(prev->y() + prev->height());
		prev = p.second;
	});
	scrollViewport(0);
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
		auto i = std::find_if(_visibleRows.begin(),_visibleRows.end(),[&](const std::pair<rowIndex,QQuickItem *> &j)
			{ return QRectF(QPointF(j.second->x(),j.second->y()),j.second->boundingRect().size()).contains(ptn); });

		if(i != _visibleRows.end())
		{
			QQuickItem *itm = i->second;

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
		auto i = std::find_if(_visibleRows.begin(),_visibleRows.end(),[&](const std::pair<rowIndex,QQuickItem *> &j)
			{ return QRectF(QPointF(j.second->x(),j.second->y()),j.second->boundingRect().size()).contains(ptn); });

		if(i != _visibleRows.end())
		{
			QQuickItem *itm = i->second;

			QVariant ret;
			QMetaObject::invokeMethod(itm,"mousePressed",Q_RETURN_ARG(QVariant,ret),Q_ARG(QVariant,ptn.x() - itm->x()),Q_ARG(QVariant,ptn.y() - itm->y()));
			event->accept();
		}
	}
}

void LinearView::geometryChanged(const QRectF&, const QRectF&)
{
	scrollViewport();
}

void LinearView::scrollViewport(float delta)
{
	QRectF bb(x(),y(),width(),height());
	rowIndex firstVisibleRowIndex = std::numeric_limits<rowIndex>::max();
	auto j = _visibleRows.begin();

	// move rows and delete those out of sight
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

	// insert empty space w/ new rows
	if(firstVisibleRowIndex == std::numeric_limits<rowIndex>::max())
	{
		insertRows(y(),0,false);
	}
	else
	{
		if(firstVisibleRowIndex > 0 && _visibleRows[firstVisibleRowIndex]->y() > bb.top())
			insertRows(_visibleRows[firstVisibleRowIndex]->y(),firstVisibleRowIndex-1,true);

		auto last = std::prev(_visibleRows.end());
		if(last->second->y() + last->second->height() < bb.bottom())
			insertRows(last->second->y() + last->second->height(),last->first + 1,false);
	}

	if(_visibleRows.size())
	{
		// prevent scrolling above the first row
		if(_visibleRows.begin()->first == 0 && _visibleRows.begin()->second->y() > 0)
			return scrollViewport(-_visibleRows.begin()->second->y());

		// prevent scrolling below the last row
		auto last = std::prev(_visibleRows.end());
		float bot = last->second->y() + last->second->height();
		if((_visibleRows.begin()->first != 0 || _visibleRows.begin()->second->y() < 0) && last->first == boost::icl::length(_delegates) - 1 && bot < bb.bottom())
		{
			float bot_adj = bb.bottom() - bot;
			float top_adj = (_visibleRows.begin()->first == 0 ? -_visibleRows.begin()->second->y() : std::numeric_limits<float>::max());

			return scrollViewport(std::min<float>(bot_adj,top_adj));
		}
	}
}

void LinearView::insertRows(float y, rowIndex gri, bool up)
{
	QRectF bb(x(),QQuickItem::y(),width(),height());

	while(gri >= 0 && gri < boost::icl::length(_delegates) && bb.contains(QPointF(bb.x(),y)))
	{
		auto j = _delegates.find(gri);

		assert(j != _delegates.end());
		{
			rowIndex l = gri - boost::icl::first(j->first);
			QQuickItem *itm = j->second->createRow(l);

			if(itm)
			{
				itm->setParentItem(this);
				itm->setX(0);
				itm->setY(y + (up ? -itm->height() : 0));

				connect(itm,SIGNAL(heightChanged()),this,SLOT(rowHeightChanged()));
				assert(_visibleRows.insert(std::make_pair(gri,itm)).second);

				y += (up ? -itm->height() : itm->height());
			}

			gri += (up ? -1 : 1);
		}
	}
}

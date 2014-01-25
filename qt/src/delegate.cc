#include <cassert>

#include "delegate.hh"
#include <panopticon/digraph.hh>

Delegate::Delegate(po::region_wloc r, QObject *p)
: QObject(p), _region(r)
{}

Delegate::~Delegate(void) {}

po::region_wloc Delegate::region(void) const { return _region; }

BinaryDelegate::BinaryDelegate(po::region_wloc r, unsigned int w, QQmlEngine *e, QQuickItem *p)
: Delegate(r,p), _width(w), _engine(e),
	_rowComponent(_engine,QUrl("qrc:/Binary.qml")),
	_titleComponent(_engine,QUrl("qrc:/BinaryTitle.qml")),
	_cursorComponent(_engine,QUrl("qrc:/Cursor.qml")),
	_overlays(), _visibleRows(),
	_cursor(boost::none), _cursorOverlay(0), _collapsed(false)
{
	assert(w);
	qDebug() << _rowComponent.errors();
	qDebug() << _titleComponent.errors();
	qDebug() << _cursorComponent.errors();
}

BinaryDelegate::~BinaryDelegate(void) {}

unsigned int BinaryDelegate::rowCount(void) const
{
	if(_collapsed)
		return 1;
	else
	{
		size_t l = region()->size();
		return l / _width + !!(l % _width) + 1;
	}
}

QQuickItem *BinaryDelegate::createRow(unsigned int l)
{
	unsigned int i = 0, w = (l == rowCount() - 1 && region()->size() % _width ? region()->size() % _width : _width);
	QVariantList _data;
	QQuickItem *ret = 0;

	assert(l < rowCount());

	if(l > 0)
	{
		while(i < w)
		{
			_data.append(QVariant::fromValue(QString("??")));
			i++;
		}

		assert(_data.size());

		ret = qobject_cast<QQuickItem*>(_rowComponent.create());
		assert(ret);

		ret->setProperty("address",QVariant((l-1) * _width));
		ret->setProperty("row",QVariant(l));
		ret->setProperty("payload",QVariant(_data));

		connect(ret,SIGNAL(elementEntered(int,int)),this,SLOT(elementEntered(int,int)));
		connect(ret,SIGNAL(elementClicked(int,int)),this,SLOT(elementClicked(int,int)));
	}
	else
	{
		ret = qobject_cast<QQuickItem*>(_titleComponent.create());

		assert(ret);
		ret->setProperty("title",QString(QString::fromStdString(region()->name())));
		connect(ret,SIGNAL(collapse()),this,SLOT(collapseRows()));
	}

	ret->setParent(this);
	ret->setParentItem(qobject_cast<QQuickItem*>(parent()));

	assert(_visibleRows.insert(std::make_pair(l,ret)).second);
	updateOverlays(ElementSelection(l,0,l,_width-1));

	return ret;
}

void BinaryDelegate::deleteRow(QQuickItem *i)
{
	assert(i);
	auto j = std::find_if(_visibleRows.begin(),_visibleRows.end(),[&](std::pair<int,QQuickItem*> p) { return p.second == i; });

	assert(j != _visibleRows.end());

	int l = j->first;

	_visibleRows.erase(j);
	updateOverlays(ElementSelection(l,0,l,_width-1));
	i->deleteLater();
}

QQuickItem *BinaryDelegate::createOverlay(const ElementSelection &sel)
{
	QQuickItem *ov = qobject_cast<QQuickItem*>(_cursorComponent.create());

	assert(ov);
	ov->setParent(this);
	ov->setParentItem(qobject_cast<QQuickItem*>(parent()));
	ov->setVisible(false);
	assert(QQmlProperty::write(ov,"cursor",QVariant::fromValue<QObject*>(new ElementSelectionObject(sel,ov))));

	_overlays.insert(std::make_pair(sel,ov));
	updateOverlays(sel);

	return ov;
}

void BinaryDelegate::deleteOverlay(QQuickItem *ov)
{
	assert(ov);
	auto i = std::find_if(_overlays.begin(),_overlays.end(),[&](const std::pair<ElementSelection,QQuickItem*> &p) { return p.second == ov; });
	assert(i != _overlays.end());
	auto key = i->first;

	_overlays.erase(i);
	updateOverlays(key);
	ov->deleteLater();
}

void BinaryDelegate::collapseRows(void)
{
	_collapsed = !_collapsed;
	emit modified();
}

boost::optional<std::pair<QQuickItem*,QQuickItem*>> BinaryDelegate::attachableRows(const ElementSelection &sel)
{
	auto fi = _visibleRows.lower_bound(sel.firstLine());
	auto li = _visibleRows.lower_bound(sel.lastLine());

	if(fi != _visibleRows.end() && fi->first >= sel.firstLine() && fi->first <= sel.lastLine())
	{
		if(li == _visibleRows.end() || li->first > sel.lastLine())
			--li;
		if(li->first >= sel.firstLine() && li->first <= sel.lastLine())
		{
			return std::make_pair(fi->second,li->second);
		}
	}

	return boost::none;
}

void BinaryDelegate::updateOverlays(const ElementSelection &sel)
{
	for(auto i: _overlays)
	{
		if(!i.first.disjoint(sel))
		{
			auto rows = attachableRows(i.first);

			if(rows)
			{
				QVariant ret, first = QVariant::fromValue(rows->first), last = QVariant::fromValue(rows->second);
				assert(QQmlProperty::write(i.second,"firstRow",first));
				assert(QQmlProperty::write(i.second,"lastRow",last));
				i.second->setVisible(true);

				continue;
			}

			i.second->setVisible(false);
		}
	}
}

void BinaryDelegate::elementClicked(int col, int row)
{
	emit selected((row - 1) * _width + col,true);
}

void BinaryDelegate::elementEntered(int col, int row)
{
	emit selected((row - 1) * _width + col,false);
}

void BinaryDelegate::select(boost::optional<std::pair<po::offset,po::offset>> p)
{
	if(p)
		setCursor(ElementSelection(1 + std::trunc(p->first / _width),p->first % _width,1 + std::trunc(p->second / _width),p->second % _width));
	else
		setCursor(boost::none);
}

void BinaryDelegate::setCursor(const boost::optional<ElementSelection> &sel)
{
	if(!_cursorOverlay)
	{
		_cursorOverlay = createOverlay(ElementSelection(0,0,0,0));
		_cursorOverlay->setVisible(false);
	}

	if(sel != _cursor)
	{
		auto i = std::find_if(_overlays.begin(),_overlays.end(),[&](const std::pair<ElementSelection,QQuickItem*> &p) { return p.second == _cursorOverlay; });

		if(i != _overlays.end())
			_overlays.erase(i);

		if((_cursor = sel))
		{
			auto p = attachableRows(*_cursor);

			assert(QQmlProperty::write(_cursorOverlay,"cursor",QVariant::fromValue<QObject*>(new ElementSelectionObject(*sel,_cursorOverlay))));

			if(p)
			{
				assert(QQmlProperty::write(_cursorOverlay,"firstRow",QVariant::fromValue<QObject*>(p->first)));
				assert(QQmlProperty::write(_cursorOverlay,"lastRow",QVariant::fromValue<QObject*>(p->second)));
			}

			_overlays.insert(std::make_pair(*_cursor,_cursorOverlay));
			_cursorOverlay->setVisible(true);
		}
		else
		{
			_cursorOverlay->setVisible(false);
			assert(QQmlProperty::write(_cursorOverlay,"cursor",QVariant()));
		}
	}
}

#include <cassert>

#include "delegate.hh"
#include <panopticon/digraph.hh>

Delegate::Delegate(po::region_wloc r, QObject *p)
: QObject(p), _region(r)
{}

Delegate::~Delegate(void) {}

po::region_wloc Delegate::region(void) const { return _region; }

TestDelegate::TestDelegate(po::region_wloc r, unsigned int w, QQmlEngine *e, QQuickItem *p)
: Delegate(r,p), m_width(w), m_engine(e),
	m_rowComponent(m_engine,QUrl("qrc:/Test.qml")),
	m_headComponent(m_engine,QUrl("qrc:/Block.qml")),
	m_cursorComponent(m_engine,QUrl("qrc:/Cursor.qml")),
	m_overlays(), m_visibleRows(),
	m_cursor(boost::none), m_cursorOverlay(0), m_collapsed(false)
{
	assert(w);
	qDebug() << m_rowComponent.errors();
	qDebug() << m_headComponent.errors();
	qDebug() << m_cursorComponent.errors();
}

TestDelegate::~TestDelegate(void) {}

unsigned int TestDelegate::rowCount(void) const
{
	if(m_collapsed)
		return 0;
	else
	{
		size_t l = region()->size();
		return l / m_width + !!(l % m_width);
	}
}

QQuickItem *TestDelegate::createRow(unsigned int l)
{
	unsigned int i = 0, w = (l == rowCount() - 1 && region()->size() % m_width ? region()->size() % m_width : m_width);
	QVariantList _data;

	assert(l < rowCount());

	while(i < w)
	{
		_data.append(QVariant::fromValue(QString("??")));
		i++;
	}

	assert(_data.size());

	auto ret = qobject_cast<QQuickItem*>(m_rowComponent.create(/*ctx*/));
	assert(ret);

	ret->setProperty("address",QVariant(l * m_width));
	ret->setProperty("row",QVariant(l));
	ret->setProperty("payload",QVariant(_data));

	connect(ret,SIGNAL(elementEntered(int,int)),this,SLOT(elementEntered(int,int)));
	connect(ret,SIGNAL(elementClicked(int,int)),this,SLOT(elementClicked(int,int)));
	ret->setParent(this);
	ret->setParentItem(qobject_cast<QQuickItem*>(parent()));

	assert(m_visibleRows.insert(std::make_pair(l,ret)).second);
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

	assert(ret);
	ret->setProperty("name",QString(QString::fromStdString(region()->name())));
	ret->setParent(this);

	connect(ret,SIGNAL(collapse()),this,SLOT(collapseRows()));

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
	assert(QQmlProperty::write(ov,"cursor",QVariant::fromValue<QObject*>(new ElementSelectionObject(sel,ov))));

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

boost::optional<std::pair<QQuickItem*,QQuickItem*>> TestDelegate::attachableRows(const ElementSelection &sel)
{
	auto fi = m_visibleRows.lower_bound(sel.firstLine());
	auto li = m_visibleRows.lower_bound(sel.lastLine());

	if(fi != m_visibleRows.end() && fi->first >= sel.firstLine() && fi->first <= sel.lastLine())
	{
		if(li == m_visibleRows.end() || li->first > sel.lastLine())
			--li;
		if(li->first >= sel.firstLine() && li->first <= sel.lastLine())
		{
			return std::make_pair(fi->second,li->second);
		}
	}

	return boost::none;
}

void TestDelegate::updateOverlays(const ElementSelection &sel)
{
	for(auto i: m_overlays)
	{
		if(!i.first.disjoint(sel))
		{
			auto rows = attachableRows(i.first);

			if(rows)
			{
				QVariant ret, first = QVariant::fromValue(rows->first), last = QVariant::fromValue(rows->second);
				assert(QQmlProperty::write(i.second,"firstRow",first));
				assert(QQmlProperty::write(i.second,"lastRow",last));

				//QMetaObject::invokeMethod(i.second,"attach",Q_RETURN_ARG(QVariant,ret),Q_ARG(QVariant,first),Q_ARG(QVariant,last));
				i.second->setVisible(true);

				continue;
			}

			i.second->setVisible(false);
		}
	}
}

void TestDelegate::elementClicked(int col, int row)
{
	emit selected(row * m_width + col,true);
}

void TestDelegate::elementEntered(int col, int row)
{
	emit selected(row * m_width + col,false);
}

void TestDelegate::collapseRows(void)
{
	m_collapsed = !m_collapsed;
	emit modified();
}

void TestDelegate::select(boost::optional<std::pair<po::offset,po::offset>> p)
{
	if(p)
		setCursor(ElementSelection(std::trunc(p->first / m_width),p->first % m_width,std::trunc(p->second / m_width),p->second % m_width));
	else
		setCursor(boost::none);
}

void TestDelegate::setCursor(const boost::optional<ElementSelection> &sel)
{
	if(!m_cursorOverlay)
	{
		m_cursorOverlay = createOverlay(ElementSelection(0,0,0,0));
		m_cursorOverlay->setVisible(false);
	}

	if(sel != m_cursor)
	{
		auto i = std::find_if(m_overlays.begin(),m_overlays.end(),[&](const std::pair<ElementSelection,QQuickItem*> &p) { return p.second == m_cursorOverlay; });

		if(i != m_overlays.end())
			m_overlays.erase(i);

		if((m_cursor = sel))
		{
			auto p = attachableRows(*m_cursor);

			assert(QQmlProperty::write(m_cursorOverlay,"cursor",QVariant::fromValue<QObject*>(new ElementSelectionObject(*sel,m_cursorOverlay))));

			if(p)
			{
				assert(QQmlProperty::write(m_cursorOverlay,"firstRow",QVariant::fromValue<QObject*>(p->first)));
				assert(QQmlProperty::write(m_cursorOverlay,"lastRow",QVariant::fromValue<QObject*>(p->second)));
			}

			m_overlays.insert(std::make_pair(*m_cursor,m_cursorOverlay));
			m_cursorOverlay->setVisible(true);
		}
		else
		{
			m_cursorOverlay->setVisible(false);
			assert(QQmlProperty::write(m_cursorOverlay,"cursor",QVariant()));
		}
	}
}

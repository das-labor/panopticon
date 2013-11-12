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
: QQuickItem(parent), m_engine(), m_component(&m_engine), m_context(), m_session(nullptr), m_viewport(), m_viewportIndex(0)
{
	setFlags(QQuickItem::ItemHasContents);
	setAcceptedMouseButtons(Qt::LeftButton);

	m_engine.rootContext()->setContextProperty("linearViewContext",&m_context);
	m_component.loadUrl(QUrl("qrc:/Element.qml"));

	addRows();
	setClip(true);
}

LinearView::~LinearView(void)
{}

Session *LinearView::session(void)
{
	return (m_session ? *m_session : nullptr);
}

void LinearView::setSession(Session *s)
{
	if(s != m_session)
	{
		m_session = s;

		int id = 0;
		int i = 0;

		m_currentView.clear();
		m_hidden.clear();
		//m_firstRow = m_lastRow = m_firstColumn = m_lastColumn = -1;

		for(auto p: po::projection((*m_session)->graph().get_node(po::root((*m_session)->graph())),(*m_session)->graph()))
		{
			QSharedPointer<Delegate> delegate(new TestDelegate(p.second,p.first,10,&m_engine,this));
			auto len = delegate->rows();

			m_currentView.add(std::make_pair(decltype(m_currentView)::interval_type::right_open(i,i + 1),LinearViewBlock(LinearViewBlock::Header,delegate,id)));
			m_currentView.add(std::make_pair(decltype(m_currentView)::interval_type::right_open(i + 1,i + 1 + len),LinearViewBlock(LinearViewBlock::Data,delegate,id)));

			connect(delegate.data(),SIGNAL(modified(const boost::optional<ElementSelection> &)),this,SLOT(delegateModified(const boost::optional<ElementSelection> &)));

			i += len + 1;
			id += 1;
		}

		emit sessionChanged();
	}
}

QQuickItem *LinearView::data(int idx)
{
	auto iter = m_currentView.find(idx);
	QQuickItem *ret = nullptr;

	if(iter == m_currentView.end())
		return nullptr;

	if(iter->second.type == LinearViewBlock::Data)
	{
		ret = iter->second.delegate->data(idx - boost::icl::first(iter->first));
	}
	else
	{
		/*ret = new Header(QString::fromStdString(iter->second.delegate->space().name),
				 						 iter->second.type == LinearViewBlock::HeaderCollapsed,
										 iter->second.id);*/
		auto ctx = new QQmlContext(&m_engine);

		ctx->setContextProperty("index",QVariant::fromValue(idx));
		ret = qobject_cast<QQuickItem*>(m_component.create(ctx));
		ctx->setParent(ret);
	}

	if(ret)
	{
		ret->setParentItem(this);
		ret->setX(0);
		connect(ret,SIGNAL(heightChanged()),this,SLOT(test()));
	}

	return ret;
}

void LinearView::addRows(bool up)
{
	assert(!(m_viewport.empty() && up));

	qreal yy = (m_viewport.empty() ? 0 : (up ? m_viewport.front()->y() : m_viewport.back()->y() + m_viewport.back()->height()));
	int idx = (m_viewport.empty() ? 0 : m_viewportIndex + (up ? -1 : m_viewport.size()));

	while(yy >= y() && yy < y() + height())
	{
		QQuickItem *itm = data(idx);

		if(!itm)
			return;

		if(up)
		{
			m_viewport.push_front(itm);
			--idx;
			--m_viewportIndex;
			itm->setY(yy - itm->height());
			yy += 1 - itm->height();
		}
		else
		{
			m_viewport.push_back(itm);
			++idx;
			itm->setY(yy);
			yy += itm->height();
		}
	}
}

void LinearView::test(void)
{
	QQuickItem *prev = 0;
	std::for_each(std::find(m_viewport.begin(),m_viewport.end(),sender()),m_viewport.end(),[&](QQuickItem *itm)
	{
		if(prev)
			itm->setY(prev->y() + prev->height());
		prev = itm;
	});
}

void LinearView::delegateModified(const boost::optional<ElementSelection> &sel)
{
	Delegate *delegate = qobject_cast<Delegate*>(sender());
	assert(delegate);

	// XXX: rename
	// XXX: find delegate Data Block in m_availableBlocks
	// XXX: replace rows in m_visibleRows if needed

void LinearView::wheelEvent(QWheelEvent *event)
{
	scrollViewport(event->angleDelta().y() / 8);
}

void LinearView::mouseMoveEvent(QMouseEvent *event)
{
	QPointF ptn = event->localPos();
	QQuickItem *itm = qobject_cast<QQuickItem*>(childAt(ptn.x(),ptn.y()));

	if(event->buttons() & Qt::LeftButton && itm && std::find(m_viewport.begin(),m_viewport.end(),itm) != m_viewport.end())
	{
		QVariant ret;
		QMetaObject::invokeMethod(itm,"mouseMoved",Q_RETURN_ARG(QVariant,ret),Q_ARG(QVariant,ptn.x() - itm->x()),Q_ARG(QVariant,ptn.y() - itm->y()));
		event->accept();
	}
}

void LinearView::mousePressEvent(QMouseEvent *event)
{
	QPointF ptn = event->localPos();
	QQuickItem *itm = qobject_cast<QQuickItem*>(childAt(ptn.x(),ptn.y()));

	if(event->buttons() & Qt::LeftButton && itm && std::find(m_viewport.begin(),m_viewport.end(),itm) != m_viewport.end())
	{
		QVariant ret;
		QMetaObject::invokeMethod(itm,"mousePressed",Q_RETURN_ARG(QVariant,ret),Q_ARG(QVariant,ptn.x() - itm->x()),Q_ARG(QVariant,ptn.y() - itm->y()));
		event->accept();
	}
}

void LinearView::geometryChanged(const QRectF&, const QRectF&)
{
	scrollViewport(0);
}

unsigned int LinearView::rows(void) const
{
	return boost::icl::length(m_currentView);
}

void LinearView::scrollViewport(qreal d)
{
	QRectF bb(x(),y(),width(),height());

	if(d &&
		 !m_viewportIndex &&
		 m_viewport.size() >= rows() &&
		 m_viewport.front()->y() >= bb.top() &&
		 m_viewport.back()->y() + m_viewport.back()->height() <= bb.bottom())
		return;

	if(d)
	{
		// move elements
		for(auto i: m_viewport)
			i->setY(i->y() + d);
	}

	// delete elements out of sight
	auto i = m_viewport.begin();
	while(i != m_viewport.end())
	{
		if((bb & QRectF((*i)->x(),(*i)->y(),(*i)->width(),(*i)->height())).isNull())
		{
			delete *i;

			if(i == m_viewport.begin())
				++m_viewportIndex;
			i = m_viewport.erase(i);
		}
		else
		{
			++i;
		}
	}

	// add new elements
	if(m_viewport.empty())
		addRows(false);
	else
	{
		if(m_viewport.front()->y() > y())
			addRows(true);
		if(m_viewport.back()->y() < y() + height())
			addRows(false);

		// Make sure we don't scroll past the first/last element
		if(d)
		{
			if(!m_viewportIndex)
			{
				qreal delta = m_viewport.front()->y() - bb.top();

				if(delta > 0)
				{
					scrollViewport(-delta);
					return;
				}
			}

			if(m_viewportIndex + m_viewport.size() >= rows())
			{
				qreal delta = bb.bottom() - (m_viewport.back()->y() + m_viewport.back()->height());

				if(delta > 0)
					scrollViewport(delta);
			}
		}
	}
}

#include <QApplication>
#include <QQuickView>
#include <QQuickItem>
#include <QAbstractListModel>
#include <QDebug>
#include <QQmlApplicationEngine>
#include <QQmlComponent>
#include <QQmlContext>
#include <limits>

#include "linearscene.hh"

Header::Header(void) : QObject(), m_name(""), m_collapsed(false), m_id(-1) {}
Header::Header(const QString &h, bool col, int id) : QObject(), m_name(h), m_collapsed(col), m_id(id) {}
Header::~Header(void) {}

QString Header::name(void) const { return m_name; }
bool Header::collapsed(void) const { return m_collapsed; }
int Header::id(void) const { return m_id; }

LinearSceneModel::LinearSceneBlock::LinearSceneBlock(void)
: type(Data), delegate(), id(-1)
{}

LinearSceneModel::LinearSceneBlock::LinearSceneBlock(LinearSceneModel::LinearSceneBlock::Type t, QSharedPointer<Delegate> d, int i)
: type(t), delegate(d), id(i)
{}

LinearSceneModel::LinearSceneBlock::LinearSceneBlock(const LinearSceneModel::LinearSceneBlock &o)
: type(o.type), delegate(o.delegate), id(o.id)
{}

bool LinearSceneModel::LinearSceneBlock::operator==(const LinearSceneModel::LinearSceneBlock &r) const
{
	return type == r.type && delegate == r.delegate && id == r.id;
}

LinearSceneModel::LinearSceneBlock &LinearSceneModel::LinearSceneBlock::operator+=(const LinearSceneModel::LinearSceneBlock &r)
{
	return *this;
}

LinearSceneModel::LinearSceneModel(void)
: QAbstractListModel()//, m_firstRow(0), m_lastRow(1), m_firstColumn(0), m_lastColumn(3), m_currentView()
{}

LinearSceneModel::~LinearSceneModel(void) {}

int LinearSceneModel::rowCount(const QModelIndex &parent) const
{
	if(!parent.isValid())
		return boost::icl::length(m_currentView);
	else
		return 0;
}

QVariant LinearSceneModel::data(const QModelIndex &index, int role) const
{
	auto iter = m_currentView.find(index.row());

	if(iter == m_currentView.end())
		return QVariant();

	if(iter->second.type == LinearSceneBlock::Data)
	{
		switch(role)
		{
			case Qt::DisplayRole: return QString("-- Row --");
			case Qt::UserRole: return iter->second.delegate->line(index.row() - boost::icl::first(iter->first));
			case Qt::UserRole + 1: return "qrc:/Element.qml";
			case Qt::UserRole + 2: return QString("%1 + %2").arg(QString::fromStdString(iter->second.delegate->space().name))
														 													.arg(index.row() - boost::icl::first(iter->first));
			default: return QVariant();
		}
	}
	else
	{
		switch(role)
		{
			case Qt::DisplayRole: return QString("-- Block head --");
			case Qt::UserRole: return QVariant::fromValue(new Header(QString::fromStdString(iter->second.delegate->space().name),
															 																 iter->second.type == LinearSceneBlock::HeaderCollapsed,
																															 iter->second.id));
			case Qt::UserRole + 1: return "qrc:/Block.qml";
			default: return QVariant();
		}
	}
}

QHash<int, QByteArray> LinearSceneModel::roleNames(void) const
{
	QHash<int, QByteArray> ret;

	ret.insert(Qt::DisplayRole,QByteArray("display"));
	ret.insert(Qt::UserRole,QByteArray("payload"));
	ret.insert(Qt::UserRole+1,QByteArray("delegate"));
	ret.insert(Qt::UserRole+2,QByteArray("offset"));
	return ret;
}

void LinearSceneModel::setSelect(int anchorRow, int anchorCol, int cursorRow, int cursorCol)
{
	if(anchorRow >= 0 && anchorCol >= 0 && cursorRow >= 0 && cursorCol >= 0)
	{
		ElementSelection sel(anchorRow,anchorCol,cursorRow,cursorCol);
		auto iv = decltype(m_currentView)::interval_type(sel.firstLine(),sel.lastLine() + 1);

		for(auto q: m_currentView)
		{
			auto del = q.second.delegate;

			if(boost::icl::intersects(q.first,iv) && q.second.type == LinearSceneBlock::Data)
			{
				int base = boost::icl::first(q.first);
				int last = del->lines() - 1;

				assert(last >= 0);
				del->setCursor(ElementSelection(std::max<int>(sel.firstLine() - base,0),sel.firstLine() >= static_cast<unsigned int>(base) ? sel.firstColumn() : 0,
																				std::min<int>(sel.lastLine() - base,last),sel.lastLine() <= static_cast<unsigned int>(base + last) ? sel.lastColumn() : del->width(last) - 1));
			}
			else
			{
				del->setCursor(boost::none);
			}
		}
	}
	else
	{
		for(auto q: m_currentView)
			q.second.delegate->setCursor(boost::none);
	}
}

void LinearSceneModel::setVisibility(int blkId, bool hide)
{
	int nextBlk = 0;
	auto newBlks = decltype(m_currentView)();
	int modFirst = 0, modLast = 0;
	int changedBlk = 0;

	for(auto p: m_currentView)
	{
		if(p.second.id == blkId)
		{
			if(p.second.type == LinearSceneBlock::Header || p.second.type == LinearSceneBlock::HeaderCollapsed)
			{
				// Block (header)
				newBlks.add(std::make_pair(decltype(m_currentView)::interval_type::right_open(nextBlk,nextBlk + 1),
																	 LinearSceneBlock(hide ? LinearSceneBlock::HeaderCollapsed : LinearSceneBlock::Header,p.second.delegate,p.second.id)));
				changedBlk = nextBlk;
				nextBlk += 1;

				// Show Rows
				if(!hide)
				{
					auto i = m_hidden.find(p.second.id);

					assert(i != m_hidden.end() && i->second.type == LinearSceneBlock::Data);

					modFirst = nextBlk;
					modLast = nextBlk + i->second.delegate->lines() - 1;
					newBlks.add(std::make_pair(decltype(m_currentView)::interval_type::right_open(nextBlk,nextBlk + i->second.delegate->lines()),i->second));
					nextBlk += i->second.delegate->lines();
					m_hidden.erase(i);
				}
			}
			else
			{
				assert(p.second.type == LinearSceneBlock::Data);

				// Move Row into m_hidden
				if(hide)
				{
					modFirst = nextBlk;
					modLast = nextBlk + boost::icl::length(p.first) - 1;
					m_hidden.insert(std::make_pair(p.second.id,p.second));
				}
			}
		}
		else
		{
			newBlks.add(std::make_pair(decltype(m_currentView)::interval_type::right_open(nextBlk,nextBlk + boost::icl::length(p.first)),p.second));
			nextBlk += boost::icl::length(p.first);
		}
	}

	//m_firstRow = m_lastRow = m_firstColumn = m_lastColumn = -1;

	if(hide)
		beginRemoveRows(QModelIndex(),modFirst,modLast);
	else
		beginInsertRows(QModelIndex(),modFirst,modLast);

	m_currentView = newBlks;

	if(hide)
		endRemoveRows();
	else
		endInsertRows();

	emit dataChanged(index(changedBlk),index(changedBlk));
}

/*bool LinearSceneModel::selected(int row, int col) const
{
	return (m_firstRow < row && m_lastRow > row) ||
				 (m_firstRow == row && m_lastRow == row && m_firstColumn <= col && m_lastColumn >= col) ||
				 (m_firstRow == row && m_lastRow != row && m_firstColumn <= col) ||
				 (m_lastRow == row && m_firstRow != row && m_lastColumn >= col);
}*/

void LinearSceneModel::setGraph(const po::graph<po::address_space,po::rrange> &g)
{
	int id = 0;
	int i = 0;

	beginResetModel();

	m_currentView.clear();
	m_hidden.clear();
	//m_firstRow = m_lastRow = m_firstColumn = m_lastColumn = -1;

	for(auto p: po::projection(g.get_node(po::root(g)),g))
	{
		QSharedPointer<Delegate> delegate(new TestDelegate(p.second,p.first,10));
		auto len = delegate->lines();

		m_currentView.add(std::make_pair(decltype(m_currentView)::interval_type::right_open(i,i + 1),LinearSceneBlock(LinearSceneBlock::Header,delegate,id)));
		m_currentView.add(std::make_pair(decltype(m_currentView)::interval_type::right_open(i + 1,i + 1 + len),LinearSceneBlock(LinearSceneBlock::Data,delegate,id)));

		connect(delegate.data(),SIGNAL(modified(const boost::optional<ElementSelection> &)),this,SLOT(delegateModified(const boost::optional<ElementSelection> &)));

		i += len + 1;
		id += 1;
	}
	endResetModel();
}

void LinearSceneModel::delegateModified(const boost::optional<ElementSelection> &sel)
{
	Delegate *del = dynamic_cast<Delegate*>(sender());

	assert(del && sel);
	for(auto p: m_currentView)
	{
		if(p.second.delegate.data() == del && p.second.type == LinearSceneBlock::Data)
		{
			int b = boost::icl::first(p.first);

			emit dataChanged(index(b + sel->firstLine(),0),index(b + sel->lastLine(),0));
			return;
		}
	}
}

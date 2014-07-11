#include "session.hh"

using namespace po;

const int columnWidth = 16;

LinearModel::LinearModel(dbase_loc db, QObject *p)
: QAbstractListModel(p), _dbase(db), _projection(po::projection(_dbase->data))
{}

int LinearModel::rowCount(const QModelIndex& parent) const
{
	if(parent == QModelIndex())
	{
		po::offset tot = std::accumulate(_projection.begin(),_projection.end(),po::offset(0),[&](po::offset a, const std::pair<bound,region_wloc>& p)
			{ return a + boost::icl::size(p.first); });

		return (tot % columnWidth) ? tot / columnWidth + 1 : tot / columnWidth;
	}
	else
		return 0;
}

QVariant LinearModel::data(const QModelIndex& idx, int role) const
{
	if(role != Qt::DisplayRole)
	{
		return QVariant();
	}
	else
	{
		const po::offset t = idx.row() * columnWidth;
		po::offset o = 0;
		QStringList hex, text, comments;

		for(auto p: _projection)
		{
			if(o <= t && o + boost::icl::size(p.first) > t)
			{
				slab sl = p.second.lock()->read();
				auto i = boost::begin(sl) + (t - o);
				auto j = boost::begin(sl) + std::min<po::offset>((t - o) + columnWidth,boost::icl::size(p.first));
				auto k = _dbase->comments.lower_bound(ref{p.second->name(),o});

				while(k != _dbase->comments.end() &&
							k->first.reg == p.second->name() &&
							k->first.off < o + t)
				{
					comments.append(QString::fromStdString(*(k->second)));
					++k;
				}

				for(po::tryte s: iters(std::make_pair(i,j)))
				{
					if(s)
					{
						hex += QString("'%1'").arg(static_cast<unsigned int>(*s),2,16,QChar('0'));

						if(isprint(*s))
						{
							if(*s == '\\' || *s == '\'')
								text += QString("'\\%1'").arg(QChar(*s));
							else
								text += QString("'%1'").arg(QChar(*s));
						}
						else
							text += QString("' '");
					}
					else
					{
						hex += "'?""?'";
						text += "'?""?'";
					}
				}

				while(hex.size() < columnWidth)
				{
					hex += QString("''");
					text += QString("''");
				}
			}

			o += boost::icl::size(p.first);
		}

		ensure(text.size() == hex.size() && text.size());
		return QString("{ 'address': '0x%1', 'hex': %2, 'text': %3, 'comment': '%4' }")
						.arg(idx.row() * columnWidth,0,16)
						.arg("[" + hex.join(",") + "]")
						.arg("[" + text.join(",") + "]")
						.arg(comments.join("\n"));
	}
}

void LinearModel::postComment(int row, QString c)
{
	po::offset o = 0, t = row * columnWidth;
	for(auto p: _projection)
	{
		if(o <= t && o + boost::icl::size(p.first) > t)
		{
			auto k = _dbase->comments.lower_bound(ref{p.second->name(),o});

			while(k != _dbase->comments.end() &&
						k->first.reg == p.second->name() &&
						k->first.off < o + columnWidth)
			{
				k = _dbase.write().comments.erase(k);
			}

			_dbase.write().comments.emplace(ref{p.second->name(),o},comment_loc(new std::string(c.toStdString())));
			dataChanged(createIndex(row,0),createIndex(row,0));
			return;
		}
	}

	ensure(false);
}

Session::Session(po::session sess, QObject *p)
: QObject(p), _session(sess), _linear(new LinearModel(sess.dbase,this))
{}

Session::~Session(void)
{}

Session* Session::open(QString s)
{
	return new Session(po::open(s.toStdString()));
}

Session* Session::create(QString s)
{
	return new Session(po::raw(s.toStdString()));
}

void Session::postComment(int r, QString c)
{
	qDebug() << "post" << c << "in" << r;
	_linear->postComment(r,c);
}

#include "session.hh"
#include <panopticon/program.hh>

using namespace po;
using namespace boost::icl;

const int columnWidth = 16;

namespace boost
{
	row_t& operator+=(row_t& a, const row_t& b)
	{
		return a;
	}
}

LinearModel::LinearModel(dbase_loc db, QObject *p)
: QAbstractListModel(p), _dbase(db), _projection(po::projection(_dbase->data))
{
	int row = 0;

	for(auto p: _projection)
	{
		po::offset o = p.first.lower();

		while(o < p.first.upper())
		{
			auto r = po::next_record(ref{p.second->name(),o},_dbase);

			struct add_vis : public boost::static_visitor<>
			{
				add_vis(int &ro, split_interval_map<int,row_t>& r, region_loc re) : row(ro), _rows(r), reg(re) {}

				void operator()(bblock_loc bb) const
				{
					interval<int>::type iv(row,row + bb->mnemonics().size());
					_rows += std::make_pair(iv,row_t(reg,bb));
					row += bb->mnemonics().size();
				}

				void operator()(struct_loc s) const
				{
					interval<int>::type iv(row,row + s->fields.size());
					_rows += std::make_pair(iv,row_t(reg,s));
					row += s->fields.size();
				}

				int& row;
				split_interval_map<int,row_t>& _rows;
				region_loc reg;
			};

			struct area_vis : public boost::static_visitor<po::bound>
			{
				po::bound operator()(bblock_loc bb) const
				{
					return bb->area();
				}

				po::bound operator()(struct_loc s) const
				{
					return s->area();
				}
			};

			if(r)
			{
				po::bound a = boost::apply_visitor(area_vis(), *r);

				if(a.lower() > o)
				{
					po::offset delta = a.lower() - o;
					int nrow = row + (delta / columnWidth) + (delta % columnWidth == 0 ? 0 : 1);
					interval<int>::type iv(row,nrow);
					row_t r(p.second.lock(),row_t::second_type(po::bound(o,a.lower())));

					_rows += std::make_pair(iv,r);
					row = nrow;
				}

				boost::apply_visitor(add_vis(row,_rows,p.second.lock()), *r);
				o = a.upper();
			}
			else
			{
				po::offset delta = p.first.upper() - 1 - o;

				if(delta > 0)
				{
					int nrow = row + (delta / columnWidth) + (delta % columnWidth == 0 ? 0 : 1);
					interval<int>::type iv(row,nrow);
					row_t r(p.second.lock(),po::bound(o,p.first.upper()));

					_rows += std::make_pair(iv,r);
					row = nrow;
				}

				o = p.first.upper();
			}
		}
	}

	for(auto p: _rows)
	{
		struct visitor : public boost::static_visitor<std::string>
		{
			std::string operator()(const po::bound& b) const { return "[" + std::to_string(b.lower()) + ":" + std::to_string(b.upper()) + ")"; }
			std::string operator()(const po::bblock_loc& bb) const { return "bb(" + std::to_string((ptrdiff_t)(&(*bb))) + ")"; }
			std::string operator()(const po::struct_loc&) const { return "struct"; }
		};
		std::cout << p.first << " " << boost::apply_visitor(visitor(), p.second.second) << std::endl;
	}
}

int LinearModel::rowCount(const QModelIndex& parent) const
{
	if(parent == QModelIndex())
		return boost::icl::size(_rows);
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
		struct string_vis : public boost::static_visitor<std::pair<QString,po::bound>>
		{
			string_vis(int r, interval<int>::type iv, region_loc re) : row(r), ival(iv), reg(re) {}

			std::pair<QString,po::bound> operator()(po::bound b) const
			{
				po::offset o = b.lower() + (row - ival.lower()) * columnWidth;
				po::offset p = std::min<po::offset>(o + columnWidth,b.upper());
				slab sl = reg->read();
				auto i = boost::begin(sl) + o;
				auto j = boost::begin(sl) + p;
				QStringList hex, text;

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

				return std::make_pair(
					QString("{ 'type': 'raw', 'hex': %2, 'text': %3 }")
						.arg("[" + hex.join(",") + "]")
						.arg("[" + text.join(",") + "]"),
					po::bound(o,p));
			}

			std::pair<QString,po::bound> operator()(bblock_loc bb) const
			{
				int o = row - ival.lower();

				std::cout << "mne: " << o << std::endl;
				const mnemonic& mne = bb->mnemonics().at(o);
				QStringList ops;

				for(auto q: mne.operands)
				{
					std::stringstream ss;
					ss << q;
					ops.append("'" + QString::fromStdString(ss.str()) + "'");
				}

				return std::make_pair(
					QString("{ 'type': 'mne', 'op': '%1', 'args': %2 }")
						.arg(QString::fromStdString(mne.opcode))
						.arg("[" + ops.join(",") + "]"),
					mne.area);
			}

			std::pair<QString,po::bound> operator()(struct_loc s) const
			{
				//int o = ival.lower() - row;

				return std::make_pair(
					QString("{ 'type': 'struct', 'name': 'name', 'value': 'value' }"),
					po::bound());
			}

			int row;
			interval<int>::type ival;
			region_loc reg;
		};

		QString payload;
		po::bound b;
		auto r = _rows.find(idx.row());

		if(r == _rows.end())
		{
			std::cout << "idx: " << idx.row() << " = none" << std::endl;
			return QVariant();
		}

		std::cout << "idx: " << idx.row() << " = " << r->first << std::endl;

		QStringList comments;

		std::tie(payload,b) = boost::apply_visitor(string_vis(idx.row(),r->first,r->second.first.lock()), r->second.second);

		auto k = _dbase->comments.lower_bound(ref{r->second.first->name(),b.lower()});
		while(k != _dbase->comments.end() &&
					k->first.reg == r->second.first->name() &&
					k->first.off < b.upper())
		{
			comments.append(QString::fromStdString(*(k->second)));
			++k;
		}

		ensure(payload.size());

		return QString("{ 'address': '0x%1', 'payload': %2, 'comment': '%4' }")
						.arg(b.lower(),0,16)
						.arg(payload)
						.arg(comments.join("\n"));
	}
}

void LinearModel::postComment(int row, QString c)
{
	po::offset o = 0, t = row * columnWidth;
	for(auto p: _projection)
	{
		if(o <= t && o + size(p.first) > t)
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
: QObject(p), _session(sess), _linear(new LinearModel(sess.dbase,this)), _procedures()
{
	for(auto prog: _session.dbase->programs)
		for(auto proc: prog->procedures())
			_procedures.append(QString::fromStdString(proc->name));
}

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

void Session::disassemble(int r, int c)
{
	qDebug() << "start disassemble at" << r << "/" << c;
}

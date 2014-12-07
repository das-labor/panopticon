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
: QAbstractListModel(p), _dbase(db), _projection(filterUndefined(po::projection(_dbase->data)))
{
	int row = 0;

	for(auto p: _projection)
	{
		if(std::get<2>(p))
		{
			po::offset o = std::get<0>(p).lower();

			while(o < std::get<0>(p).upper())
			{
				auto r = po::next_record(ref{std::get<1>(p)->name(),o},_dbase);

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
						row_t r(std::get<1>(p).lock(),row_t::second_type(po::bound(o,a.lower())));

						_rows += std::make_pair(iv,r);
						row = nrow;
					}

					auto old_row = row;
					boost::apply_visitor(add_vis(row,_rows,std::get<1>(p).lock()), *r);

					try
					{
						bblock_loc bb = boost::get<bblock_loc>(*r);
						for(auto pro: _dbase->programs)
						{
							for(auto proc: pro->procedures())
							{
								boost::optional<bblock_loc> maybe_bb;
								if((maybe_bb = find_bblock(proc,bb->area().lower())))
								{
									auto vx = find_node(boost::variant<bblock_loc,rvalue>(bb),proc->control_transfers);

									if(maybe_bb == proc->entry)
									{
										_procedures.emplace(proc->name,old_row);
									}

									for(auto e: iters(in_edges(vx,proc->control_transfers)))
									{
										try
										{
											auto vy = source(e,proc->control_transfers);
											bblock_loc ba = boost::get<bblock_loc>(get_vertex(vy,proc->control_transfers));

											if(ba->area().upper() != bb->area().lower() && ba != bb)
												findTrack(po::bound(std::min(bb->area().lower(),ba->area().upper() - 1),
																						std::max(bb->area().lower(),ba->area().upper() - 1)),
																	bb->area().lower() < ba->area().upper() - 1);
										}
										catch(const boost::bad_get&)
										{}
									}

									for(auto e: iters(out_edges(vx,proc->control_transfers)))
									{
										try
										{
											auto vy = target(e,proc->control_transfers);
											bblock_loc ba = boost::get<bblock_loc>(get_vertex(vy,proc->control_transfers));

											if(ba->area().lower() != bb->area().upper() && ba != bb)
												findTrack(po::bound(std::min(bb->area().upper() - 1,ba->area().lower()),
																						std::max(bb->area().upper() - 1,ba->area().lower())),
																	bb->area().lower() >= ba->area().upper() - 1);
										}
										catch(const boost::bad_get&)
										{}
									}
								}
							}
						}
					}
					catch(const boost::bad_get&)
					{}

					o = a.upper();
				}
				else
				{
					po::offset delta = std::get<0>(p).upper() - 1 - o;

					if(delta > 0)
					{
						long long nrow = row + (delta / columnWidth) + (delta % columnWidth == 0 ? 0 : 1);
						interval<int>::type iv(row,nrow);
						row_t r(std::get<1>(p).lock(),po::bound(o,std::get<0>(p).upper()));

						_rows += std::make_pair(iv,r);
						row = nrow;
					}

					o = std::get<0>(p).upper();
				}
			}
		}
	}

	ensure(_rows.size());
}

int LinearModel::rowForProcedure(QString s) const
{
	std::string n = s.toStdString();
	auto i = _procedures.find(n);
	return i != _procedures.end() ? i->second : -1;
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
		auto r = _rows.find(idx.row());
		if(r == _rows.end())
			return QVariant();

		// payload
		QString payload;
		po::bound b;
		std::list<po::bound> pass;
		std::tie(payload,b,pass) = boost::apply_visitor(data_visitor(idx.row(),r->first,r->second.first.lock()), r->second.second);
		ensure(payload.size());

		// comments
		QStringList comments;

		auto k = _dbase->comments.lower_bound(ref{r->second.first->name(),b.lower()});
		while(k != _dbase->comments.end() &&
					k->first.reg == r->second.first->name() &&
					k->first.off < b.upper())
		{
			comments.append(QString::fromStdString(*(k->second)));
			++k;
		}

		// arrows
		QStringList pass_here, end_here, begin_here;
		size_t track = 0;
		boost::icl::discrete_interval<po::offset> iv = boost::icl::discrete_interval<po::offset>::closed(b.lower(),b.upper() - 1);

		while(track < _tracks.size())
		{
			const boost::icl::split_interval_map<po::offset,int>& tr = *(std::next(_tracks.begin(),track));

			auto i = tr.lower_bound(iv);
			while(i != tr.end() && !boost::icl::disjoint(i->first,iv))
			{
				if(boost::icl::contains(b,i->first.lower()))
					begin_here.append(QString("{ 'track': %1, 'tip': %2 }").arg(track).arg(i->second == 2));
				else if(boost::icl::contains(b,i->first.upper()))
					end_here.append(QString("{ 'track': %1, 'tip': %2 }").arg(track).arg(i->second != 2));
				else
					pass_here.append(QString("%1").arg(track));
				++i;
			}

			++track;
		}

		QString arrows = QString("{ 'pass': [%1], 'end': [%2], 'begin': [%3] }")
											.arg(pass_here.join(","))
											.arg(end_here.join(","))
											.arg(begin_here.join(","));

		return QString("{ 'address': '0x%1', 'payload': %2, 'comment': '%3', 'arrows': %4 }")
						.arg(b.lower(),0,16)
						.arg(payload)
						.arg(comments.join("\n"))
						.arg(arrows);
	}
}

void LinearModel::postComment(int row, QString c)
{
	auto r = _rows.find(row);
	ensure(r != _rows.end());

	QString payload;
	po::bound b;
	std::list<po::bound> pass;
	std::tie(payload,b,pass) = boost::apply_visitor(data_visitor(row,r->first,r->second.first.lock()), r->second.second);
	ref rr{r->second.first.lock()->name(),b.lower()};
	auto k = _dbase->comments.lower_bound(rr);

	while(k != _dbase->comments.end() &&
				k->first.reg == rr.reg &&
				k->first.off < b.lower() + columnWidth)
	{
		k = _dbase.write().comments.erase(k);
	}

	_dbase.write().comments.insert(std::make_pair(rr,comment_loc(new std::string(c.toStdString()))));
	dataChanged(createIndex(row,0),createIndex(row,0));
}

std::list<std::tuple<po::bound,po::region_wloc,bool>> LinearModel::filterUndefined(const std::list<std::pair<po::bound,po::region_wloc>>& l) const
{
	std::list<std::tuple<po::bound,po::region_wloc,bool>> ret;

	for(auto p: l)
	{
		const std::list<std::pair<bound,layer_wloc>>& flat = p.second.lock()->flatten();

		for(auto q: flat)
		{
			po::bound b = q.first & p.first;

			if(b.lower() != b.upper())
				ret.emplace_back(b,p.second,!q.second.lock()->is_undefined());
		}
	}

	return ret;
}

int LinearModel::findTrack(po::bound b, bool d)
{
	if(boost::icl::size(b) == 0)
		throw std::invalid_argument("bound is empty");

	boost::icl::discrete_interval<po::offset> iv = boost::icl::discrete_interval<po::offset>::closed(b.lower(),b.upper());
	auto i = _tracks.begin();
	while(i != _tracks.end())
	{
		boost::icl::split_interval_map<po::offset,int>& s = *i;

		auto p = boost::icl::find(s,iv);

		if(p == s.end() || boost::icl::disjoint(p->first,iv))
		{
			s += std::make_pair(iv,d ? 2:1);
			return std::distance(_tracks.begin(),i);
		}
		else if(p->first == iv)
		{
			return std::distance(_tracks.begin(),i);
		}

		++i;
	}

	_tracks.push_back(boost::icl::split_interval_map<po::offset,int>());
	_tracks.back() += (std::make_pair(iv,d?2:1));

	return _tracks.size() - 1;
}

LinearModel::data_visitor::data_visitor(int r, interval<int>::type iv, region_loc re) : row(r), ival(iv), reg(re) {}

std::tuple<QString,po::bound,std::list<po::bound>> LinearModel::data_visitor::operator()(po::bound b) const
{
	po::offset o = b.lower() + (row - ival.lower()) * columnWidth;
	po::offset p = std::min<po::offset>(o + columnWidth,b.upper());
	slab sl = reg->read();
	auto i = sl.begin() + o;
	auto j = sl.begin() + p;
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
			text += "''";
		}
	}

	while(hex.size() < columnWidth)
	{
		hex += QString("''");
		text += QString("''");
	}

	return std::make_tuple(
		QString("{ 'type': 'raw', 'hex': %2, 'text': %3 }")
			.arg("[" + hex.join(",") + "]")
			.arg("[" + text.join(",") + "]"),
		po::bound(o,p),
		std::list<po::bound>());
}

std::tuple<QString,po::bound,std::list<po::bound>> LinearModel::data_visitor::operator()(bblock_loc bb) const
{
	size_t o = row - ival.lower();

	const mnemonic& mne = bb->mnemonics().at(o);
	QStringList ops, hex;
	slab sl = reg->read();
	auto i = sl.begin() + mne.area.lower();
	auto j = sl.begin() + mne.area.upper();
	std::list<po::bound> conn;

	for(po::tryte s: iters(std::make_pair(i,j)))
	{
		if(s)
			hex += QString("'%1'").arg(static_cast<unsigned int>(*s),2,16,QChar('0'));
		else
			hex += "'?""?'";
	}

	for(auto q: mne.operands)
	{
		std::stringstream ss;
		ss << q;
		ops.append("'" + QString::fromStdString(ss.str()) + "'");
	}

	return std::make_tuple(
		QString("{ 'type': 'mne', 'op': '%1', 'args': %2, 'hex': %3, 'begin': %4, 'end': %5 }")
			.arg(QString::fromStdString(mne.opcode))
			.arg("[" + ops.join(",") + "]")
			.arg("[" + hex.join(",") + "]")
			.arg(o == 0 ? "true" : "false")
			.arg(o == bb->mnemonics().size() - 1 ? "true" : "false"),
		mne.area,
		conn);
}

std::tuple<QString,po::bound,std::list<po::bound>> LinearModel::data_visitor::operator()(struct_loc s) const
{
	return std::make_tuple(
		QString("{ 'type': 'struct', 'name': 'name', 'value': 'value' }"),
		po::bound(),
		std::list<po::bound>());
}

Session::Session(po::session sess, QObject *p)
: QObject(p), _session(sess), _linear(new LinearModel(sess.dbase,this)), _procedures(), _activeProcedure(nullptr)
{
	bool set = false;

	for(auto proc: (*_session.dbase->programs.begin())->procedures())
	{
		auto p = new Procedure(this);

		p->setProcedure(proc);
		_procedures.append(QVariant::fromValue<QObject*>(p));
		if(!set)
		{
			set = true;
			//_graph->setProcedure(proc);
		}
	}
}

Session::~Session(void)
{}

Session* Session::open(QString s)
{
	return new Session(po::open(s.toStdString()));
}

Session* Session::createRaw(QString s)
{
	return new Session(po::raw(s.toStdString()));
}

Session* Session::createAvr(QString s)
{
	return new Session(po::raw_avr(s.toStdString()));
}

void Session::postComment(int r, QString c)
{
	_linear->postComment(r,c);
}

void Session::disassemble(int r, int c)
{
	qDebug() << "start disassemble at" << r << "/" << c;
}

void Session::setActiveProcedure(QObject* s)
{
	Procedure *proc = qobject_cast<Procedure*>(s);
	if(proc && proc != _activeProcedure && proc->procedure())
	{
		_activeProcedure = proc;

		/*auto prog = *_session.dbase->programs.begin();
		auto p = std::find_if(prog->procedures().begin(),prog->procedures().end(),[&](proc_loc p) { return p->name == s.toStdString(); });

		ensure(p != prog->procedures().end());

		_graph->setProcedure(*p);*/
		//_linear->setProcedure(*proc->procedure());

		emit activeProceduresChanged();
	}
}

#include <algorithm>
#include <functional>
#include <cassert>
#include <iostream>

#include <panopticon/procedure.hh>
#include <panopticon/program.hh>

#include <boost/graph/depth_first_search.hpp>

using namespace po;
using namespace std;
using namespace boost;

//domtree::domtree(bblock_ptr b) : intermediate(0), successors(), frontiers(), basic_block(b) {}

template<>
procedure* po::unmarshal(const uuid& u, const rdf::storage &store)
{
	rdf::node node(rdf::ns_po(to_string(u)));
	rdf::statement name = store.first(node,"name"_po);
	rdf::statements bbs = store.find(node,"include"_po);
	procedure* ret = new procedure(name.object.as_literal());
	using vx_desc = typename boost::graph_traits<decltype(ret->control_transfers)>::vertex_descriptor;

	for(auto bb: bbs)
		insert_node<boost::variant<bblock_loc,rvalue>,guard>(bblock_loc{uuid(bb.object.as_iri().substr(bb.object.as_iri().size()-36)),store},ret->control_transfers);

	for(auto bb: bbs)
	{
		vx_desc vx_a = find_node<boost::variant<bblock_loc,rvalue>,guard>(bblock_loc(uuid(bb.object.as_iri().substr(bb.object.as_iri().size()-36)),store),ret->control_transfers);
		rdf::statements succ = store.find(bb.object,rdf::ns_po("out")),
										pred = store.find(bb.object,rdf::ns_po("in"));
		std::unordered_set<rdf::node> cts;

		transform(succ.begin(),succ.end(),inserter(cts,cts.begin()),[&](const rdf::statement& st) { return st.object; });
		transform(pred.begin(),pred.end(),inserter(cts,cts.begin()),[&](const rdf::statement& st) { return st.object; });

		for(rdf::node ct: cts)
		{
			rdf::statement g = store.first(ct,rdf::ns_po("guard")),
										 target = store.first(ct,rdf::ns_po("target"));

			guard gg = *unique_ptr<guard>(unmarshal<guard>(uuid(g.object.as_iri().substr(g.object.as_iri().size()-36)),store));
			if(store.has(target.object,rdf::ns_rdf("type"),rdf::ns_po("BasicBlock")))
			{
				bblock_loc tgt(uuid(target.object.as_iri().substr(target.object.as_iri().size()-36)),store);
				vx_desc vx_b = find_node<boost::variant<bblock_loc,rvalue>,guard>(tgt,ret->control_transfers);

				insert_edge<boost::variant<bblock_loc,rvalue>,guard>(gg,vx_a,vx_b,ret->control_transfers);
			}
			else
			{
				rvalue tgt = *unique_ptr<rvalue>(unmarshal<rvalue>(uuid(target.object.as_iri().substr(target.object.as_iri().size()-36)),store));
				vx_desc vx_b = insert_node<boost::variant<bblock_loc,rvalue>,guard>(tgt,ret->control_transfers);

				insert_edge<boost::variant<bblock_loc,rvalue>,guard>(gg,vx_a,vx_b,ret->control_transfers);
			}
		}
	}

	rdf::statements ent = store.find(node,rdf::ns_po("entry"));
	if(ent.size())
	{
		rdf::node o = ent.front().object;
		uuid uu(o.as_iri().substr(o.as_iri().size()-36));
		auto p = vertices(ret->control_transfers);
		auto i = find_if(p.first,p.second,[&](vx_desc v)
		{
			auto n = get_node(v,ret->control_transfers);

			return get<bblock_loc>(&n) &&
						 get<bblock_loc>(n).tag() == uu;
		});

		assert(i != p.second);
		ret->entry = get<bblock_loc>(get_node(*i,ret->control_transfers));
	}

	return ret;
}

template<>
rdf::statements po::marshal(const procedure* p, const uuid& u)
{
	unsigned int cnt = 0;
	rdf::statements ret;
	boost::uuids::name_generator ng(u);
	rdf::node node(rdf::ns_po(to_string(u)));
	function<pair<rdf::node,rdf::statements>(const variant<rvalue,bblock_loc>&)> marshal_node = [&](const variant<rvalue,bblock_loc>& v) -> pair<rdf::node,rdf::statements>
	{
		if(get<rvalue>(&v))
		{
			uuid uu = ng(to_string(cnt++));
			rdf::node n = rdf::ns_local(to_string(uu));

			return make_pair(n,marshal(&get<rvalue>(v),uu));
		}
		else
			return make_pair(rdf::ns_local(to_string(get<bblock_loc>(v).tag())),rdf::statements());
	};

	ret.emplace_back(node,rdf::ns_rdf("type"),rdf::ns_po("Procedure"));
	ret.emplace_back(node,rdf::ns_po("name"),rdf::lit(p->name));

	for(auto e: iters(edges(p->control_transfers)))
	{
		if(get<rvalue>(&get_node(source(e,p->control_transfers),p->control_transfers)) &&
			 get<rvalue>(&get_node(target(e,p->control_transfers),p->control_transfers)))
			continue;

		uuid cu = ng(to_string(cnt++));
		rdf::node cn = rdf::ns_local(to_string(cu));
		uuid gu = ng(to_string(cnt++));
		rdf::node gn = rdf::ns_local(to_string(gu));
		rdf::statements g = marshal(&get_edge(e,p->control_transfers),gu);
		pair<rdf::node,rdf::statements> in_p = marshal_node(get_node(target(e,p->control_transfers),p->control_transfers));
		pair<rdf::node,rdf::statements> out_p = marshal_node(get_node(source(e,p->control_transfers),p->control_transfers));

		std::move(g.begin(),g.end(),back_inserter(ret));
		std::move(in_p.second.begin(),in_p.second.end(),back_inserter(ret));
		std::move(out_p.second.begin(),out_p.second.end(),back_inserter(ret));

		ret.emplace_back(cn,rdf::ns_po("guard"),gn);
		ret.emplace_back(cn,rdf::ns_po("in"),in_p.first);
		ret.emplace_back(cn,rdf::ns_po("out"),out_p.first);
	}

	for(auto bb: p->rev_postorder())
		ret.emplace_back(node,rdf::ns_po("include"),rdf::ns_local(to_string(bb.tag())));

	if(p->entry)
		ret.emplace_back(node,rdf::ns_po("entry"),rdf::ns_local(to_string(p->entry->tag())));

	return ret;
}


procedure::procedure(const std::string &n)
: name(n), entry(boost::none), control_transfers(), _rev_postorder(boost::none), _dominance(boost::none)
{}

const vector<bblock_loc>& procedure::rev_postorder(void) const
{
	assert(entry);

	if(!_rev_postorder)
	{
		using vx_desc = graph_traits<decltype(control_transfers)>::vertex_descriptor;
		using time_pm_type = associative_property_map<std::unordered_map<vx_desc,int>>;
		using color_pm_type = associative_property_map<std::unordered_map<vx_desc,default_color_type>>;

		std::unordered_map<vx_desc,int> ftime;
		std::unordered_map<vx_desc,default_color_type> color;

		_rev_postorder = make_optional(vector<bblock_loc>());

		if(num_vertices(control_transfers))
		{
			for(vx_desc vx: iters(vertices(control_transfers)))
				if(get<bblock_loc>(&get_node(vx,control_transfers)))
					_rev_postorder->push_back(get<bblock_loc>(get_node(vx,control_transfers)));

			int time = 0;
			depth_first_search(
				control_transfers,
				make_dfs_visitor(stamp_times(time_pm_type(ftime),time,on_finish_vertex())),
				color_pm_type(color),
				find_node<boost::variant<bblock_loc,rvalue>,guard>(*entry,control_transfers));

			assert(_rev_postorder->size() <= ftime.size());
			sort(_rev_postorder->begin(),_rev_postorder->end(),[&](bblock_loc a, bblock_loc b)
				{ return ftime[find_node<variant<bblock_loc,rvalue>,guard>(a,control_transfers)] < ftime[find_node<variant<bblock_loc,rvalue>,guard>(b,control_transfers)]; });
		}
	}
	return *_rev_postorder;
}

boost::optional<bblock_loc> po::find_bblock(proc_loc proc, offset a)
{
	for(auto vx: iters(vertices(proc->control_transfers)))
	{
		auto bb = get_node<variant<bblock_loc,rvalue>,guard>(vx,proc->control_transfers);
		if(get<bblock_loc>(&bb) && icl::contains(get<bblock_loc>(bb)->area(),a))
			return get<bblock_loc>(bb);
	}

	return boost::none;
}

std::pair<typename boost::shared_container_iterator<std::set<typename boost::graph_traits<po::digraph<boost::variant<bblock_loc,rvalue>,guard>>::edge_descriptor>>,typename boost::shared_container_iterator<std::set<typename boost::graph_traits<po::digraph<boost::variant<bblock_loc,rvalue>,guard>>::edge_descriptor>>>
po::incoming(proc_loc p, bblock_loc bb)
{
	return in_edges(find_node(variant<bblock_loc,rvalue>(bb),p->control_transfers),p->control_transfers);
}

std::pair<typename boost::graph_traits<decltype(procedure::control_transfers)>::out_edge_iterator,typename boost::graph_traits<decltype(procedure::control_transfers)>::out_edge_iterator>
po::outgoing(proc_loc p, bblock_loc bb)
{
	return out_edges(find_node(variant<bblock_loc,rvalue>(bb),p->control_transfers),p->control_transfers);
}

void po::execute(proc_loc proc,function<void(const lvalue &left, instr::Function fn, const vector<rvalue> &right)> f)
{
	for(const bblock_loc &bb: proc->rev_postorder())
	{
		size_t sz_mne = bb->mnemonics().size(), i_mne = 0;
		const mnemonic *ary_mne = bb->mnemonics().data();

		while(i_mne < sz_mne)
		{
			const mnemonic &mne = ary_mne[i_mne++];
			size_t sz_instr = mne.instructions.size(), i_instr = 0;
			const instr *ary_instr = mne.instructions.data();

			while(i_instr < sz_instr)
			{
				const instr &instr = ary_instr[i_instr++];

				f(instr.left,instr.function,instr.right);
			}
		}
	}
}

void po::conditional_jump(proc_loc p, bblock_loc from, bblock_loc to, guard g)
{
	using vx_desc = typename boost::graph_traits<decltype(p->control_transfers)>::vertex_descriptor;
	auto vx_a = find_node(variant<bblock_loc,rvalue>(from),p->control_transfers);
	auto vx_b = find_node(variant<bblock_loc,rvalue>(to),p->control_transfers);
	auto q = vertices(p->control_transfers);

	assert(std::find_if(q.first,q.second,[&](vx_desc d) { try { return get<bblock_loc>(get_node(d,p->control_transfers)) == from; } catch(...) { return false; } }) != q.second &&
				 std::find_if(q.first,q.second,[&](vx_desc d) { try { return get<bblock_loc>(get_node(d,p->control_transfers)) == to; } catch(...) { return false; } }) != q.second);
	insert_edge(g,vx_a,vx_b,p.write().control_transfers);
}

void po::conditional_jump(proc_loc p, bblock_loc from, rvalue to, guard g)
{
	auto vx_a = find_node(variant<bblock_loc,rvalue>(from),p->control_transfers);

	try
	{
		auto vx_b = find_node(variant<bblock_loc,rvalue>(to),p->control_transfers);
		insert_edge(g,vx_a,vx_b,p.write().control_transfers);
	}
	catch(const out_of_range&)
	{
		insert_edge(g,vx_a,insert_node(variant<bblock_loc,rvalue>(to),p.write().control_transfers),p.write().control_transfers);
	}
}

void po::unconditional_jump(proc_loc p, bblock_loc from, bblock_loc to)
{
	return conditional_jump(p,from,to,guard());
}

void po::unconditional_jump(proc_loc p, bblock_loc from, rvalue to)
{
	return conditional_jump(p,from,to,guard());
}

/*
void po::replace_incoming(bblock_loc to, bblock_loc oldbb, bblock_loc newbb)
{
	to->mutate_incoming([&](list<ctrans> &in)
	{
		replace(in,oldbb,newbb);
	});
}

void po::replace_outgoing(bblock_loc from, bblock_loc oldbb, bblock_loc newbb)
{
	assert(from && oldbb && newbb);
	from->mutate_outgoing([&](list<ctrans> &out)
	{
		replace(out,oldbb,newbb);
	});
}

void po::resolve_incoming(bblock_loc to, rvalue v, bblock_loc bb)
{
	assert(to && bb);
	to->mutate_incoming([&](list<ctrans> &in)
	{
		resolve(in,v,bb);
	});
}

void po::resolve_outgoing(bblock_loc from, rvalue v, bblock_loc bb)
{
	assert(from && bb);
	from->mutate_outgoing([&](list<ctrans> &out)
	{
		resolve(out,v,bb);
	});
}

// last == true -> pos is last in `up', last == false -> pos is first in `down'
pair<bblock_loc,bblock_loc> po::split(bblock_loc bb, addr_t pos, bool last)
{
	assert(bb);

	bblock_loc up(new basic_block()), down(new basic_block());
	bool sw = false;
	basic_block::out_iterator j,jend;
	basic_block::in_iterator k,kend;
	function<void(bool,bblock_loc,ctrans)> append = [](bool in, bblock_loc bb, ctrans ct)
	{
		if(in)
			bb->mutate_incoming([&](list<ctrans> &l) { l.push_back(ct); });
		else
			bb->mutate_outgoing([&](list<ctrans> &l) { l.push_back(ct); });
	};

	// distribute mnemonics under `up' and `down'
	for_each(bb->mnemonics().begin(),bb->mnemonics().end(),[&](const mnemonic &m)
	{
		assert(!m.area.includes(pos) || m.area.begin == pos);

		if(!last)
			sw |= m.area.includes(pos);

		if(sw)
			down->mutate_mnemonics([&](vector<mnemonic> &ms) { ms.push_back(m); });
		else
			up->mutate_mnemonics([&](vector<mnemonic> &ms) { ms.push_back(m); });

		if(last)
			sw |= m.area.includes(pos);
	});
	assert(sw);

	// move outgoing ctrans to down
	for_each(bb->outgoing().begin(),bb->outgoing().end(),[&](const ctrans &ct)
	{
		if(ct.bblock.lock() == bb)
		{
			append(false,down,ctrans(ct.condition,up));
			append(true,up,ctrans(ct.condition,up));
		}
		else
		{
			if(ct.bblock.lock())
			{
				append(false,down,ctrans(ct.condition,ct.bblock.lock()));
				ct.bblock.lock()->mutate_incoming([&](list<ctrans> &in)
				{
					in.emplace_back(ctrans(ct.condition,down));
					in.erase(find_if(in.begin(),in.end(),[&](const ctrans &ct)
						{ return ct.bblock.lock() == bb; }));
				});
			}
			else
				append(false,down,ctrans(ct.condition,ct.value));
		}
	});

	// move incoming edges to up
	for_each(bb->incoming().begin(),bb->incoming().end(),[&](const ctrans &ct)
	{
		if(ct.bblock.lock() == bb)
		{
			append(true,up,ctrans(ct.condition,down));
			append(false,down,ctrans(ct.condition,up));
		}
		else
		{
			if(ct.bblock.lock())
			{
				append(true,up,ctrans(ct.condition,ct.bblock.lock()));
				ct.bblock.lock()->mutate_outgoing([&](list<ctrans> &out)
				{
					out.emplace_back(ctrans(ct.condition,up));
					out.erase(find_if(out.begin(),out.end(),[&](const ctrans &ct)
						{ return ct.bblock.lock() == bb; }));
				});
			}
			else
				append(true,up,ctrans(ct.condition,ct.value));
		}
	});

	bb->clear();
	unconditional_jump(up,down);
	return make_pair(up,down);
}

bblock_loc po::merge(bblock_loc up, bblock_loc down)
{
	assert(up && down);
	if(up->area().begin == down->area().end) tie(up,down) = make_pair(down,up);
	assert(up->area().end == down->area().begin);

	bblock_loc ret(new basic_block());
	auto fn = [&ret](const bblock_loc &bb, const mnemonic &m) { ret->mutate_mnemonics([&](vector<mnemonic> &ms)
		{ ms.push_back(m); }); };

	for_each(up->mnemonics().begin(),up->mnemonics().end(),bind(fn,up,placeholders::_1));
	for_each(down->mnemonics().begin(),down->mnemonics().end(),bind(fn,down,placeholders::_1));

	for_each(up->incoming().begin(),up->incoming().end(),[&](const ctrans &ct)
	{
		if(ct.bblock.lock())
			replace_outgoing(ct.bblock.lock(),up,ret);
		ret->mutate_incoming([&](list<ctrans> &in) { in.emplace_back(ct); });
	});

	for_each(down->outgoing().begin(),down->outgoing().end(),[&](const ctrans &ct)
	{
		if(ct.bblock.lock())
			replace_incoming(ct.bblock.lock(),down,ret);
		ret->mutate_outgoing([&](list<ctrans> &out) { out.emplace_back(ct); });
	});

	up->clear();
	down->clear();
	return ret;
}

void po::replace(list<ctrans> &lst, bblock_loc from, bblock_loc to)
{
	assert(from && to);

	auto i = lst.begin();
	while(i != lst.end())
	{
		ctrans ct = *i;
		if(ct.bblock.lock() == from)
			i = lst.insert(lst.erase(i),ctrans(ct.condition,to));
		++i;
	}
}

void po::resolve(list<ctrans> &lst, rvalue v, bblock_loc bb)
{
	assert(bb);

	auto i = lst.begin();
	while(i != lst.end())
	{
		ctrans ct = *i;
		if(ct.value == v)
			i = lst.insert(lst.erase(i),ctrans(ct.condition,bb));
		++i;
	}
}*/

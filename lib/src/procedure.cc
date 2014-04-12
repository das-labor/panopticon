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
		pair<rdf::node,rdf::statements> in_p = marshal_node(target(e,p->control_transfers));
		pair<rdf::node,rdf::statements> out_p = marshal_node(source(e,p->control_transfers));

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
	if(!_rev_postorder)
	{
		using vx_desc = graph_traits<decltype(control_transfers)>::vertex_descriptor;
		using time_pm_type = associative_property_map<std::unordered_map<vx_desc,int>>;
		using color_pm_type = associative_property_map<std::unordered_map<vx_desc,default_color_type>>;

		std::unordered_map<vx_desc,int> ftime;
		std::unordered_map<vx_desc,default_color_type> color;

		_rev_postorder = make_optional(vector<bblock_loc>());

		for(vx_desc vx: iters(vertices(control_transfers)))
			if(get<bblock_loc>(&get_node(vx,control_transfers)))
				_rev_postorder->push_back(get<bblock_loc>(get_node(vx,control_transfers)));

		int time = 0;
		depth_first_search(
			control_transfers,
			make_dfs_visitor(stamp_times(time_pm_type(ftime),time,on_finish_vertex())),
			color_pm_type(color),
			find_node<boost::variant<bblock_loc,rvalue>,guard>(*entry,control_transfers));

		assert(_rev_postorder->size() == ftime.size());
		sort(_rev_postorder->begin(),_rev_postorder->end(),[&](bblock_loc a, bblock_loc b)
			{ return ftime[find_node<variant<bblock_loc,rvalue>,guard>(a,control_transfers)] < ftime[find_node<variant<bblock_loc,rvalue>,guard>(b,control_transfers)]; });
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

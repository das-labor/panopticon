#include <panopticon/database.hh>

using namespace po;
using namespace std;

template<>
rdf::statements po::marshal(const database* db, const uuid& u)
{
	rdf::statements ret;
	rdf::node root = rdf::ns_local(to_string(u));
	boost::uuids::name_generator ng(u);

	ret.emplace_back(root,rdf::ns_rdf("type"),rdf::ns_po("Database"));
	ret.emplace_back(root,rdf::ns_po("title"),rdf::lit(db->title));

	for(auto s: db->structures)
		ret.emplace_back(root,rdf::ns_po("structure"),rdf::ns_local(to_string(s.tag())));
	for(auto s: db->programs)
		ret.emplace_back(root,rdf::ns_po("program"),rdf::ns_local(to_string(s.tag())));
	for(auto s: db->comments)
	{
		rdf::node cmt = rdf::ns_local(to_string(ng("cmnt" + to_string(s.second.tag()))));
		ret.emplace_back(root,rdf::ns_po("comment"),cmt);
		ret.emplace_back(cmt,rdf::ns_po("region-name"),rdf::lit(s.first.reg));
		ret.emplace_back(cmt,rdf::ns_po("offset"),rdf::lit(s.first.off));
		ret.emplace_back(cmt,rdf::ns_po("body"),rdf::ns_local(to_string(s.second.tag())));
	}

	size_t cnt = 0;

	for(auto e: iters(edges(db->data)))
	{
		rdf::node nd = rdf::ns_local(to_string(ng("reg" + to_string(cnt++))));
		rdf::node from_nd = rdf::ns_local(to_string(get_vertex(source(e,db->data),db->data).tag()));
		rdf::node to_nd = rdf::ns_local(to_string(get_vertex(target(e,db->data),db->data).tag()));
		bound ar = get_edge(e,db->data);
		rdf::node ar_nd = rdf::lit(to_string(ar.lower()) + ":" + to_string(ar.upper()));

		ret.emplace_back(nd,rdf::ns_po("area"),ar_nd);
		ret.emplace_back(nd,rdf::ns_po("from"),from_nd);
		ret.emplace_back(nd,rdf::ns_po("to"),to_nd);
		ret.emplace_back(root,rdf::ns_po("mapping"),nd);
	}

	for(auto o: iters(vertices(db->data)))
		ret.emplace_back(root,rdf::ns_po("region"),rdf::ns_local(to_string(get_vertex(o,db->data).tag())));

	return ret;
}

template<>
database* po::unmarshal(const uuid& u, const rdf::storage& store)
{
	using vx = boost::graph_traits<po::regions>::vertex_descriptor;
	rdf::node root = rdf::ns_local(to_string(u));

	if(!store.has(root,rdf::ns_rdf("type"),rdf::ns_po("Database")))
		throw marshal_exception("invalid type");

	rdf::statement title_st = store.first(root,rdf::ns_po("title"));
	rdf::statements struct_st = store.find(root,rdf::ns_po("structure"));
	rdf::statements prog_st = store.find(root,rdf::ns_po("program"));
	rdf::statements cmnt_st = store.find(root,rdf::ns_po("comment"));

	std::unordered_set<struct_loc> structs;
	std::unordered_set<prog_loc> progs;
	std::map<ref,comment_loc> cmnts;
	regions data;

	for(auto st: struct_st)
		structs.emplace(struct_loc{uuid(st.object.as_iri().substr(st.object.as_iri().size()-36)),store});
	for(auto st: prog_st)
		progs.emplace(prog_loc{uuid(st.object.as_iri().substr(st.object.as_iri().size()-36)),store});
	for(auto st: cmnt_st)
	{
		rdf::statement reg_st = store.first(st.object,rdf::ns_po("region-name"));
		rdf::statement off_st = store.first(st.object,rdf::ns_po("offset"));
		rdf::statement body_st = store.first(st.object,rdf::ns_po("body"));

		cmnts.emplace(ref{reg_st.object.as_literal(),stoull(off_st.object.as_literal())},comment_loc{uuid(body_st.object.as_iri().substr(body_st.object.as_iri().size()-36)),store});
	}
	for(auto st: store.find(root,rdf::ns_po("region")))
		insert_vertex(region_loc{uuid(st.object.as_iri().substr(st.object.as_iri().size()-36)),store},data);
	for(auto st: store.find(root,rdf::ns_po("mapping")))
	{
		rdf::node reg = st.object;
		rdf::statement from_st = store.first(reg,rdf::ns_po("from"));
		rdf::statement to_st = store.first(reg,rdf::ns_po("to"));
		rdf::statement ar_st = store.first(reg,rdf::ns_po("area"));
		std::string b = ar_st.object.as_literal();
		std::string::size_type div = b.find(":");

		if(div == std::string::npos)
			throw marshal_exception("ill-formed bound");

		region_loc from{uuid(from_st.object.as_iri().substr(from_st.object.as_iri().size()-36)),store};
		region_loc to{uuid(to_st.object.as_iri().substr(to_st.object.as_iri().size()-36)),store};

		vx f = find_node(from,data);
		vx t = find_node(to,data);
		insert_edge(bound(stoull(b.substr(0,div)),stoull(b.substr(div + 1))),f,t,data);
	}

	return new database{title_st.object.as_literal(),data,structs,progs,cmnts};
}

#include <panopticon/database.hh>
#include <panopticon/marshal.hh>
#include <panopticon/avr/avr.hh>

using namespace po;
using namespace std;

template<>
archive po::marshal(const database* db, const uuid& u)
{
	rdf::statements ret;
	rdf::node root = rdf::iri(u);
	boost::uuids::name_generator ng(u);

	ret.emplace_back(root,rdf::ns_rdf("type"),rdf::ns_po("Database"));
	ret.emplace_back(root,rdf::ns_po("title"),rdf::lit(db->title));

	for(auto s: db->structures)
		ret.emplace_back(root,rdf::ns_po("structure"),rdf::iri(s.tag()));
	for(auto s: db->programs)
		ret.emplace_back(root,rdf::ns_po("program"),rdf::iri(s.tag()));
	for(auto s: db->comments)
	{
		rdf::node cmt = rdf::iri(ng("cmnt" + to_string(s.second.tag())));
		ret.emplace_back(root,rdf::ns_po("comment"),cmt);
		ret.emplace_back(cmt,rdf::ns_po("region-name"),rdf::lit(s.first.reg));
		ret.emplace_back(cmt,rdf::ns_po("offset"),rdf::lit(s.first.off));
		ret.emplace_back(cmt,rdf::ns_po("body"),rdf::iri(s.second.tag()));
	}

	size_t cnt = 0;

	for(auto e: iters(edges(db->data)))
	{
		rdf::node nd = rdf::iri(ng("reg" + to_string(cnt++)));
		rdf::node from_nd = rdf::iri(get_vertex(source(e,db->data),db->data).tag());
		rdf::node to_nd = rdf::iri(get_vertex(target(e,db->data),db->data).tag());
		bound ar = get_edge(e,db->data);
		rdf::node ar_nd = rdf::lit(to_string(ar.lower()) + ":" + to_string(ar.upper()));

		ret.emplace_back(nd,rdf::ns_po("area"),ar_nd);
		ret.emplace_back(nd,rdf::ns_po("from"),from_nd);
		ret.emplace_back(nd,rdf::ns_po("to"),to_nd);
		ret.emplace_back(root,rdf::ns_po("mapping"),nd);
	}

	for(auto o: iters(vertices(db->data)))
		ret.emplace_back(root,rdf::ns_po("region"),rdf::iri(get_vertex(o,db->data).tag()));

	return ret;
}

template<>
database* po::unmarshal(const uuid& u, const rdf::storage& store)
{
	using vx = boost::graph_traits<po::regions>::vertex_descriptor;
	rdf::node root = rdf::iri(u);

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
		structs.emplace(struct_loc{st.object.as_iri().as_uuid(),store});
	for(auto st: prog_st)
		progs.emplace(prog_loc{st.object.as_iri().as_uuid(),store});
	for(auto st: cmnt_st)
	{
		rdf::statement reg_st = store.first(st.object,rdf::ns_po("region-name"));
		rdf::statement off_st = store.first(st.object,rdf::ns_po("offset"));
		rdf::statement body_st = store.first(st.object,rdf::ns_po("body"));

		ref r{reg_st.object.as_literal(),stoull(off_st.object.as_literal())};
		comment_loc c(body_st.object.as_iri().as_uuid(),store);

		cmnts.insert(std::make_pair(r,c));
	}
	for(auto st: store.find(root,rdf::ns_po("region")))
		insert_vertex(region_loc{st.object.as_iri().as_uuid(),store},data);
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

		region_loc from{from_st.object.as_iri().as_uuid(),store};
		region_loc to{to_st.object.as_iri().as_uuid(),store};

		vx f = find_node(from,data);
		vx t = find_node(to,data);
		insert_edge(bound(stoull(b.substr(0,div)),stoull(b.substr(div + 1))),f,t,data);
	}

	return new database{title_st.object.as_literal(),data,structs,progs,cmnts};
}

session::~session(void)
{
	// erase dbase before store
	rdf::storage st;
	dbase = dbase_loc(uuid(),st);
	store.reset();
}

session po::open(const std::string& path)
{
	std::shared_ptr<rdf::storage> store = make_shared<rdf::storage>(path);
	rdf::statement st = store->first(rdf::ns_po("Root"),rdf::ns_po("meta"));
	dbase_loc db(st.object.as_iri().as_uuid(),*store);

	return session{db,store};
}

session po::raw(const std::string& path)
{
	std::shared_ptr<rdf::storage> store = make_shared<rdf::storage>();
	dbase_loc db(new database());

	db.write().title = boost::filesystem::path(path).filename().string();
	region_loc reg = region::mmap("base",path);
	insert_vertex(reg,db.write().data);

	return session{db,store};
}

session po::raw_avr(const std::string& path)
{
	std::shared_ptr<rdf::storage> store = make_shared<rdf::storage>();
	dbase_loc db(new database());

	db.write().title = boost::filesystem::path(path).filename().string();
	region_loc reg = region::mmap("base",path);
	insert_vertex(reg,db.write().data);

	po::slab sl = reg->read();
	prog_loc p = avr::disassemble(boost::none,sl,po::ref{"base",0});
	db.write().programs.insert(p);

	std::cout << "width: " << boost::size(sl) << std::endl;

	std::cout << p->procedures().size() << " procedures" << std::endl;
	for(auto q: p->procedures())
		std::cout << q->name << "(" << (unsigned long)&(*q) << "): " << num_vertices(q->control_transfers) << " bblocks" << std::endl;

	return session{db,store};
}

boost::optional<record> po::next_record(const ref& r, dbase_loc db)
{
	boost::optional<std::pair<po::offset,record>> ret = boost::none;

	for(auto s: db->structures)
	{
		if(s->reg == r.reg)
		{
			if(s->area().lower() <= r.off && s->area().upper() > r.off)
			{
				return record(s);
			}
			else if(r.off < s->area().lower())
			{
				po::offset d = s->area().lower() - r.off;

				if(!ret || (d < ret->first))
					ret = std::make_pair(d,record(s));
			}
		}
	}

	for(auto p: db->programs)
	{
		if(p->reg == r.reg)
		{
			for(auto q: p->procedures())
			{
				for(auto c: iters(vertices(q->control_transfers)))
				{
					try
					{
						auto bb = boost::get<bblock_loc>(get_vertex(c,q->control_transfers));

						if(bb->area().lower() <= r.off && bb->area().upper() > r.off)
						{
							return record(bb);
						}
						else if(r.off < bb->area().lower())
						{
							po::offset d = bb->area().lower() - r.off;

							if(!ret || (d < ret->first))
								ret = std::make_pair(d,record(bb));
						}
					}
					catch(const boost::bad_get&)
					{
						;
					}
				}
			}
		}
	}

	return ret ? boost::make_optional(ret->second) : boost::none;
}

template<>
archive po::marshal(const std::string* c, const uuid& u)
{
	rdf::statements ret;
	rdf::node root = rdf::iri(u);

	ret.emplace_back(root,rdf::ns_rdf("type"),rdf::ns_po("Comment"));
	ret.emplace_back(root,rdf::ns_po("body"),rdf::lit(*c));

	return ret;
}

template<>
std::string* po::unmarshal(const uuid& u, const rdf::storage& store)
{
	rdf::node root = rdf::iri(u);

	if(!store.has(root,rdf::ns_rdf("type"),rdf::ns_po("Comment")))
		throw marshal_exception("invalid type");

	rdf::statement body_st = store.first(root,rdf::ns_po("body"));
	return new std::string(body_st.object.as_literal());
}

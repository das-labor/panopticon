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

session po::pe(const string& p)
{
	dbase_loc db(new database());

	blob file(p);
	region_loc ram = region::undefined("base",0xc0000000);

	if(file.size() < 2)
		throw runtime_error("file too short");

	std::cout << "file magic: " << file.data()[0] << file.data()[1] << std::endl;

	if(file.data()[0] != 'M' || file.data()[1] != 'Z')
		throw runtime_error("unknown magic");

	if(file.size() < 0x3c)
		throw runtime_error("file too short");

	size_t pe_off = *(uint8_t*)(file.data() + 0x3c);

	std::cout << "pe header at " << pe_off << std::endl;

	struct pe_hdr
	{
		// pe header
		char magic1;
		char magic2;
		char magic3;
		char magic4;
		uint16_t machine;
		uint16_t num_sections;
		uint32_t timestamp;
		uint32_t symtab;
		uint32_t num_symbols;
		uint16_t opthdr_size;
		uint16_t flags;
	};

	if(file.size() < pe_off + sizeof(pe_hdr))
		throw runtime_error("file too short");

	pe_hdr* hdr = (pe_hdr*)(file.data() + pe_off);

	std::cout << "magic1: " << hdr->magic1 << std::endl;
	std::cout << "magic2: " << hdr->magic2 << std::endl;
	std::cout << "magic3: " << hdr->magic3 << std::endl;
	std::cout << "magic4: " << hdr->magic4 << std::endl;
	std::cout << "machine: " << hdr->machine << std::endl;
	std::cout << "num_sections: " << hdr->num_sections << std::endl;
	std::cout << "timestamp: " << hdr->timestamp << std::endl;
	std::cout << "symtab: " << hdr->symtab << std::endl;
	std::cout << "num_symbols: " << hdr->num_symbols << std::endl;
	std::cout << "opthdr_size: " << std::dec << hdr->opthdr_size << std::endl;
	std::cout << "flags: " << hdr->flags << std::endl;

	if(hdr->magic1 != 'P' || hdr->magic2 != 'E' || hdr->magic3 != 0 || hdr->magic4 != 0)
		throw runtime_error("wrong magic");

	if(hdr->machine == 0x14c || hdr->machine == 0x864)
		std::cout << "ia32/amd64" << std::endl;
	else
		throw std::runtime_error("unsupported machine type");

	if(!(hdr->flags & 2))
		throw std::runtime_error("image not executable");

	struct opt_hdr
	{
		uint16_t magic;
		uint8_t major;
		uint8_t minor;
		uint32_t text_size;
		uint32_t data_size;
		uint32_t bss_size;
		uint32_t entry_point;
		uint32_t text_base;

		union
		{
			struct narrow
			{
				uint32_t data_base; // w
				uint32_t image_base; // w
				uint32_t section_align;
				uint32_t file_align;
				uint16_t os_major;
				uint16_t os_minor;
				uint16_t imgae_major;
				uint16_t image_minor;
				uint16_t subsys_major;
				uint16_t subsys_minor;
				uint32_t win32_ver;
				uint32_t image_size;
				uint32_t header_size;
				uint32_t checksum;
				uint16_t subsys;
				uint16_t dll_flags;
				uint32_t stack_reserve; // w
				uint32_t stack_commit; // w
				uint32_t heap_reserve; // w
				uint32_t heap_commit; // w
				uint32_t loader_flags;
				uint32_t datadir_entries;
			} pe;

			struct wide
			{
				uint64_t image_base; // w
				uint32_t section_align;
				uint32_t file_align;
				uint16_t os_major;
				uint16_t os_minor;
				uint16_t imgae_major;
				uint16_t image_minor;
				uint16_t subsys_major;
				uint16_t subsys_minor;
				uint32_t win32_ver;
				uint32_t image_size;
				uint32_t header_size;
				uint32_t checksum;
				uint16_t subsys;
				uint16_t dll_flags;
				uint64_t stack_reserve; // w
				uint64_t stack_commit; // w
				uint64_t heap_reserve; // w
				uint64_t heap_commit; // w
				uint32_t loader_flags;
				uint32_t datadir_entries;
			} pe_plus;
		} u;
	};

	if(file.size() < pe_off + sizeof(pe_hdr) + 2)
		throw runtime_error("file too short");

	opt_hdr *opt = (opt_hdr*)(file.data() + pe_off + sizeof(pe_hdr));

	std::cout << "=== Optional Header === " << std::dec << sizeof(opt_hdr) << std::endl;

	size_t ddir_cnt = 0;
	uint64_t image_base = 0;

	if(opt->magic == 0x10b)
	{
		std::cout << "magic: " << std::hex << opt->magic << std::dec << " (PE)" << std::endl;
		std::cout << "entry: " << std::hex << opt->entry_point << std::dec << std::endl;
		std::cout << "image base: " << std::hex << opt->u.pe.image_base << std::endl;
		std::cout << "section alignment: " << opt->u.pe.section_align << std::endl;
		std::cout << "file alignment: " << opt->u.pe.file_align << std::endl;
		std::cout << "subsystem ver: " << opt->u.pe.subsys_major << "." << opt->u.pe.subsys_minor << std::endl;
		std::cout << "image size: " << opt->u.pe.image_size << std::endl;
		std::cout << "hdr size: " << opt->u.pe.header_size << std::endl;
		std::cout << "subsys: " << opt->u.pe.subsys << std::endl;
		ddir_cnt = opt->u.pe.datadir_entries;
		image_base = opt->u.pe.image_base;
	}
	else if(opt->magic == 0x20b)
	{
		std::cout << "magic: " << std::hex << opt->magic << std::dec << " (PE+)" << std::endl;
		std::cout << "entry: " << std::hex << opt->entry_point << std::dec << std::dec << std::endl;
		std::cout << "image base: " << std::hex << opt->u.pe_plus.image_base << std::dec << std::endl;
		std::cout << "section alignment: " << opt->u.pe_plus.section_align << std::endl;
		std::cout << "file alignment: " << opt->u.pe_plus.file_align << std::endl;
		std::cout << "subsystem ver: " << opt->u.pe_plus.subsys_major << "." << opt->u.pe_plus.subsys_minor << std::endl;
		std::cout << "image size: " << opt->u.pe_plus.image_size << std::endl;
		std::cout << "hdr size: " << opt->u.pe_plus.header_size << std::endl;
		std::cout << "subsys: " << opt->u.pe_plus.subsys << std::endl;
		ddir_cnt = opt->u.pe_plus.datadir_entries;
		image_base = opt->u.pe_plus.image_base;
	}
	else
		throw std::runtime_error("unknown optional header magic");


	struct ddir_entry
	{
		int32_t rva;
		uint32_t size;
	};

	std::cout << "data directory offset: " << pe_off + sizeof(pe_hdr) + hdr->opthdr_size - sizeof(ddir_entry) * ddir_cnt << std::endl;
	ddir_entry *ddir = (ddir_entry*)(file.data() + pe_off + sizeof(pe_hdr) - sizeof(ddir_entry) * ddir_cnt + hdr->opthdr_size);
	size_t ddir_idx = 0;

	while(ddir_idx < ddir_cnt)
	{
		std::cout << "data dir entry #" << ddir_idx << std::endl;
		std::cout << "rva: " << ddir[ddir_idx].rva << " (" << ddir[ddir_idx].rva + image_base << ")" << std::endl;
		std::cout << "size: " << ddir[ddir_idx].size << std::endl;

		++ddir_idx;
	}

	struct section
	{
		char name0;
		char name1;
		char name2;
		char name3;
		char name4;
		char name5;
		char name6;
		char name7;

		uint32_t virt_sz_or_phy_addr;
		uint32_t virt_address;

		uint32_t raw_sz;
		uint32_t raw_ptr;
		uint32_t reloc_ptr;
		uint32_t linenr_ptr;
		uint16_t reloc_count;
		uint16_t linenr_cout;
		uint32_t flags;
	};

	section* sec_ptr = (section *)(file.data() + pe_off + sizeof(pe_hdr) + hdr->opthdr_size);
	size_t sec_idx = 0;

	while(sec_idx < hdr->num_sections)
	{
		section& s = sec_ptr[sec_idx++];
		std::string n = {s.name0, s.name1, s.name2, s.name3, s.name4, s.name5, s.name6, s.name7};

		std::cout << n << ": ";

		if(s.raw_sz)
		{
			layer_loc l(new layer(n,(uint8_t *)(file.data() + s.raw_ptr),s.raw_sz));
			ram.write().add(po::bound(image_base + s.virt_address,image_base + s.virt_address + s.raw_sz),l);

			std::cout << "mapped" << std::endl;
		}
		else
		{
			std::cout << "not mapped" << std::endl;

			if(s.virt_sz_or_phy_addr)
			{
				layer_loc l(new layer(n,s.virt_sz_or_phy_addr));
				ram.write().add(po::bound(image_base + s.virt_address,image_base + s.virt_address + s.virt_sz_or_phy_addr),l);
			}
		}
	}

	return session{db,std::make_shared<rdf::storage>()};
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

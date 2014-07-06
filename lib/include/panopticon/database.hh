#include <unordered_map>
#include <map>

#include <boost/shared_container_iterator.hpp>

#include <panopticon/tree.hh>
#include <panopticon/loc.hh>
#include <panopticon/marshal.hh>
#include <panopticon/structure.hh>
#include <panopticon/program.hh>
#include <panopticon/basic_block.hh>

#pragma once

namespace po
{
	/// Everything that occupys space on the region graph
	using record = boost::variant<bblock_loc,const field&>;
	using comment_loc = loc<std::string>;

	area extends(const record&);

	struct database
	{
		using record_iterator = boost::shared_container_iterator<std::unordered_set<record>>;

		std::string title;
		regions data;
		std::unordered_set<struct_loc> structures;
		std::unordered_set<prog_loc> programs;
		std::map<ref,comment_loc> comments;
	};

	using dbase_loc = loc<database>;
	using dbase_wloc = wloc<database>;

	template<>
	archive marshal(const database*, const uuid&);

	template<>
	database* unmarshal(const uuid&, const rdf::storage&);

	std::pair<database::record_iterator,database::record_iterator> lookup(const area& a, bool allow_overlap, dbase_loc d);

	struct session
	{
		~session(void);

		dbase_loc dbase;
		std::shared_ptr<rdf::storage> store;
	};

	session open(const std::string&);
	session elf(const std::string&);
	session pe(const std::string&);
	session raw(const std::string&);
	session macho(const std::string&);
	session empty(const std::string&);
}

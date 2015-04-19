/*
 * This file is part of Panopticon (http://panopticon.re).
 * Copyright (C) 2014 Kai Michaelis
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

#include <unordered_map>
#include <map>

#include <boost/shared_container_iterator.hpp>

#include <panopticon/tree.hh>
#include <panopticon/loc.hh>
#include <panopticon/marshal.hh>
#include <panopticon/structure.hh>
#include <panopticon/basic_block.hh>

#pragma once

namespace po
{
	/// Everything that occupys space on the region graph
	using record = boost::variant<bblock_loc,struct_loc>;
	using comment_loc = loc<std::string>;

	template<>
	archive marshal(const std::string*, const uuid&);

	template<>
	std::string* unmarshal(const uuid&, const rdf::storage&);

	struct database
	{
		using record_iterator = boost::shared_container_iterator<std::unordered_set<record>>;

		std::string title;
		regions data;
		std::unordered_set<struct_loc> structures;
		std::unordered_set<loc<struct program>> programs;
		std::map<ref,comment_loc> comments;
	};

	using dbase_loc = loc<database>;
	using dbase_wloc = wloc<database>;

	template<>
	archive marshal(const database*, const uuid&);

	template<>
	database* unmarshal(const uuid&, const rdf::storage&);

	boost::optional<record> next_record(const ref& r, dbase_loc db);

	struct session
	{
		~session(void);

		dbase_loc dbase;
		std::shared_ptr<rdf::storage> store;
	};

	session open(const std::string&);
	session elf(const std::string&);
	session raw_avr(const std::string&, struct avr_state const&);
	session pe(const std::string&);
	session raw(const std::string&);
	session macho(const std::string&);
	session empty(const std::string&);
}

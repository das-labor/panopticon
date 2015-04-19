/*
 * This file is part of Panopticon (http://panopticon.re).
 * Copyright (C) 2014 Panopticon authors
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

#include <string>
#include <functional>
#include <memory>

#include <boost/variant.hpp>

#include <panopticon/tree.hh>
#include <panopticon/region.hh>

namespace po
{
	struct format;

	struct format
	{
		struct ieee754
		{
			unsigned int bytes;
		};

		struct skip
		{
			offset bytes;
			std::string display;
		};

		struct integer
		{
			boost::variant<
				std::pair<bool,unsigned int>,															///< has_sign/bytes
				unsigned long long																				///< mask
			> mode;
			boost::variant<
				unsigned int,																							///< base
				std::pair<unsigned int,unsigned int>,											///< two bases
				std::shared_ptr<std::map<std::vector<byte>,std::string>>,	///< symbolic
				std::function<std::string(const std::list<byte>&)>				///< custom
			> display;
		};

		struct boolean
		{
			unsigned long long mask;
			std::pair<std::string,std::string> display;
		};

		struct reference
		{
			unsigned int bytes;
			offset off;
			std::string reg;
		};

		struct composition
		{
			boost::variant<
				std::string,
				std::function<std::string(const tree<format>&)>
			> display;
		};

		format(const std::string&, const ieee754&);
		format(const std::string&, const skip&);
		format(const std::string&, const integer&);
		format(const std::string&, const boolean&);
		format(const std::string&, const reference&);
		format(const std::string&, const composition&);

		std::string read(slab) const;
		unsigned int width(void) const;

		std::string name;
		boost::variant<ieee754,integer,boolean,composition> field;
	};

	tree<format> ipv4(region_loc reg);
}

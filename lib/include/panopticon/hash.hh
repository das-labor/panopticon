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

#include <unordered_set>

#pragma once

namespace po
{
	inline size_t hash_struct(void)
	{
		return 0;
	}

	/// Hashes a sequence of fields and combines them.
	template<typename Car, typename... Cdr>
	size_t hash_struct(const Car &c, const Cdr&... parameters)
	{
		size_t seed = std::hash<Car>()(c);
		return seed ^ (hash_struct(parameters...) + 0x9e3779b9 + (seed << 6) + (seed >> 2));
	}
}

namespace std
{
	template<typename A, typename B>
	struct hash<pair<A,B>>
	{
		size_t operator()(const pair<A,B> &p) const
		{
			return po::hash_struct(p.first,p.second);
		}
	};
}

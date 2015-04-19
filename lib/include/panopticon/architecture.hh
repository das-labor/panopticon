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
#include <vector>

#pragma once

#include <panopticon/value.hh>

namespace po
{
	template<typename T>
	struct architecture_traits
	{
		using token_type = void;	///< Smallest integer type that can hold one token
		using state_type = std::nullptr_t;	///< additional semantic info. Type of sem_state<Tag>::state
	};

	/// Generate new temporary variable. Must not collide with any previous temporaries.
	template<typename T>
	lvalue temporary(T);

	/// List of all registers supported by the architecture.
	template<typename T>
	const std::vector<std::string>& registers(T);

	/// Width of the register @arg n in bits. Allowed values for n are returned by registers<T>()
	template<typename T>
	uint8_t width(std::string n, T);
}

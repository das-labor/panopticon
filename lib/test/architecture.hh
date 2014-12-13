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

#include <panopticon/disassembler.hh>

#pragma once

struct test_tag {};
extern unsigned int ununsed;
extern std::vector<std::string> regs;

namespace po
{
	template<>
	struct architecture_traits<test_tag>
	{
		using token_type = unsigned char;
		using state_type = unsigned char;
	};

	template<>
	lvalue temporary(test_tag);

	template<>
	const std::vector<std::string> &registers(test_tag);

	template<>
	uint8_t width(std::string n, test_tag);
}

struct wtest_tag {};

namespace po
{
	template<>
	struct architecture_traits<wtest_tag>
	{
		using token_type = uint16_t;
		using state_type = unsigned char;
	};

	template<>
	lvalue temporary(wtest_tag);

	template<>
	const std::vector<std::string> &registers(wtest_tag);

	template<>
	uint8_t width(std::string n, wtest_tag);
}

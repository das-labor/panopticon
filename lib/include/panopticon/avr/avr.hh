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

#include <panopticon/program.hh>

#pragma once

namespace po
{
	// architecture_traits
	struct avr_tag {};

	template<>
	struct architecture_traits<avr_tag>
	{
		using token_type = uint16_t;
		using state_type = std::nullptr_t;
	};

	template<>
	lvalue temporary(avr_tag);

	template<>
	const std::vector<std::string> &registers(avr_tag);

	template<>
	uint8_t width(std::string n, avr_tag);

	namespace avr
	{
		typedef sem_state<avr_tag> sm;
		typedef std::function<void(sm &)> sem_action;
		typedef code_generator<avr_tag> cg;

		boost::optional<prog_loc> disassemble(boost::optional<prog_loc>, po::slab, const po::ref&);
	}
}

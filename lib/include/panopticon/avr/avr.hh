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

	struct avr_state
	{
		static avr_state mega103(void)  { return avr_state(0x10000 * 2); }
		static avr_state mega161(void)  { return avr_state(0x2000 * 2); }
		static avr_state mega163(void)  { return avr_state(0x2000 * 2); }
		static avr_state mega168(void)  { return avr_state(0x2000 * 2); }
		static avr_state mega16(void)   { return avr_state(0x2000 * 2); }
		static avr_state mega2561(void) { return avr_state(0x20000 * 2); }
		static avr_state mega3250(void) { return avr_state(0x4000 * 2); }
		static avr_state mega3290(void) { return avr_state(0x4000 * 2); }
		static avr_state mega32(void)   { return avr_state(0x4000 * 2); }
		static avr_state mega48(void)   { return avr_state(0x800 * 2); }
		static avr_state mega64(void)   { return avr_state(0x8000 * 2); }
		static avr_state mega8535(void) { return avr_state(0x1000 * 2); }
		static avr_state mega8(void)    { return avr_state(0x1000 * 2); }
		static avr_state mega128(void)  { return avr_state(0x10000 * 2); }
		static avr_state mega162(void)  { return avr_state(0x2000 * 2); }
		static avr_state mega165(void)  { return avr_state(0x2000 * 2); }
		static avr_state mega169(void)  { return avr_state(0x2000 * 2); }
		static avr_state mega2560(void) { return avr_state(0x20000 * 2); }
		static avr_state mega323(void)  { return avr_state(0x4000 * 2); }
		static avr_state mega325(void)  { return avr_state(0x4000 * 2); }
		static avr_state mega329(void)  { return avr_state(0x4000 * 2); }
		static avr_state mega406(void)  { return avr_state(0x5000 * 2); }
		static avr_state mega649(void)  { return avr_state(0x8000 * 2); }
		static avr_state mega8515(void) { return avr_state(0x1000 * 2); }
		static avr_state mega88(void)   { return avr_state(0x1000 * 2); }

		avr_state(size_t fs) : flash_sz(fs) {}

		size_t flash_sz;
	};

	template<>
	struct architecture_traits<avr_tag>
	{
		using token_type = uint16_t;
		using state_type = avr_state;
	};

	template<>
	lvalue temporary(avr_tag);

	template<>
	const std::vector<std::string> &registers(avr_tag);

	template<>
	uint8_t width(std::string n, avr_tag);

	namespace avr
	{
		using sm = sem_state<avr_tag>;
		using sem_action = std::function<void(sm &)>;
		using cg = code_generator<avr_tag>;

		boost::optional<prog_loc> disassemble(po::avr_state const& st, boost::optional<prog_loc>, po::slab, const po::ref&);
	}
}

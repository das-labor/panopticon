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

#include "architecture.hh"

unsigned int ununsed = 0;
std::vector<std::string> regs({"a","b","c","d"});

template<>
po::lvalue po::temporary(test_tag)
{
	return po::variable("t" + std::to_string(ununsed++),16);
}

template<>
const std::vector<std::string>& po::registers(test_tag)
{
	return regs;
}

template<>
uint8_t po::width(std::string n, test_tag)
{
	return 8;
}

template<>
po::lvalue po::temporary(wtest_tag)
{
	return po::variable("t" + std::to_string(ununsed++),16);
}

template<>
const std::vector<std::string>& po::registers(wtest_tag)
{
	return regs;
}

template<>
uint8_t po::width(std::string n, wtest_tag)
{
	return 8;
}

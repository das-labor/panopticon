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

#include <gtest/gtest.h>

#include <panopticon/database.hh>

using namespace po;

TEST(session,pe)
{
	session sess = pe("test.exe");
	auto p = po::projection(sess.dbase->data);

	for(auto x: p)
	{
		std::cout << x.first << ": " << x.second->name() << std::endl;
		for(auto y: x.second->flatten())
		{
			std::cout << "\t" << y.first << ": " << y.second->name() << std::endl;
			slab sl = y.second->filter(slab());
		}

		size_t i = 100;
		while(i--)
			slab sl = x.second->read();
	}
}

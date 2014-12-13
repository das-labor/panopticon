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

#include <panopticon/amd64/amd64.hh>

using namespace po;

TEST(amd64,simple)
{
	region_loc reg = region::wrap("ram",{
0x48,0x11,0x1c,0x25,0xa1,0x1a,0x00,0x00
	});

	po::slab sl = reg->read();
	boost::optional<prog_loc> maybe_proc = amd64::disassemble(boost::none,sl,po::ref{"ram",0});

	ASSERT_TRUE(!!maybe_proc);
	ASSERT_EQ((*maybe_proc)->procedures().size(), 1u);
	ASSERT_EQ((*(*maybe_proc)->procedures().begin())->rev_postorder().size(), 1u);
	ASSERT_EQ((*(*maybe_proc)->procedures().begin())->rev_postorder()[0]->mnemonics().size(), 1u);
}

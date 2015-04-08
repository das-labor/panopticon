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

TEST(amd64,all64)
{
	region_loc reg = region::mmap("ram","lib/test/amd64-testraw");

	po::slab sl = reg->read();
	boost::optional<prog_loc> maybe_proc = amd64::disassemble<64>(boost::none,sl,po::ref{"ram",0});

	ASSERT_TRUE(!!maybe_proc);
	ASSERT_EQ((*maybe_proc)->procedures().size(), 1u);
	ASSERT_EQ((*(*maybe_proc)->procedures().begin())->rev_postorder().size(), 1u);
	ASSERT_EQ((*(*maybe_proc)->procedures().begin())->rev_postorder()[0]->mnemonics().size(), 13u);
}

TEST(amd64,all32)
{
	region_loc reg = region::mmap("ram","lib/test/ia32-testraw");

	po::slab sl = reg->read();
	boost::optional<prog_loc> maybe_proc = amd64::disassemble<32>(boost::none,sl,po::ref{"ram",0});

	ASSERT_TRUE(!!maybe_proc);
	ASSERT_EQ((*maybe_proc)->procedures().size(), 1u);
	ASSERT_EQ((*(*maybe_proc)->procedures().begin())->rev_postorder().size(), 1u);
	ASSERT_TRUE((*(*maybe_proc)->procedures().begin())->rev_postorder()[0]->mnemonics().size() > 0);

	/*for(auto mne: (*(*maybe_proc)->procedures().begin())->rev_postorder()[0]->mnemonics())
		std::cout << mne << std::endl;*/
}

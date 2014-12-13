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

#include <panopticon/mnemonic.hh>

using namespace po;

TEST(mnemonic,marshal)
{
	mnemonic mn1(bound(0,10),"op1","{8:-:eax} nog",{constant(1),variable("a",3)},{
		instr(int_add<rvalue>{constant(1),constant(2)},variable("a",2)),
		instr(int_add<rvalue>{constant(4),constant(2)},variable("a",1)),
		instr(univ_nop<rvalue>{variable("a",2)},variable("a",3))});

	uuid uu;
	archive st1 = marshal(&mn1,uu);

	ASSERT_GT(st1.triples.size(),0u);
	ASSERT_EQ(st1.blobs.size(),0u);
	archive st2 = marshal(&mn1,uu);

	ASSERT_TRUE(st1 == st2);

	rdf::storage store;

	for(auto s: st1.triples)
	{
		std::cerr << s << std::endl;
		store.insert(s);
	}

	mnemonic mn2 = *std::unique_ptr<mnemonic>(unmarshal<mnemonic>(uu,store));

	ASSERT_TRUE(mn2 == mn1);
}

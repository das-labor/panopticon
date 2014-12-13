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

#include <iostream>
#include <algorithm>
#include <iterator>
#include <stdexcept>

#include <boost/graph/isomorphism.hpp>
#include <gtest/gtest.h>
#include <panopticon/program.hh>

using namespace po;
using namespace boost;
using namespace std;

TEST(program,marshal)
{
	proc_loc p1(new procedure("proc1"));
	proc_loc p2(new procedure("proc2"));
	proc_loc p3(new procedure("proc3"));
	proc_loc p4(new procedure("proc4"));
	proc_loc p5(new procedure("proc5"));
	proc_loc p6(new procedure("proc6"));
	prog_loc prog(new program("base","prog"));

	prog.write().insert(p1);
	prog.write().insert(p2);
	prog.write().insert(p3);
	prog.write().insert(p4);
	prog.write().insert(p5);
	prog.write().insert(p6);

	call(prog,p1,p2);
	call(prog,p2,p3);
	call(prog,p3,p1);
	call(prog,p3,p4);
	call(prog,p5,p6);

	call(prog,p1,"printf");

	prog.write().name = "test";

	rdf::storage st;
	save_point(st);

	std::unique_ptr<program> prog2(unmarshal<program>(prog.tag(),st));

	ASSERT_EQ(prog->name, prog2->name);
	ASSERT_EQ(num_vertices(prog2->calls()), num_vertices(prog->calls()));
	ASSERT_EQ(num_edges(prog2->calls()), num_edges(prog->calls()));
}

TEST(program,continue)
{
	FAIL();
}

TEST(program,disassemble)
{
	FAIL();
}

TEST(program,recursive)
{
	FAIL();
}

TEST(program,mutual_recursive)
{
	FAIL();
}

TEST(program,empty)
{
	FAIL();
}

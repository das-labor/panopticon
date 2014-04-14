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
	prog_loc prog(new program("prog"));

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

	std::unique_ptr<program> proc2(unmarshal<program>(prog.tag(),st));

	ASSERT_EQ(prog->name, proc2->name);

	ASSERT_TRUE(boost::isomorphism(prog->calls(),proc2->calls()));
}

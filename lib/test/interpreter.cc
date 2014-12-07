#include <iostream>
#include <algorithm>
#include <iterator>
#include <stdexcept>

#include <gtest/gtest.h>

#include <panopticon/interpreter.hh>
#include <panopticon/procedure.hh>
#include <panopticon/dflow.hh>

using namespace po;
using namespace boost;

TEST(interpreter,concrete_semantics)
{
	environment<domain_traits<concrete_domain>::value_type> env;
	concrete_interpreter i(env);
	instr::operation op = logic_and<rvalue>{constant(true),constant(true)};

	ASSERT_EQ(constant(true), boost::apply_visitor(i,op));
	op = logic_or<rvalue>{constant(true),undefined()};
	ASSERT_EQ(constant(true), boost::apply_visitor(i,op));
	op = int_add<rvalue>{variable("a",8),constant(0)};
	ASSERT_EQ(variable("a",8), boost::apply_visitor(i,op));
	env[variable("b",8)] = constant(33);
	op = int_add<rvalue>{variable("b",8),constant(11)};
	ASSERT_EQ(constant(44), boost::apply_visitor(i,op));
	op = univ_phi<rvalue>{{constant(11),constant(11)}};
	ASSERT_EQ(constant(11), boost::apply_visitor(i,op));
	op = univ_phi<rvalue>{{constant(33),variable("b",8)}};
	ASSERT_EQ(constant(33), boost::apply_visitor(i,op));
	op = int_equal<rvalue>{undefined(),undefined()};
	ASSERT_EQ(undefined(), boost::apply_visitor(i,op));
}

TEST(interpreter,kset_interpreter)
{
	using vx = boost::graph_traits<digraph<boost::variant<bblock_loc,rvalue>,guard>>::vertex_descriptor;

	// b0
	mnemonic mne01(bound(0,1),"mne1","",{},{instr(univ_nop<rvalue>{constant(1)},variable("i",8,0))});
	mnemonic mne02(bound(1,2),"mne1","",{},{instr(univ_nop<rvalue>{undefined()},variable("j",8,0))});
	mnemonic mne03(bound(2,3),"mne1","",{},{instr(logic_neg<rvalue>{variable("j",8,0)},variable("jn",8,0))});
	// b1
	mnemonic mne1(bound(3,4),"mne2","",{},{instr(univ_nop<rvalue>{constant(1)},variable("a",8,0))});
	// b2
	mnemonic mne2(bound(4,5),"mne5","",{},{instr(univ_nop<rvalue>{constant(2)},variable("a",8,1))});
	// b3
	mnemonic mne31(bound(5,6),"mne2","",{},{instr(univ_phi<rvalue>{{variable("a",8,0),variable("a",8,1)}},variable("a",8,2))});
	mnemonic mne32(bound(6,7),"mne2","",{},{instr(int_add<rvalue>{variable("i",8,0),variable("a",8,2)},variable("a",8,3))});

	proc_loc proc(new procedure("proc1"));
	vx b0, b1, b2, b3;

	b0 = insert_vertex(boost::variant<bblock_loc,rvalue>(bblock_loc(new basic_block({mne01,mne02,mne03}))),proc.write().control_transfers);
	b1 = insert_vertex(boost::variant<bblock_loc,rvalue>(bblock_loc(new basic_block({mne1}))),proc.write().control_transfers);
	b2 = insert_vertex(boost::variant<bblock_loc,rvalue>(bblock_loc(new basic_block({mne2}))),proc.write().control_transfers);
	b3 = insert_vertex(boost::variant<bblock_loc,rvalue>(bblock_loc(new basic_block({mne31,mne32}))),proc.write().control_transfers);

	insert_edge(guard(variable("j",8,0),relation::Eq,constant(1)),b0,b1,proc.write().control_transfers);
	insert_edge(guard(variable("jn",8,0),relation::Eq,constant(1)),b0,b2,proc.write().control_transfers);
	insert_edge(guard(),b2,b3,proc.write().control_transfers);
	insert_edge(guard(),b1,b3,proc.write().control_transfers);

	proc.write().entry = get<bblock_loc>(get_vertex(b0,proc->control_transfers));

	//ssa(proc,*dominance_tree(proc),liveness(proc));
	environment<domain_traits<kset_domain<16>>::value_type> env = interpret(proc,kset_domain<16>());

	for(auto p: env)
		std::cout << p.first << " := " << p.second << std::endl;

	// i0
	variable var = variable("i",8,0);
	ASSERT_EQ(1u,env.count(var));
	ASSERT_TRUE(boost::get<kset_set>(&(env.at(var))));
	ASSERT_EQ(boost::get<kset_set>(env.at(var)), kset_set({1}));
	// j0
	var = variable("j",8,0);
	ASSERT_EQ(1u,env.count(var));
	ASSERT_TRUE(boost::get<join_t>(&(env.at(var))));
	// jn0
	var = variable("jn",8,0);
	ASSERT_EQ(1u,env.count(var));
	ASSERT_TRUE(boost::get<join_t>(&(env.at(var))));
	// a0
	var = variable("a",8,0);
	ASSERT_EQ(1u,env.count(var));
	ASSERT_TRUE(boost::get<kset_set>(&(env.at(var))));
	ASSERT_EQ(boost::get<kset_set>(env.at(var)), kset_set({1}));
	// a1
	var = variable("a",8,1);
	ASSERT_EQ(1u,env.count(var));
	ASSERT_TRUE(boost::get<kset_set>(&(env.at(var))));
	ASSERT_EQ(boost::get<kset_set>(env.at(var)), kset_set({2}));
	// a2
	var = variable("a",8,2);
	ASSERT_EQ(1u,env.count(var));
	ASSERT_EQ(boost::get<kset_set>(env.at(var)), kset_set({2,1}));
	// a3
	var = variable("a",8,3);
	ASSERT_EQ(1u,env.count(var));
	ASSERT_EQ(boost::get<kset_set>(env.at(var)), kset_set({2,3}));
}

TEST(interpreter,kset_semantics)
{
	using kset16 = kset_domain<16>;

	kset_value a = overapproximate(constant(1),kset16{});
	ASSERT_EQ(boost::get<kset_set>(a), kset_set({1}));

	a = overapproximate(memory(constant(1),1,LittleEndian,"none"),kset16{});
	ASSERT_TRUE(boost::get<meet_t>(&a));

	a = overapproximate(undefined{},kset16{});
	ASSERT_TRUE(boost::get<join_t>(&a));

	a = overapproximate(variable("a",8,1),kset16{});
	ASSERT_TRUE(boost::get<meet_t>(&a));

	a = supremum(join,join,kset16{});
	ASSERT_TRUE(boost::get<join_t>(&a));
	a = supremum(meet,join,kset16{});
	ASSERT_TRUE(boost::get<join_t>(&a));
	a = supremum(join,meet,kset16{});
	ASSERT_TRUE(boost::get<join_t>(&a));
	a = supremum(meet,meet,kset16{});
	ASSERT_TRUE(boost::get<meet_t>(&a));
	a = supremum(kset_set({1}),join,kset16{});
	ASSERT_TRUE(boost::get<join_t>(&a));
	a = supremum(kset_set({1}),meet,kset16{});
	ASSERT_EQ(boost::get<kset_set>(a),kset_set({1}));
	a = supremum(join,kset_set({1}),kset16{});
	ASSERT_TRUE(boost::get<join_t>(&a));
	a = supremum(meet,kset_set({1}),kset16{});
	ASSERT_EQ(boost::get<kset_set>(a),kset_set({1}));

	a = supremum(kset_set({1}),kset_set({2}),kset16{});
	ASSERT_EQ(boost::get<kset_set>(a),kset_set({1,2}));
	a = supremum(kset_set({2}),kset_set({2}),kset16{});
	ASSERT_EQ(boost::get<kset_set>(a),kset_set({2}));
	a = supremum(kset_set({1,2,3,4,5,6,7,8}),kset_set({9,10,11,12,13,14,15,16}),kset16{});
	ASSERT_EQ(boost::get<kset_set>(a),kset_set({1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16}));
	a = supremum(kset_set({1,2,3,4,5,6,7,8}),kset_set({9,10,11,12,13,14,15,16,17}),kset16{});
	ASSERT_TRUE(boost::get<join_t>(&a));
}

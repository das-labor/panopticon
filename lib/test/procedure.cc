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
#include <boost/graph/graphviz.hpp>

#include <gtest/gtest.h>

#include <panopticon/procedure.hh>
#include <panopticon/disassembler.hh>

#include "architecture.hh"

using namespace po;
using namespace boost;

struct proc_writer
{
	using edge_descriptor = boost::graph_traits<decltype(procedure::control_transfers)>::edge_descriptor;
	using vertex_descriptor = boost::graph_traits<decltype(procedure::control_transfers)>::vertex_descriptor;

	proc_writer(proc_loc p) : proc(p) {}

	void operator()(std::ostream& os, vertex_descriptor vx) const
	{
		auto n = get_vertex(vx,proc->control_transfers);

		if(get<bblock_loc>(&n))
			os << "[label=\"" << get<bblock_loc>(n)->area() << "\"]";
		else
			os << "[label=\"" << get<rvalue>(n) << "\"]";
	}

	void operator()(std::ostream& os, edge_descriptor e) const
	{
		assert(false);
	}

private:
	proc_loc proc;
};

class disassembler_mockup
{
public:
	using iter = po::slab::iterator;
	using token = architecture_traits<test_tag>::token_type;

	disassembler_mockup(const std::map<typename po::architecture_traits<test_tag>::token_type,po::sem_state<test_tag>> &states)
	: m_states(states) {}

	boost::optional<std::pair<iter,sem_state<test_tag>>> try_match(iter begin, iter end,sem_state<test_tag> const& in_state) const
	{
		if(begin == end)
			return boost::none;

		auto i = m_states.find(**begin);

		if(i != m_states.end())
		{
			sem_state<test_tag> state = in_state;
			state.mnemonics = i->second.mnemonics;
			state.jumps = i->second.jumps;

			return boost::make_optional(std::make_pair(begin + std::accumulate(state.mnemonics.begin(),state.mnemonics.end(),0,[](size_t acc, const po::mnemonic &m) { return icl::size(m.area) + acc; }),state));
		}
		else
			return boost::none;
	}

private:
	std::map<typename po::architecture_traits<test_tag>::token_type,po::sem_state<test_tag>> m_states;
};

TEST(procedure,add_single)
{
	std::vector<typename po::architecture_traits<test_tag>::token_type> bytes = {0};
	std::map<typename po::architecture_traits<test_tag>::token_type,po::sem_state<test_tag>> states;

	{
		po::sem_state<test_tag> st(0,'a');
		st.mnemonic(1,"test");
		states.insert(std::make_pair(0,st));
	}

	disassembler_mockup mockup(states);
	boost::optional<po::proc_loc> maybe_proc = po::procedure::disassemble<test_tag,disassembler_mockup>(boost::none,mockup,'a',slab(bytes.data(),bytes.size()),0);
	ASSERT_TRUE(!!maybe_proc);
	proc_loc proc = *maybe_proc;

	ASSERT_EQ(proc->rev_postorder().size(), 1u);

	po::bblock_loc bb = *proc->rev_postorder().begin();

	ASSERT_EQ(bb->mnemonics().size(), 1u);
	ASSERT_EQ(bb->mnemonics()[0].opcode, "test");
	ASSERT_EQ(bb->mnemonics()[0].area, po::bound(0,1));
	ASSERT_EQ(po::bound(0,1), bb->area());
	ASSERT_EQ(bb, *(proc->entry));
	ASSERT_EQ(num_edges(proc->control_transfers), 0u);
	ASSERT_EQ(num_vertices(proc->control_transfers), 1u);
	ASSERT_NE(proc->name, "");
}

TEST(procedure,continuous)
{
	std::vector<typename po::architecture_traits<test_tag>::token_type> bytes({0,1,2,3,4,5});
	std::map<typename po::architecture_traits<test_tag>::token_type,po::sem_state<test_tag>> states;
	auto add = [&](po::offset p, const std::string &n) -> void
	{
		po::sem_state<test_tag> st(p,'a');
		st.mnemonic(1,n);
		st.jump(p+1);
		states.insert(std::make_pair(p,st));
	};
	auto check = [&](const po::mnemonic &m, const std::string &n, po::offset p) -> void
	{
		ASSERT_EQ(m.opcode, n);
		ASSERT_EQ(m.operands.size(), 0u);
		ASSERT_EQ(m.instructions.size(), 0u);
		ASSERT_EQ(m.area, po::bound(p,p+1));
	};

	add(0,"test0");
	add(1,"test1");
	add(2,"test2");
	add(3,"test3");
	add(4,"test4");
	add(5,"test5");

	disassembler_mockup mockup(states);
	boost::optional<po::proc_loc> maybe_proc = po::procedure::disassemble<test_tag,disassembler_mockup>(boost::none,mockup,'a',slab(bytes.data(),bytes.size()),0);
	ASSERT_TRUE(!!maybe_proc);
	proc_loc proc = *maybe_proc;

	ASSERT_TRUE(!!proc->entry);
	ASSERT_EQ(proc->rev_postorder().size(), 1u);

	po::bblock_loc bb = *proc->rev_postorder().begin();

	ASSERT_EQ(bb->mnemonics().size(), 6u);

	check(bb->mnemonics()[0],"test0",0);
	check(bb->mnemonics()[1],"test1",1);
	check(bb->mnemonics()[2],"test2",2);
	check(bb->mnemonics()[3],"test3",3);
	check(bb->mnemonics()[4],"test4",4);
	check(bb->mnemonics()[5],"test5",5);

	auto ep = edges(proc->control_transfers);
	using edge_descriptor = boost::graph_traits<decltype(procedure::control_transfers)>::edge_descriptor;
	ASSERT_TRUE(std::all_of(ep.first,ep.second,[&](edge_descriptor e) { try { get_edge(e,proc->control_transfers); return true; } catch(...) { return false; } }));

	auto in_p = in_edges(find_node(variant<bblock_loc,rvalue>(bb),proc->control_transfers),proc->control_transfers);
	auto out_p = out_edges(find_node(variant<bblock_loc,rvalue>(bb),proc->control_transfers),proc->control_transfers);

	ASSERT_EQ(distance(in_p.first,in_p.second), 0);
	ASSERT_EQ(distance(out_p.first,out_p.second), 1);
	ASSERT_TRUE(get_edge(*out_p.first,proc->control_transfers).relations.empty());
	ASSERT_TRUE(is_constant(get<rvalue>(get_vertex(target(*out_p.first,proc->control_transfers),proc->control_transfers))));
	ASSERT_EQ(to_constant(get<rvalue>(get_vertex(target(*out_p.first,proc->control_transfers),proc->control_transfers))).content(), 6u);
	ASSERT_EQ(bb->area(), po::bound(0,6));
	ASSERT_EQ(bb, *(proc->entry));
	ASSERT_NE(proc->name, "");
}

TEST(procedure,branch)
{
	std::vector<typename po::architecture_traits<test_tag>::token_type> bytes({0,1,2});
	std::map<typename po::architecture_traits<test_tag>::token_type,po::sem_state<test_tag>> states;
	auto add = [&](po::offset p, const std::string &n, po::offset b1, boost::optional<po::offset> b2) -> void
	{
		po::sem_state<test_tag> st(p,'a');
		st.mnemonic(1,n);
		st.jump(b1);
		if(b2)
			st.jump(*b2);
		states.insert(std::make_pair(p,st));
	};
	auto check = [&](const po::mnemonic &m, const std::string &n, po::offset p) -> void
	{
		ASSERT_EQ(m.opcode, n);
		ASSERT_TRUE(m.operands.empty());
		ASSERT_TRUE(m.instructions.empty());
		ASSERT_EQ(m.area, po::bound(p,p+1));
	};

	add(0,"test0",1,2);
	add(1,"test1",3,boost::none);
	add(2,"test2",1,boost::none);

	disassembler_mockup mockup(states);
	boost::optional<po::proc_loc> maybe_proc = po::procedure::disassemble<test_tag,disassembler_mockup>(boost::none,mockup,'a',slab(bytes.data(),bytes.size()),0);
	ASSERT_TRUE(!!maybe_proc);
	proc_loc proc = *maybe_proc;

	ASSERT_TRUE(!!proc->entry);
	ASSERT_EQ(proc->rev_postorder().size(), 3u);

	auto i0 = std::find_if(proc->rev_postorder().begin(),proc->rev_postorder().end(),[&](po::bblock_loc bb) { return bb->area().lower() == 0; });
	auto i1 = std::find_if(proc->rev_postorder().begin(),proc->rev_postorder().end(),[&](po::bblock_loc bb) { return bb->area().lower() == 1; });
	auto i2 = std::find_if(proc->rev_postorder().begin(),proc->rev_postorder().end(),[&](po::bblock_loc bb) { return bb->area().lower() == 2; });

	ASSERT_NE(i0, proc->rev_postorder().end());
	ASSERT_NE(i1, proc->rev_postorder().end());
	ASSERT_NE(i2, proc->rev_postorder().end());

	po::bblock_loc bb0 = *i0;
	po::bblock_loc bb1 = *i1;
	po::bblock_loc bb2 = *i2;

	ASSERT_EQ(bb0->mnemonics().size(), 1u);
	ASSERT_EQ(bb1->mnemonics().size(), 1u);
	ASSERT_EQ(bb2->mnemonics().size(), 1u);

	auto in0_p = in_edges(find_node(variant<bblock_loc,rvalue>(bb0),proc->control_transfers),proc->control_transfers);
	auto out0_p = out_edges(find_node(variant<bblock_loc,rvalue>(bb0),proc->control_transfers),proc->control_transfers);

	ASSERT_EQ(distance(in0_p.first,in0_p.second), 0);
	check(bb0->mnemonics()[0],"test0",0);
	ASSERT_EQ(distance(out0_p.first,out0_p.second), 2);

	auto in1_p = in_edges(find_node(variant<bblock_loc,rvalue>(bb1),proc->control_transfers),proc->control_transfers);
	auto out1_p = out_edges(find_node(variant<bblock_loc,rvalue>(bb1),proc->control_transfers),proc->control_transfers);

	ASSERT_EQ(distance(in1_p.first,in1_p.second), 2);
	check(bb1->mnemonics()[0],"test1",1);
	ASSERT_EQ(distance(out1_p.first,out1_p.second), 1);

	auto in2_p = in_edges(find_node(variant<bblock_loc,rvalue>(bb2),proc->control_transfers),proc->control_transfers);
	auto out2_p = out_edges(find_node(variant<bblock_loc,rvalue>(bb2),proc->control_transfers),proc->control_transfers);

	ASSERT_EQ(distance(in2_p.first,in2_p.second), 1);
	check(bb2->mnemonics()[0],"test2",2);
	ASSERT_EQ(distance(out2_p.first,out2_p.second), 1);
}

TEST(procedure,loop)
{
	std::vector<typename po::architecture_traits<test_tag>::token_type> bytes({0,1,2});
	std::map<typename po::architecture_traits<test_tag>::token_type,po::sem_state<test_tag>> states;
	auto add = [&](po::offset p, const std::string &n, po::offset b1) -> void
	{
		po::sem_state<test_tag> st(p,'a');
		st.mnemonic(1,n);
		st.jump(b1);
		states.insert(std::make_pair(p,st));
	};
	auto check = [&](const po::mnemonic &m, const std::string &n, po::offset p) -> void
	{
		ASSERT_EQ(m.opcode, n);
		ASSERT_TRUE(m.operands.empty());
		ASSERT_TRUE(m.instructions.empty());
		ASSERT_EQ(m.area, po::bound(p,p+1));
	};

	add(0,"test0",1);
	add(1,"test1",2);
	add(2,"test2",0);

	disassembler_mockup mockup(states);
	boost::optional<po::proc_loc> maybe_proc = po::procedure::disassemble<test_tag,disassembler_mockup>(boost::none,mockup,'a',slab(bytes.data(),bytes.size()),0);
	ASSERT_TRUE(!!maybe_proc);
	proc_loc proc = *maybe_proc;

	ASSERT_EQ(proc->rev_postorder().size(), 1u);

	po::bblock_loc bb = *proc->rev_postorder().begin();

	ASSERT_EQ(bb->mnemonics().size(), 3u);

	check(bb->mnemonics()[0],"test0",0);
	check(bb->mnemonics()[1],"test1",1);
	check(bb->mnemonics()[2],"test2",2);

	auto in_p = in_edges(find_node(variant<bblock_loc,rvalue>(bb),proc->control_transfers),proc->control_transfers);
	auto out_p = out_edges(find_node(variant<bblock_loc,rvalue>(bb),proc->control_transfers),proc->control_transfers);

	ASSERT_EQ(distance(in_p.first,in_p.second), 1);
	ASSERT_EQ(distance(out_p.first,out_p.second), 1);
}

TEST(procedure,empty)
{
	std::vector<typename po::architecture_traits<test_tag>::token_type> bytes({});
	std::map<typename po::architecture_traits<test_tag>::token_type,po::sem_state<test_tag>> states;
	disassembler_mockup mockup(states);
	boost::optional<po::proc_loc> maybe_proc = po::procedure::disassemble<test_tag,disassembler_mockup>(boost::none,mockup,'a',slab(bytes.data(),bytes.size()),0);
	ASSERT_TRUE(!maybe_proc);
}

TEST(procedure,refine)
{
	std::vector<typename po::architecture_traits<test_tag>::token_type> bytes({0,1,2,3});
	std::map<typename po::architecture_traits<test_tag>::token_type,po::sem_state<test_tag>> states;
	auto add = [&](po::offset p, size_t l, const std::string &n, po::offset b1) -> void
	{
		po::sem_state<test_tag> st(p,'a');
		st.mnemonic(l,n);
		st.jump(b1);
		states.insert(std::make_pair(p,st));
	};
	/*auto check = [&](const po::mnemonic &m, const std::string &n, po::bound p) -> void
	{
		ASSERT_EQ(m.opcode, n);
		ASSERT_TRUE(m.operands.empty());
		ASSERT_TRUE(m.instructions.empty());
		ASSERT_EQ(m.area, p);
	};*/

	/*
	 * test0
	 *  -"-  test1
	 * test2
	 */
	add(0,2,"test0",2);
	add(2,1,"test2",1);
	add(1,1,"test1",2);

	disassembler_mockup mockup(states);
	boost::optional<po::proc_loc> maybe_proc = po::procedure::disassemble<test_tag,disassembler_mockup>(boost::none,mockup,'a',slab(bytes.data(),bytes.size()),0);
	ASSERT_TRUE(!!maybe_proc);
	proc_loc proc = *maybe_proc;
	boost::write_graphviz(std::cout,proc->control_transfers,proc_writer(proc));

	// XXX: Disabled until functionality is needed
	/*
	ASSERT_EQ(proc->rev_postorder().size(), 2);

	auto i0 = std::find_if(proc->rev_postorder().begin(),proc->rev_postorder().end(),[&](po::bblock_loc bb) { return bb->area().lower() == 0; });
	auto i1 = std::find_if(proc->rev_postorder().begin(),proc->rev_postorder().end(),[&](po::bblock_loc bb) { return bb->area().lower() == 1; });

	ASSERT_NE(i0, proc->rev_postorder().end());
	ASSERT_NE(i1, proc->rev_postorder().end());

	po::bblock_loc bb0 = *i0;
	po::bblock_loc bb1 = *i1;

	ASSERT_EQ(bb0->mnemonics().size(), 1);
	ASSERT_EQ(bb1->mnemonics().size(), 2);

	check(bb0->mnemonics()[0],"test0",po::bound(0,2));
	check(bb1->mnemonics()[0],"test1",po::bound(0,2));
	check(bb1->mnemonics()[1],"test2",po::bound(2,3));

	auto in0_p = in_edges(find_node(variant<bblock_loc,rvalue>(bb0),proc->control_transfers),proc->control_transfers);
	auto out0_p = out_edges(find_node(variant<bblock_loc,rvalue>(bb0),proc->control_transfers),proc->control_transfers);

	ASSERT_EQ(distance(in0_p.first,in0_p.second), 0);
	ASSERT_EQ(distance(out0_p.first,out0_p.second), 1);

	auto in1_p = in_edges(find_node(variant<bblock_loc,rvalue>(bb1),proc->control_transfers),proc->control_transfers);
	auto out1_p = out_edges(find_node(variant<bblock_loc,rvalue>(bb1),proc->control_transfers),proc->control_transfers);

	ASSERT_EQ(distance(in1_p.first,in1_p.second), 2);
	ASSERT_EQ(distance(out1_p.first,out1_p.second), 1);*/
}

TEST(procedure,continue)
{
	rdf::storage store;
	po::proc_loc proc(new po::procedure(""));
	po::mnemonic mne0(po::bound(0,1),"test0","",{},{});
	po::mnemonic mne1(po::bound(1,2),"test1","",{},{});
	po::mnemonic mne2(po::bound(2,3),"test2","",{},{});
	po::mnemonic mne3(po::bound(6,7),"test6","",{},{});
	po::bblock_loc bb0(new po::basic_block());
	po::bblock_loc bb1(new po::basic_block());
	po::bblock_loc bb2(new po::basic_block());

	insert_vertex(variant<bblock_loc,rvalue>(bb0),proc.write().control_transfers);
	insert_vertex(variant<bblock_loc,rvalue>(bb1),proc.write().control_transfers);
	insert_vertex(variant<bblock_loc,rvalue>(bb2),proc.write().control_transfers);

	save_point(store);

	find_node(variant<bblock_loc,rvalue>(bb0),proc->control_transfers);
	find_node(variant<bblock_loc,rvalue>(bb1),proc->control_transfers);
	find_node(variant<bblock_loc,rvalue>(bb1),proc->control_transfers);

	bb0.write().mnemonics().push_back(mne0);
	bb0.write().mnemonics().push_back(mne1);
	bb1.write().mnemonics().push_back(mne2);
	bb2.write().mnemonics().push_back(mne3);

	unconditional_jump(proc,bb2,po::constant(40));
	unconditional_jump(proc,bb0,bb1);
	unconditional_jump(proc,bb0,bb2);

	proc.write().entry = bb0;

	std::vector<typename po::architecture_traits<test_tag>::token_type> bytes({0,1,2,0,0,0,6,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,40,41,42});
	std::map<typename po::architecture_traits<test_tag>::token_type,po::sem_state<test_tag>> states;
	auto add = [&](po::offset p, const std::string &n, boost::optional<po::offset> b1, boost::optional<po::offset> b2) -> void
	{
		po::sem_state<test_tag> st(p,'a');
		st.mnemonic(1,n);
		if(b1)
			st.jump(*b1);
		if(b2)
			st.jump(*b2);

		states.insert(std::make_pair(p,st));
	};
	auto check = [&](const po::mnemonic &m, const std::string &n, po::offset p) -> void
	{
		ASSERT_EQ(m.opcode, n);
		ASSERT_TRUE(m.operands.empty());
		ASSERT_TRUE(m.instructions.empty());
		ASSERT_EQ(m.area, po::bound(p,p+1));
	};

	add(0,"test0",1,boost::none);
	add(1,"test1",2,6);
	add(2,"test2",boost::none,boost::none);
	add(6,"test6",40,boost::none);

	add(40,"test40",41,boost::none);
	add(41,"test41",42,boost::none);
	add(42,"test42",55,make_optional<offset>(0));

	disassembler_mockup mockup(states);
	ASSERT_TRUE(!!proc->entry);

	boost::optional<proc_loc> maybe_proc = po::procedure::disassemble<test_tag,disassembler_mockup>(proc,mockup,'a',slab(bytes.data(),bytes.size()),40);
	ASSERT_TRUE(!!maybe_proc);

	proc = *maybe_proc;

	ASSERT_TRUE(!!proc->entry);
	ASSERT_EQ(proc->rev_postorder().size(), 4u);

	auto i0 = std::find_if(proc->rev_postorder().begin(),proc->rev_postorder().end(),[&](po::bblock_loc bb) { return bb->area().lower() == 0; });
	auto i1 = std::find_if(proc->rev_postorder().begin(),proc->rev_postorder().end(),[&](po::bblock_loc bb) { return bb->area().lower() == 2; });
	auto i2 = std::find_if(proc->rev_postorder().begin(),proc->rev_postorder().end(),[&](po::bblock_loc bb) { return bb->area().lower() == 6; });
	auto i3 = std::find_if(proc->rev_postorder().begin(),proc->rev_postorder().end(),[&](po::bblock_loc bb) { return bb->area().lower() == 40; });

	ASSERT_NE(i0, proc->rev_postorder().end());
	ASSERT_NE(i1, proc->rev_postorder().end());
	ASSERT_NE(i2, proc->rev_postorder().end());
	ASSERT_NE(i3, proc->rev_postorder().end());

	po::bblock_loc bbo0 = *i0;
	po::bblock_loc bbo1 = *i1;
	po::bblock_loc bbo2 = *i2;
	po::bblock_loc bbo3 = *i3;
	auto ct = proc->control_transfers;

	auto in0_p = in_edges(find_node(variant<bblock_loc,rvalue>(bbo0),proc->control_transfers),proc->control_transfers);
	auto out0_p = out_edges(find_node(variant<bblock_loc,rvalue>(bbo0),proc->control_transfers),proc->control_transfers);

	ASSERT_EQ(distance(in0_p.first,in0_p.second), 1);
	ASSERT_TRUE(get<bblock_loc>(get_vertex(source(*in0_p.first,ct),ct)) == bbo3);
	ASSERT_EQ(bbo0->mnemonics().size(), 2u);
	check(bbo0->mnemonics()[0],"test0",0);
	check(bbo0->mnemonics()[1],"test1",1);
	ASSERT_EQ(distance(out0_p.first,out0_p.second), 2);
	ASSERT_TRUE(get<bblock_loc>(get_vertex(target(*out0_p.first,ct),ct)) == bbo1 || get<bblock_loc>(get_vertex(target(*out0_p.first,ct),ct)) == bbo2);
	ASSERT_TRUE(get<bblock_loc>(get_vertex(target(*(out0_p.first+1),ct),ct)) == bbo1 || get<bblock_loc>(get_vertex(target(*(out0_p.first+1),ct),ct)) == bbo2);

	auto in1_p = in_edges(find_node(variant<bblock_loc,rvalue>(bbo1),proc->control_transfers),proc->control_transfers);
	auto out1_p = out_edges(find_node(variant<bblock_loc,rvalue>(bbo1),proc->control_transfers),proc->control_transfers);

	ASSERT_EQ(distance(in1_p.first,in1_p.second), 1);
	ASSERT_TRUE(get<bblock_loc>(get_vertex(source(*in1_p.first,ct),ct)) == bbo0);
	ASSERT_EQ(bbo1->mnemonics().size(), 1u);
	check(bbo1->mnemonics()[0],"test2",2);
	ASSERT_EQ(distance(out1_p.first,out1_p.second), 0);

	auto in2_p = in_edges(find_node(variant<bblock_loc,rvalue>(bbo2),proc->control_transfers),proc->control_transfers);
	auto out2_p = out_edges(find_node(variant<bblock_loc,rvalue>(bbo2),proc->control_transfers),proc->control_transfers);

	ASSERT_EQ(distance(in2_p.first,in2_p.second), 1);
	ASSERT_TRUE(get<bblock_loc>(get_vertex(source(*in2_p.first,ct),ct)) == bbo0);
	ASSERT_EQ(bbo2->mnemonics().size(), 1u);
	check(bbo2->mnemonics()[0],"test6",6);
	ASSERT_EQ(distance(out2_p.first,out2_p.second), 1);
	ASSERT_TRUE(get<bblock_loc>(get_vertex(target(*out2_p.first,ct),ct)) == bbo3);

	auto in3_p = in_edges(find_node(variant<bblock_loc,rvalue>(bbo3),proc->control_transfers),proc->control_transfers);
	auto out3_p = out_edges(find_node(variant<bblock_loc,rvalue>(bbo3),proc->control_transfers),proc->control_transfers);

	ASSERT_EQ(distance(in3_p.first,in3_p.second), 1);
	ASSERT_TRUE(get<bblock_loc>(get_vertex(source(*in3_p.first,ct),ct)) == bbo2);
	ASSERT_EQ(bbo3->mnemonics().size(), 3u);
	check(bbo3->mnemonics()[0],"test40",40);
	check(bbo3->mnemonics()[1],"test41",41);
	check(bbo3->mnemonics()[2],"test42",42);
	ASSERT_EQ(distance(out3_p.first,out3_p.second), 2);
	ASSERT_TRUE((get<bblock_loc>(&get_vertex(target(*out3_p.first,ct),ct)) && get<bblock_loc>(get_vertex(target(*out3_p.first,ct),ct)) == bbo0) ||
							(get<rvalue>(&get_vertex(target(*out3_p.first,ct),ct)) && to_constant(get<rvalue>(get_vertex(target(*out3_p.first,ct),ct))).content() == 55));
	ASSERT_TRUE((get<bblock_loc>(&get_vertex(target(*(out3_p.first+1),ct),ct)) && get<bblock_loc>(get_vertex(target(*(out3_p.first+1),ct),ct)) == bbo0) ||
							(get<rvalue>(&get_vertex(target(*(out3_p.first+1),ct),ct)) && to_constant(get<rvalue>(get_vertex(target(*(out3_p.first+1),ct),ct))).content() == 55));

	ASSERT_EQ(*(proc->entry), bbo0);
}

TEST(procedure,entry_split)
{
	po::proc_loc proc(new po::procedure(""));
	po::mnemonic mne0(po::bound(0,1),"test0","",{},{});
	po::mnemonic mne1(po::bound(1,2),"test1","",{},{});
	po::bblock_loc bb0(new po::basic_block());

	insert_vertex(variant<bblock_loc,rvalue>(bb0),proc.write().control_transfers);

	bb0.write().mnemonics().push_back(mne0);
	bb0.write().mnemonics().push_back(mne1);

	unconditional_jump(proc,bb0,po::constant(2));

	proc.write().entry = bb0;

	std::vector<typename po::architecture_traits<test_tag>::token_type> bytes({0,1,2});
	std::map<typename po::architecture_traits<test_tag>::token_type,po::sem_state<test_tag>> states;
	auto add = [&](po::offset p, const std::string &n, boost::optional<po::offset> b1, boost::optional<po::offset> b2) -> void
	{
		po::sem_state<test_tag> st(p,'a');
		st.mnemonic(1,n);
		if(b1)
			st.jump(*b1);
		if(b2)
			st.jump(*b2);

		states.insert(std::make_pair(p,st));
	};
	auto check = [&](const po::mnemonic &m, const std::string &n, po::offset p) -> void
	{
		ASSERT_EQ(m.opcode, n);
		ASSERT_TRUE(m.operands.empty());
		ASSERT_TRUE(m.instructions.empty());
		ASSERT_EQ(m.area, po::bound(p,p+1));
	};

	add(0,"test0",1,boost::none);
	add(1,"test1",2,boost::none);

	add(2,"test2",1,boost::none);

	disassembler_mockup mockup(states);
	boost::optional<proc_loc> maybe_proc = po::procedure::disassemble<test_tag,disassembler_mockup>(proc,mockup,'a',slab(bytes.data(),bytes.size()),2);
	ASSERT_TRUE(!!maybe_proc);

	proc = *maybe_proc;

	ASSERT_EQ(proc->rev_postorder().size(), 2u);

	auto i0 = std::find_if(proc->rev_postorder().begin(),proc->rev_postorder().end(),[&](po::bblock_loc bb) { return bb->area().lower() == 0; });
	auto i1 = std::find_if(proc->rev_postorder().begin(),proc->rev_postorder().end(),[&](po::bblock_loc bb) { return bb->area().lower() == 1; });

	ASSERT_NE(i0, proc->rev_postorder().end());
	ASSERT_NE(i1, proc->rev_postorder().end());

	po::bblock_loc bbo0 = *i0;
	po::bblock_loc bbo1 = *i1;

	ASSERT_EQ(*(proc->entry), bbo0);
	ASSERT_EQ(bbo0->mnemonics().size(), 1u);
	check(bbo0->mnemonics()[0],"test0",0);
	ASSERT_EQ(bbo1->mnemonics().size(), 2u);
}

/*
 *   bb0 ----+
 *    |  \   |
 *   bb1  a  |
 *   /  \    |
 *   bb2 \   |
 *   \   /   |
 * +-bb3---2 |
 * +/ |      |
 *    bb4----+
 */
TEST(procedure,marshal)
{
	bblock_loc bb0(new basic_block({mnemonic(bound(0,5),"test","",{},{})}));
	bblock_loc bb1(new basic_block({mnemonic(bound(5,10),"test","",{},{})}));
	bblock_loc bb2(new basic_block({mnemonic(bound(10,12),"test","",{},{})}));
	bblock_loc bb3(new basic_block({mnemonic(bound(12,20),"test","",{},{})}));
	bblock_loc bb4(new basic_block({mnemonic(bound(20,21),"test","",{},{})}));
	rvalue rv1 = variable("a",8);
	rvalue rv2 = constant(42);
	proc_loc proc(new procedure("p1"));

	auto vx0 = insert_vertex<variant<bblock_loc,rvalue>,guard>(bb0,proc.write().control_transfers);
	auto vx1 = insert_vertex<variant<bblock_loc,rvalue>,guard>(bb1,proc.write().control_transfers);
	auto vx2 = insert_vertex<variant<bblock_loc,rvalue>,guard>(bb2,proc.write().control_transfers);
	auto vx3 = insert_vertex<variant<bblock_loc,rvalue>,guard>(bb3,proc.write().control_transfers);
	auto vx4 = insert_vertex<variant<bblock_loc,rvalue>,guard>(bb4,proc.write().control_transfers);
	auto vx5 = insert_vertex<variant<bblock_loc,rvalue>,guard>(rv1,proc.write().control_transfers);
	auto vx6 = insert_vertex<variant<bblock_loc,rvalue>,guard>(rv2,proc.write().control_transfers);

	insert_edge(guard(),vx0,vx1,proc.write().control_transfers);
	insert_edge(guard(),vx0,vx5,proc.write().control_transfers);
	insert_edge(guard(),vx1,vx2,proc.write().control_transfers);
	insert_edge(guard(),vx2,vx3,proc.write().control_transfers);
	insert_edge(guard(),vx1,vx3,proc.write().control_transfers);
	insert_edge(guard(),vx3,vx3,proc.write().control_transfers);
	insert_edge(guard(),vx3,vx6,proc.write().control_transfers);
	insert_edge(guard(),vx3,vx4,proc.write().control_transfers);
	insert_edge(guard(),vx4,vx0,proc.write().control_transfers);

	proc.write().entry = bb0;

	rdf::storage st;
	save_point(st);

	for(auto s: st.all())
		std::cout << s << std::endl;

	proc_loc p2(proc.tag(),unmarshal<procedure>(proc.tag(),st));

	ASSERT_EQ(proc->name, p2->name);
	ASSERT_TRUE(**proc->entry == **p2->entry);
	ASSERT_EQ(num_vertices(p2->control_transfers), num_vertices(proc->control_transfers));
	ASSERT_EQ(num_edges(p2->control_transfers), num_edges(proc->control_transfers));
	ASSERT_EQ(proc->rev_postorder().size(), p2->rev_postorder().size());
}

using sw = po::sem_state<wtest_tag>&;
TEST(procedure,wide_token)
{
	std::vector<uint8_t> _buf = {0x22,0x11, 0x44,0x33, 0x44,0x55, 0x44,0x55};
	po::slab buf(_buf.data(),_buf.size());
	po::disassembler<wtest_tag> dec;

	dec[0x1122] = [](sw s)
	{
		s.mnemonic(2,"A");
		s.jump(s.address + 2);
		return true;
	};

	dec[0x3344] = [](sw s)
	{
		s.mnemonic(2,"B");
		s.jump(s.address + 2);
		s.jump(s.address + 4);
		return true;
	};

	dec[0x5544] = [](sw s)
	{
		s.mnemonic(2, "C");
		return true;
	};

	boost::optional<proc_loc> maybe_proc = procedure::disassemble<wtest_tag,po::disassembler<wtest_tag>>(boost::none, dec,'a', buf, 0);
	ASSERT_TRUE(!!maybe_proc);
	proc_loc proc = *maybe_proc;

	EXPECT_EQ(num_vertices(proc->control_transfers), 3u);
	EXPECT_EQ(num_edges(proc->control_transfers), 2u);

	using vx_desc = digraph<boost::variant<bblock_loc,rvalue>,guard>::vertex_descriptor;
	auto p = vertices(proc->control_transfers);
	size_t sz = std::count_if(p.first,p.second,[&](const vx_desc& v)
	{
		try
		{
			bblock_loc bb = boost::get<bblock_loc>(get_vertex(v,proc->control_transfers));
			return bb->area() == po::bound(0,4) && bb->mnemonics().size() == 2;
		}
		catch(const boost::bad_get&)
		{
			return false;
		}
	});
	EXPECT_EQ(sz, 1u);
	sz = std::count_if(p.first,p.second,[&](const vx_desc& v)
	{
		try
		{
			bblock_loc bb = boost::get<bblock_loc>(get_vertex(v,proc->control_transfers));
			return bb->area() == po::bound(4,6) && bb->mnemonics().size() == 1;
		}
		catch(const boost::bad_get&)
		{
			return false;
		}
	});
	EXPECT_EQ(sz, 1u);
	sz = std::count_if(p.first,p.second,[&](const vx_desc& v)
	{
		try
		{
			bblock_loc bb = boost::get<bblock_loc>(get_vertex(v,proc->control_transfers));
			return bb->area() == po::bound(6,8) && bb->mnemonics().size() == 1;
		}
		catch(const boost::bad_get&)
		{
			return false;
		}
	});
	EXPECT_EQ(sz, 1u);
}

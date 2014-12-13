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

#include <gtest/gtest.h>

#include <panopticon/dflow.hh>
#include <panopticon/procedure.hh>

using namespace po;
using namespace boost;

struct dflow : public ::testing::Test
{
	using vx = boost::graph_traits<digraph<boost::variant<bblock_loc,rvalue>,guard>>::vertex_descriptor;

	dflow(void) : proc(new procedure("proc"))
	{
		// b0
		mnemonic mne1(bound(0,1),"mne1","",{},{instr(univ_nop<rvalue>{constant(1)},variable("i",8))});
		// b1
		mnemonic mne2(bound(1,2),"mne2","",{},{instr(univ_nop<rvalue>{undefined()},variable("a",8))});
		mnemonic mne3(bound(2,3),"mne3","",{},{instr(univ_nop<rvalue>{undefined()},variable("c",8))});
		mnemonic mne4(bound(3,4),"mne4","",{},{instr(int_less<rvalue>{variable("a",8),variable("c",8)},memory(undefined(),1,LittleEndian,"none"))});

		// b2
		mnemonic mne5(bound(4,5),"mne5","",{},{instr(univ_nop<rvalue>{undefined()},variable("b",8))});
		mnemonic mne6(bound(5,6),"mne6","",{},{instr(univ_nop<rvalue>{undefined()},variable("c",8))});
		mnemonic mne7(bound(6,7),"mne7","",{},{instr(univ_nop<rvalue>{undefined()},variable("d",8))});
		// b3
		mnemonic mne8(bound(7,8),"mne8","",{},{instr(int_add<rvalue>{variable("a",8),variable("b",8)},variable("y",8))});
		mnemonic mne9(bound(8,9),"mne9","",{},{instr(int_add<rvalue>{variable("c",8),variable("d",8)},variable("z",8))});
		mnemonic mne10(bound(9,10),"mne10","",{},{instr(int_add<rvalue>{variable("i",8),constant(1)},variable("i",8))});
		mnemonic mne11(bound(10,11),"mne11","",{},{instr(int_less<rvalue>{variable("i",8),constant(100)},memory(undefined(),1,LittleEndian,"none"))});
		// b4
		mnemonic mne12(bound(11,12),"mne12","",{},{});
		// b5
		mnemonic mne13(bound(12,13),"mne13","",{},{instr(univ_nop<rvalue>{undefined()},variable("a",8))});
		mnemonic mne14(bound(13,14),"mne14","",{},{instr(univ_nop<rvalue>{undefined()},variable("d",8))});
		mnemonic mne15(bound(14,15),"mne15","",{},{instr(int_less<rvalue>{variable("a",8),variable("d",8)},memory(undefined(),1,LittleEndian,"none"))});

		// b6
		mnemonic mne16(bound(15,16),"mne16","",{},{instr(univ_nop<rvalue>{undefined()},variable("d",8))});
		// b7
		mnemonic mne17(bound(16,17),"mne17","",{},{instr(univ_nop<rvalue>{undefined()},variable("b",8))});
		// b8
		mnemonic mne18(bound(17,18),"mne18","",{},{instr(univ_nop<rvalue>{undefined()},variable("c",8))});

		b0 = insert_vertex(boost::variant<bblock_loc,rvalue>(bblock_loc(new basic_block({mne1}))),proc.write().control_transfers);
		b1 = insert_vertex(boost::variant<bblock_loc,rvalue>(bblock_loc(new basic_block({mne2,mne3,mne4}))),proc.write().control_transfers);
		b2 = insert_vertex(boost::variant<bblock_loc,rvalue>(bblock_loc(new basic_block({mne5,mne6,mne7}))),proc.write().control_transfers);
		b3 = insert_vertex(boost::variant<bblock_loc,rvalue>(bblock_loc(new basic_block({mne8,mne9,mne10,mne11}))),proc.write().control_transfers);
		b4 = insert_vertex(boost::variant<bblock_loc,rvalue>(bblock_loc(new basic_block({mne12}))),proc.write().control_transfers);
		b5 = insert_vertex(boost::variant<bblock_loc,rvalue>(bblock_loc(new basic_block({mne13,mne14,mne15}))),proc.write().control_transfers);
		b6 = insert_vertex(boost::variant<bblock_loc,rvalue>(bblock_loc(new basic_block({mne16}))),proc.write().control_transfers);
		b7 = insert_vertex(boost::variant<bblock_loc,rvalue>(bblock_loc(new basic_block({mne17}))),proc.write().control_transfers);
		b8 = insert_vertex(boost::variant<bblock_loc,rvalue>(bblock_loc(new basic_block({mne18}))),proc.write().control_transfers);

		insert_edge(guard(),b0,b1,proc.write().control_transfers);
		insert_edge(guard(),b1,b2,proc.write().control_transfers);
		insert_edge(guard(),b1,b5,proc.write().control_transfers);
		insert_edge(guard(),b5,b6,proc.write().control_transfers);
		insert_edge(guard(),b5,b8,proc.write().control_transfers);
		insert_edge(guard(),b6,b7,proc.write().control_transfers);
		insert_edge(guard(),b8,b7,proc.write().control_transfers);
		insert_edge(guard(),b2,b3,proc.write().control_transfers);
		insert_edge(guard(),b7,b3,proc.write().control_transfers);
		insert_edge(guard(),b3,b4,proc.write().control_transfers);
		insert_edge(guard(),b3,b1,proc.write().control_transfers);

		proc.write().entry = get<bblock_loc>(get_vertex(b0,proc->control_transfers));
		save_point(store);
	}

	rdf::storage store;
	proc_loc proc;
	vx b0, b1, b2, b3, b4, b5, b6, b7, b8;
};

TEST_F(dflow,dominance)
{
	auto d = dominance_tree(proc);
	const tree<bblock_wloc>& tr = d->dominance;
	auto q = po::tree<bblock_wloc>::depth_first_search(tr.root(),tr);
	auto i = q.first;

	while(i != q.second)
	{
		bblock_loc bb = i->lock();

		if(bb == get<bblock_loc>(get_vertex(b0,proc->control_transfers)))
		{
			ASSERT_EQ(*tr.root(), bb);
			auto j = tr.begin(i);
			ASSERT_EQ(std::distance(j,tr.end(i)), 1);
			ASSERT_EQ(*j, get<bblock_loc>(get_vertex(b1,proc->control_transfers)));
		}
		else if(bb == get<bblock_loc>(get_vertex(b1,proc->control_transfers)))
		{
			auto j = tr.begin(i);
			ASSERT_EQ(std::distance(j,tr.end(i)), 3);
			ASSERT_TRUE(*j == get<bblock_loc>(get_vertex(b2,proc->control_transfers)) ||
									*std::next(j,1) == get<bblock_loc>(get_vertex(b2,proc->control_transfers)) ||
									*std::next(j,2) == get<bblock_loc>(get_vertex(b2,proc->control_transfers)));
			ASSERT_TRUE(*j == get<bblock_loc>(get_vertex(b3,proc->control_transfers)) ||
									*std::next(j,1) == get<bblock_loc>(get_vertex(b3,proc->control_transfers)) ||
									*std::next(j,2) == get<bblock_loc>(get_vertex(b3,proc->control_transfers)));
			ASSERT_TRUE(*j == get<bblock_loc>(get_vertex(b5,proc->control_transfers)) ||
									*std::next(j,1) == get<bblock_loc>(get_vertex(b5,proc->control_transfers)) ||
									*std::next(j,2) == get<bblock_loc>(get_vertex(b5,proc->control_transfers)));
		}
		else if(bb == get<bblock_loc>(get_vertex(b2,proc->control_transfers)))
		{
			auto j = tr.begin(i);
			ASSERT_EQ(std::distance(j,tr.end(i)), 0);
		}
		else if(bb == get<bblock_loc>(get_vertex(b3,proc->control_transfers)))
		{
			auto j = tr.begin(i);
			ASSERT_EQ(std::distance(j,tr.end(i)), 1);
			ASSERT_EQ(*j, get<bblock_loc>(get_vertex(b4,proc->control_transfers)));
		}
		else if(bb == get<bblock_loc>(get_vertex(b4,proc->control_transfers)))
		{
			auto j = tr.begin(i);
			ASSERT_EQ(std::distance(j,tr.end(i)), 0);
		}
		else if(bb == get<bblock_loc>(get_vertex(b5,proc->control_transfers)))
		{
			auto j = tr.begin(i);
			ASSERT_EQ(std::distance(j,tr.end(i)), 3);
			ASSERT_TRUE(*j == get<bblock_loc>(get_vertex(b6,proc->control_transfers)) ||
									*std::next(j,1) == get<bblock_loc>(get_vertex(b6,proc->control_transfers)) ||
									*std::next(j,2) == get<bblock_loc>(get_vertex(b6,proc->control_transfers)));
			ASSERT_TRUE(*j == get<bblock_loc>(get_vertex(b7,proc->control_transfers)) ||
									*std::next(j,1) == get<bblock_loc>(get_vertex(b7,proc->control_transfers)) ||
									*std::next(j,2) == get<bblock_loc>(get_vertex(b7,proc->control_transfers)));
			ASSERT_TRUE(*j == get<bblock_loc>(get_vertex(b8,proc->control_transfers)) ||
									*std::next(j,1) == get<bblock_loc>(get_vertex(b8,proc->control_transfers)) ||
									*std::next(j,2) == get<bblock_loc>(get_vertex(b8,proc->control_transfers)));
		}
		else if(bb == get<bblock_loc>(get_vertex(b6,proc->control_transfers)))
		{
			ASSERT_EQ(std::distance(tr.begin(i),tr.end(i)), 0);
		}
		else if(bb == get<bblock_loc>(get_vertex(b7,proc->control_transfers)))
		{
			ASSERT_EQ(std::distance(tr.begin(i),tr.end(i)), 0);
		}
		else if(bb == get<bblock_loc>(get_vertex(b8,proc->control_transfers)))
		{
			ASSERT_EQ(std::distance(tr.begin(i),tr.end(i)), 0);
		}
		else
		{
			ASSERT_TRUE(false);
		}

		++i;
	}

	ASSERT_EQ(d->frontiers.size(), 7u);
	ASSERT_EQ(d->frontiers.count(get<bblock_loc>(get_vertex(b0,proc->control_transfers))), 0u);
	ASSERT_EQ(d->frontiers.count(get<bblock_loc>(get_vertex(b1,proc->control_transfers))), 1u);
	ASSERT_TRUE(d->frontiers.equal_range(get<bblock_loc>(get_vertex(b1,proc->control_transfers))).first->second ==
			get<bblock_loc>(get_vertex(b1,proc->control_transfers)));
	ASSERT_EQ(d->frontiers.count(get<bblock_loc>(get_vertex(b2,proc->control_transfers))), 1u);
	ASSERT_TRUE(d->frontiers.equal_range(get<bblock_loc>(get_vertex(b2,proc->control_transfers))).first->second ==
			get<bblock_loc>(get_vertex(b3,proc->control_transfers)));
	ASSERT_EQ(d->frontiers.count(get<bblock_loc>(get_vertex(b3,proc->control_transfers))), 1u);
	ASSERT_TRUE(d->frontiers.equal_range(get<bblock_loc>(get_vertex(b3,proc->control_transfers))).first->second ==
			get<bblock_loc>(get_vertex(b1,proc->control_transfers)));
	ASSERT_EQ(d->frontiers.count(get<bblock_loc>(get_vertex(b4,proc->control_transfers))), 0u);
	ASSERT_EQ(d->frontiers.count(get<bblock_loc>(get_vertex(b5,proc->control_transfers))), 1u);
	ASSERT_TRUE(d->frontiers.equal_range(get<bblock_loc>(get_vertex(b5,proc->control_transfers))).first->second ==
			get<bblock_loc>(get_vertex(b3,proc->control_transfers)));
	ASSERT_EQ(d->frontiers.count(get<bblock_loc>(get_vertex(b6,proc->control_transfers))), 1u);
	ASSERT_TRUE(d->frontiers.equal_range(get<bblock_loc>(get_vertex(b6,proc->control_transfers))).first->second ==
			get<bblock_loc>(get_vertex(b7,proc->control_transfers)));
	ASSERT_EQ(d->frontiers.count(get<bblock_loc>(get_vertex(b7,proc->control_transfers))), 1u);
	ASSERT_TRUE(d->frontiers.equal_range(get<bblock_loc>(get_vertex(b7,proc->control_transfers))).first->second ==
			get<bblock_loc>(get_vertex(b3,proc->control_transfers)));
	ASSERT_EQ(d->frontiers.count(get<bblock_loc>(get_vertex(b8,proc->control_transfers))), 1u);
	ASSERT_TRUE(d->frontiers.equal_range(get<bblock_loc>(get_vertex(b8,proc->control_transfers))).first->second ==
			get<bblock_loc>(get_vertex(b7,proc->control_transfers)));
}

TEST_F(dflow,liveness)
{
	live l = liveness(proc);
	ASSERT_EQ(l.names.size(), 7u);

	// a
	auto p = l.usage.equal_range("a");
	ASSERT_EQ(std::distance(p.first,p.second), 2);
	ASSERT_TRUE(l.usage.find("a")->second == get<bblock_loc>(get_vertex(b1,proc->control_transfers)) ||
							l.usage.find("a")->second == get<bblock_loc>(get_vertex(b5,proc->control_transfers)));
	ASSERT_TRUE(std::next(l.usage.find("a"),1)->second == get<bblock_loc>(get_vertex(b1,proc->control_transfers)) ||
							std::next(l.usage.find("a"),1)->second == get<bblock_loc>(get_vertex(b5,proc->control_transfers)));

	// b
	p = l.usage.equal_range("b");
	ASSERT_EQ(std::distance(p.first,p.second), 2);
	ASSERT_TRUE(l.usage.find("b")->second == get<bblock_loc>(get_vertex(b2,proc->control_transfers)) ||
							l.usage.find("b")->second == get<bblock_loc>(get_vertex(b7,proc->control_transfers)));
	ASSERT_TRUE(std::next(l.usage.find("b"),1)->second == get<bblock_loc>(get_vertex(b2,proc->control_transfers)) ||
							std::next(l.usage.find("b"),1)->second == get<bblock_loc>(get_vertex(b7,proc->control_transfers)));

	// c
	p = l.usage.equal_range("c");
	ASSERT_EQ(std::distance(p.first,p.second), 3);
	ASSERT_TRUE(l.usage.find("c")->second == get<bblock_loc>(get_vertex(b1,proc->control_transfers)) ||
							l.usage.find("c")->second == get<bblock_loc>(get_vertex(b2,proc->control_transfers)) ||
							l.usage.find("c")->second == get<bblock_loc>(get_vertex(b8,proc->control_transfers)));
	ASSERT_TRUE(std::next(l.usage.find("c"),1)->second == get<bblock_loc>(get_vertex(b1,proc->control_transfers)) ||
							std::next(l.usage.find("c"),1)->second == get<bblock_loc>(get_vertex(b2,proc->control_transfers)) ||
							std::next(l.usage.find("c"),1)->second == get<bblock_loc>(get_vertex(b8,proc->control_transfers)));
	ASSERT_TRUE(std::next(l.usage.find("c"),2)->second == get<bblock_loc>(get_vertex(b1,proc->control_transfers)) ||
							std::next(l.usage.find("c"),2)->second == get<bblock_loc>(get_vertex(b2,proc->control_transfers)) ||
							std::next(l.usage.find("c"),2)->second == get<bblock_loc>(get_vertex(b8,proc->control_transfers)));

	// d
	p = l.usage.equal_range("d");
	ASSERT_EQ(std::distance(p.first,p.second), 3);
	ASSERT_TRUE(l.usage.find("d")->second == get<bblock_loc>(get_vertex(b2,proc->control_transfers)) ||
							l.usage.find("d")->second == get<bblock_loc>(get_vertex(b5,proc->control_transfers)) ||
							l.usage.find("d")->second == get<bblock_loc>(get_vertex(b6,proc->control_transfers)));
	ASSERT_TRUE(std::next(l.usage.find("d"),1)->second == get<bblock_loc>(get_vertex(b2,proc->control_transfers)) ||
							std::next(l.usage.find("d"),1)->second == get<bblock_loc>(get_vertex(b5,proc->control_transfers)) ||
							std::next(l.usage.find("d"),1)->second == get<bblock_loc>(get_vertex(b6,proc->control_transfers)));
	ASSERT_TRUE(std::next(l.usage.find("d"),2)->second == get<bblock_loc>(get_vertex(b2,proc->control_transfers)) ||
							std::next(l.usage.find("d"),2)->second == get<bblock_loc>(get_vertex(b5,proc->control_transfers)) ||
							std::next(l.usage.find("d"),2)->second == get<bblock_loc>(get_vertex(b6,proc->control_transfers)));

	// i
	p = l.usage.equal_range("i");
	ASSERT_EQ(std::distance(p.first,p.second), 2);
	ASSERT_TRUE(l.usage.find("i")->second == get<bblock_loc>(get_vertex(b0,proc->control_transfers)) ||
							l.usage.find("i")->second == get<bblock_loc>(get_vertex(b3,proc->control_transfers)));
	ASSERT_TRUE(std::next(l.usage.find("i"),1)->second == get<bblock_loc>(get_vertex(b0,proc->control_transfers)) ||
							std::next(l.usage.find("i"),1)->second == get<bblock_loc>(get_vertex(b3,proc->control_transfers)));

	// y
	p = l.usage.equal_range("y");
	ASSERT_EQ(std::distance(p.first,p.second), 1);
	ASSERT_TRUE(l.usage.find("y")->second == get<bblock_loc>(get_vertex(b3,proc->control_transfers)));

	// z
	p = l.usage.equal_range("z");
	ASSERT_EQ(std::distance(p.first,p.second), 1);
	ASSERT_TRUE(l.usage.find("z")->second == get<bblock_loc>(get_vertex(b3,proc->control_transfers)));

	ASSERT_EQ(l[get<bblock_loc>(get_vertex(b0,proc->control_transfers))].uevar.size(), 0u);
	ASSERT_EQ(l[get<bblock_loc>(get_vertex(b1,proc->control_transfers))].uevar.size(), 0u);
	ASSERT_EQ(l[get<bblock_loc>(get_vertex(b2,proc->control_transfers))].uevar.size(), 0u);
	ASSERT_EQ(l[get<bblock_loc>(get_vertex(b3,proc->control_transfers))].uevar.size(), 5u);
	ASSERT_EQ(l[get<bblock_loc>(get_vertex(b4,proc->control_transfers))].uevar.size(), 0u);
	ASSERT_EQ(l[get<bblock_loc>(get_vertex(b5,proc->control_transfers))].uevar.size(), 0u);
	ASSERT_EQ(l[get<bblock_loc>(get_vertex(b6,proc->control_transfers))].uevar.size(), 0u);
	ASSERT_EQ(l[get<bblock_loc>(get_vertex(b7,proc->control_transfers))].uevar.size(), 0u);
	ASSERT_EQ(l[get<bblock_loc>(get_vertex(b8,proc->control_transfers))].uevar.size(), 0u);

	ASSERT_EQ(l[get<bblock_loc>(get_vertex(b0,proc->control_transfers))].varkill.size(), 1u);
	ASSERT_EQ(l[get<bblock_loc>(get_vertex(b1,proc->control_transfers))].varkill.size(), 2u);
	ASSERT_EQ(l[get<bblock_loc>(get_vertex(b2,proc->control_transfers))].varkill.size(), 3u);
	ASSERT_EQ(l[get<bblock_loc>(get_vertex(b3,proc->control_transfers))].varkill.size(), 3u);
	ASSERT_EQ(l[get<bblock_loc>(get_vertex(b4,proc->control_transfers))].varkill.size(), 0u);
	ASSERT_EQ(l[get<bblock_loc>(get_vertex(b5,proc->control_transfers))].varkill.size(), 2u);
	ASSERT_EQ(l[get<bblock_loc>(get_vertex(b6,proc->control_transfers))].varkill.size(), 1u);
	ASSERT_EQ(l[get<bblock_loc>(get_vertex(b7,proc->control_transfers))].varkill.size(), 1u);
	ASSERT_EQ(l[get<bblock_loc>(get_vertex(b8,proc->control_transfers))].varkill.size(), 1u);

	ASSERT_EQ(l[get<bblock_loc>(get_vertex(b0,proc->control_transfers))].liveout.size(), 1u);
	ASSERT_EQ(l[get<bblock_loc>(get_vertex(b1,proc->control_transfers))].liveout.size(), 3u);
	ASSERT_EQ(l[get<bblock_loc>(get_vertex(b2,proc->control_transfers))].liveout.size(), 5u);
	ASSERT_EQ(l[get<bblock_loc>(get_vertex(b3,proc->control_transfers))].liveout.size(), 1u);
	ASSERT_EQ(l[get<bblock_loc>(get_vertex(b4,proc->control_transfers))].liveout.size(), 0u);
	ASSERT_EQ(l[get<bblock_loc>(get_vertex(b5,proc->control_transfers))].liveout.size(), 4u);
	ASSERT_EQ(l[get<bblock_loc>(get_vertex(b6,proc->control_transfers))].liveout.size(), 4u);
	ASSERT_EQ(l[get<bblock_loc>(get_vertex(b7,proc->control_transfers))].liveout.size(), 5u);
	ASSERT_EQ(l[get<bblock_loc>(get_vertex(b8,proc->control_transfers))].liveout.size(), 4u);
}

TEST_F(dflow,static_single_assignment)
{
	ssa(proc,*dominance_tree(proc),liveness(proc));

	ASSERT_NE(get<bblock_loc>(get_vertex(b0,proc->control_transfers))->mnemonics()[0].opcode, "internal-phis");
	ASSERT_EQ(get<bblock_loc>(get_vertex(b1,proc->control_transfers))->mnemonics()[0].opcode, "internal-phis");
	ASSERT_NE(get<bblock_loc>(get_vertex(b2,proc->control_transfers))->mnemonics()[0].opcode, "internal-phis");
	ASSERT_EQ(get<bblock_loc>(get_vertex(b3,proc->control_transfers))->mnemonics()[0].opcode, "internal-phis");
	ASSERT_NE(get<bblock_loc>(get_vertex(b4,proc->control_transfers))->mnemonics()[0].opcode, "internal-phis");
	ASSERT_NE(get<bblock_loc>(get_vertex(b5,proc->control_transfers))->mnemonics()[0].opcode, "internal-phis");
	ASSERT_NE(get<bblock_loc>(get_vertex(b6,proc->control_transfers))->mnemonics()[0].opcode, "internal-phis");
	ASSERT_EQ(get<bblock_loc>(get_vertex(b7,proc->control_transfers))->mnemonics()[0].opcode, "internal-phis");
	ASSERT_NE(get<bblock_loc>(get_vertex(b8,proc->control_transfers))->mnemonics()[0].opcode, "internal-phis");
}

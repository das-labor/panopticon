#include <iostream>
#include <algorithm>
#include <iterator>
#include <stdexcept>

#include <gtest/gtest.h>

#include <panopticon/dflow.hh>
#include <panopticon/procedure.hh>

using namespace po;
using namespace boost;

TEST(dflow,dominance)
{
	proc_loc p(new procedure("proc"));

	auto b0 = insert_vertex(boost::variant<bblock_loc,rvalue>(bblock_loc(new basic_block())),p.write().control_transfers);
	auto b1 = insert_vertex(boost::variant<bblock_loc,rvalue>(bblock_loc(new basic_block())),p.write().control_transfers);
	auto b2 = insert_vertex(boost::variant<bblock_loc,rvalue>(bblock_loc(new basic_block())),p.write().control_transfers);
	auto b3 = insert_vertex(boost::variant<bblock_loc,rvalue>(bblock_loc(new basic_block())),p.write().control_transfers);
	auto b4 = insert_vertex(boost::variant<bblock_loc,rvalue>(bblock_loc(new basic_block())),p.write().control_transfers);
	auto b5 = insert_vertex(boost::variant<bblock_loc,rvalue>(bblock_loc(new basic_block())),p.write().control_transfers);
	auto b6 = insert_vertex(boost::variant<bblock_loc,rvalue>(bblock_loc(new basic_block())),p.write().control_transfers);
	auto b7 = insert_vertex(boost::variant<bblock_loc,rvalue>(bblock_loc(new basic_block())),p.write().control_transfers);
	auto b8 = insert_vertex(boost::variant<bblock_loc,rvalue>(bblock_loc(new basic_block())),p.write().control_transfers);

	insert_edge(guard(),b0,b1,p.write().control_transfers);
	insert_edge(guard(),b1,b2,p.write().control_transfers);
	insert_edge(guard(),b1,b5,p.write().control_transfers);
	insert_edge(guard(),b5,b6,p.write().control_transfers);
	insert_edge(guard(),b5,b8,p.write().control_transfers);
	insert_edge(guard(),b6,b7,p.write().control_transfers);
	insert_edge(guard(),b8,b7,p.write().control_transfers);
	insert_edge(guard(),b2,b3,p.write().control_transfers);
	insert_edge(guard(),b7,b3,p.write().control_transfers);
	insert_edge(guard(),b3,b4,p.write().control_transfers);
	insert_edge(guard(),b3,b1,p.write().control_transfers);

	p.write().entry = get<bblock_loc>(get_vertex(b0,p->control_transfers));

	auto d = dominance_tree(p);
	const tree<bblock_wloc>& tr = d->dominance;
	auto q = po::tree<bblock_wloc>::depth_first_search(tr.root(),tr);
	auto i = q.first;

	while(i != q.second)
	{
		bblock_loc bb = i->lock();

		if(bb == get<bblock_loc>(get_vertex(b0,p->control_transfers)))
		{
			ASSERT_EQ(*tr.root(), bb);
			auto j = tr.begin(i);
			ASSERT_EQ(std::distance(j,tr.end(i)), 1);
			ASSERT_EQ(*j, get<bblock_loc>(get_vertex(b1,p->control_transfers)));
		}
		else if(bb == get<bblock_loc>(get_vertex(b1,p->control_transfers)))
		{
			auto j = tr.begin(i);
			ASSERT_EQ(std::distance(j,tr.end(i)), 3);
			ASSERT_TRUE(*j == get<bblock_loc>(get_vertex(b2,p->control_transfers)) ||
									*std::next(j,1) == get<bblock_loc>(get_vertex(b2,p->control_transfers)) ||
									*std::next(j,2) == get<bblock_loc>(get_vertex(b2,p->control_transfers)));
			ASSERT_TRUE(*j == get<bblock_loc>(get_vertex(b3,p->control_transfers)) ||
									*std::next(j,1) == get<bblock_loc>(get_vertex(b3,p->control_transfers)) ||
									*std::next(j,2) == get<bblock_loc>(get_vertex(b3,p->control_transfers)));
			ASSERT_TRUE(*j == get<bblock_loc>(get_vertex(b5,p->control_transfers)) ||
									*std::next(j,1) == get<bblock_loc>(get_vertex(b5,p->control_transfers)) ||
									*std::next(j,2) == get<bblock_loc>(get_vertex(b5,p->control_transfers)));
		}
		else if(bb == get<bblock_loc>(get_vertex(b2,p->control_transfers)))
		{
			auto j = tr.begin(i);
			ASSERT_EQ(std::distance(j,tr.end(i)), 0);
		}
		else if(bb == get<bblock_loc>(get_vertex(b3,p->control_transfers)))
		{
			auto j = tr.begin(i);
			ASSERT_EQ(std::distance(j,tr.end(i)), 1);
			ASSERT_EQ(*j, get<bblock_loc>(get_vertex(b4,p->control_transfers)));
		}
		else if(bb == get<bblock_loc>(get_vertex(b4,p->control_transfers)))
		{
			auto j = tr.begin(i);
			ASSERT_EQ(std::distance(j,tr.end(i)), 0);
		}
		else if(bb == get<bblock_loc>(get_vertex(b5,p->control_transfers)))
		{
			auto j = tr.begin(i);
			ASSERT_EQ(std::distance(j,tr.end(i)), 3);
			ASSERT_TRUE(*j == get<bblock_loc>(get_vertex(b6,p->control_transfers)) ||
									*std::next(j,1) == get<bblock_loc>(get_vertex(b6,p->control_transfers)) ||
									*std::next(j,2) == get<bblock_loc>(get_vertex(b6,p->control_transfers)));
			ASSERT_TRUE(*j == get<bblock_loc>(get_vertex(b7,p->control_transfers)) ||
									*std::next(j,1) == get<bblock_loc>(get_vertex(b7,p->control_transfers)) ||
									*std::next(j,2) == get<bblock_loc>(get_vertex(b7,p->control_transfers)));
			ASSERT_TRUE(*j == get<bblock_loc>(get_vertex(b8,p->control_transfers)) ||
									*std::next(j,1) == get<bblock_loc>(get_vertex(b8,p->control_transfers)) ||
									*std::next(j,2) == get<bblock_loc>(get_vertex(b8,p->control_transfers)));
		}
		else if(bb == get<bblock_loc>(get_vertex(b6,p->control_transfers)))
		{
			ASSERT_EQ(std::distance(tr.begin(i),tr.end(i)), 0);
		}
		else if(bb == get<bblock_loc>(get_vertex(b7,p->control_transfers)))
		{
			ASSERT_EQ(std::distance(tr.begin(i),tr.end(i)), 0);
		}
		else if(bb == get<bblock_loc>(get_vertex(b8,p->control_transfers)))
		{
			ASSERT_EQ(std::distance(tr.begin(i),tr.end(i)), 0);
		}
		else
		{
			ASSERT_TRUE(false);
		}

		++i;
	}

	ASSERT_EQ(d->frontiers.size(), 7);
	ASSERT_EQ(d->frontiers.count(get<bblock_loc>(get_vertex(b0,p->control_transfers))), 0);
	ASSERT_EQ(d->frontiers.count(get<bblock_loc>(get_vertex(b1,p->control_transfers))), 1);
	ASSERT_TRUE(d->frontiers.equal_range(get<bblock_loc>(get_vertex(b1,p->control_transfers))).first->second ==
			get<bblock_loc>(get_vertex(b1,p->control_transfers)));
	ASSERT_EQ(d->frontiers.count(get<bblock_loc>(get_vertex(b2,p->control_transfers))), 1);
	ASSERT_TRUE(d->frontiers.equal_range(get<bblock_loc>(get_vertex(b2,p->control_transfers))).first->second ==
			get<bblock_loc>(get_vertex(b3,p->control_transfers)));
	ASSERT_EQ(d->frontiers.count(get<bblock_loc>(get_vertex(b3,p->control_transfers))), 1);
	ASSERT_TRUE(d->frontiers.equal_range(get<bblock_loc>(get_vertex(b3,p->control_transfers))).first->second ==
			get<bblock_loc>(get_vertex(b1,p->control_transfers)));
	ASSERT_EQ(d->frontiers.count(get<bblock_loc>(get_vertex(b4,p->control_transfers))), 0);
	ASSERT_EQ(d->frontiers.count(get<bblock_loc>(get_vertex(b5,p->control_transfers))), 1);
	ASSERT_TRUE(d->frontiers.equal_range(get<bblock_loc>(get_vertex(b5,p->control_transfers))).first->second ==
			get<bblock_loc>(get_vertex(b3,p->control_transfers)));
	ASSERT_EQ(d->frontiers.count(get<bblock_loc>(get_vertex(b6,p->control_transfers))), 1);
	ASSERT_TRUE(d->frontiers.equal_range(get<bblock_loc>(get_vertex(b6,p->control_transfers))).first->second ==
			get<bblock_loc>(get_vertex(b7,p->control_transfers)));
	ASSERT_EQ(d->frontiers.count(get<bblock_loc>(get_vertex(b7,p->control_transfers))), 1);
	ASSERT_TRUE(d->frontiers.equal_range(get<bblock_loc>(get_vertex(b7,p->control_transfers))).first->second ==
			get<bblock_loc>(get_vertex(b3,p->control_transfers)));
	ASSERT_EQ(d->frontiers.count(get<bblock_loc>(get_vertex(b8,p->control_transfers))), 1);
	ASSERT_TRUE(d->frontiers.equal_range(get<bblock_loc>(get_vertex(b8,p->control_transfers))).first->second ==
			get<bblock_loc>(get_vertex(b7,p->control_transfers)));
}

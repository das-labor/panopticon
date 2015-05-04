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

#include <panopticon/structure.hh>

using namespace po;

TEST(structure,marshal_single)
{
	uuid uu1, uu2, uu3, uu4, uu5;

	struct_loc st1(uu1,std::unique_ptr<structure>(
		new structure("s1",tree<field>(field{"root",bound(0,100),integer{0xffffff,0,false,16,boost::none,LittleEndian,{}}}),"a")));
	struct_loc st2(uu2,std::unique_ptr<structure>(
		new structure("s2",tree<field>(field{"root",bound(1,100),integer{0xffffff,0,false,16,10,LittleEndian,{}}}),"a")));
	struct_loc st3(uu3,std::unique_ptr<structure>(
		new structure("s3",tree<field>(field{"root",bound(2,100),ieee754{}}),"a")));
	struct_loc st4(uu4,std::unique_ptr<structure>(
		new structure("s4",tree<field>(field{"root",bound(3,100),std::string("test")}),"a")));
	struct_loc st5(uu5,std::unique_ptr<structure>(
		new structure("s5",tree<field>(field{"root",bound(1,100),integer{0xffffff,0,false,16,10,LittleEndian,{std::make_pair(1,"one"),std::make_pair(2,"two")}}}),"a")));
	rdf::storage store;

	save_point(store);
	ASSERT_GT(store.count(),0);

	std::unique_ptr<structure> st1b(unmarshal<structure>(uu1,store));
	std::unique_ptr<structure> st2b(unmarshal<structure>(uu2,store));
	std::unique_ptr<structure> st3b(unmarshal<structure>(uu3,store));
	std::unique_ptr<structure> st4b(unmarshal<structure>(uu4,store));
	std::unique_ptr<structure> st5b(unmarshal<structure>(uu5,store));

	for(auto x: store.all())
		std::cout << x << std::endl;

	ASSERT_TRUE(*st1 == *st1b);
	ASSERT_TRUE(*st2 == *st2b);
	ASSERT_TRUE(*st3 == *st3b);
	ASSERT_TRUE(*st4 == *st4b);
	ASSERT_TRUE(*st5 == *st5b);
}

TEST(structure,marshal_tree)
{
	uuid uu1;
	tree<field> tr(field{"root",bound(0,100),integer{0xffffff,0,false,16,boost::none,LittleEndian,{}}});

	auto f1 = tr.insert(tr.root(),field{"root",bound(1,100),integer{0xffffff,0,false,16,10,LittleEndian,{}}});
	auto f2 = tr.insert(tr.root(),field{"root",bound(2,100),ieee754{}});
	auto f3 = tr.insert(f1,field{"root",bound(3,100),std::string("test")});
	auto f4 = tr.insert(f3,field{"root",bound(1,100),integer{0xffffff,0,false,16,10,LittleEndian,{std::make_pair(1,"one"),std::make_pair(2,"two")}}});

	struct_loc st1(uu1,std::unique_ptr<structure>(new structure("s1",tr,"a")));
	rdf::storage store;

	save_point(store);
	ASSERT_GT(store.count(),0);

	std::unique_ptr<structure> st1b(unmarshal<structure>(uu1,store));

	for(auto x: store.all())
		std::cout << x << std::endl;

	const tree<field>& fi = st1b->fields;
	ASSERT_TRUE(*fi.croot() == *tr.croot());
	ASSERT_TRUE(*fi.begin(fi.croot()) == *f1 || *fi.begin(fi.croot()) == *f2);
	ASSERT_TRUE(*(fi.begin(fi.croot()) + 1) == *f1 || *(fi.begin(fi.croot()) + 1) == *f2);

	auto i = fi.begin(fi.begin(fi.croot())) == fi.end(fi.begin(fi.croot())) ? fi.begin(fi.croot()) + 1 : fi.begin(fi.croot());
	ASSERT_TRUE(*fi.begin(i) == *f3);
	ASSERT_TRUE(*fi.begin(fi.begin(i)) == *f4);
}

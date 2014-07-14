#include <gtest/gtest.h>

#include <panopticon/database.hh>
#include <panopticon/program.hh>

using namespace po;

TEST(database,marshal)
{
	using vx = boost::graph_traits<po::regions>::vertex_descriptor;

	uuid uu;
	po::regions regs;

	po::region_loc r1(po::region::undefined("base",128));
	po::region_loc r2(po::region::undefined("zlib",64));
	po::region_loc r3(po::region::undefined("aes",48));

	vx vx1 = insert_vertex(r1,regs);
	vx vx2 = insert_vertex(r2,regs);
	vx vx3 = insert_vertex(r3,regs);

	insert_edge(po::bound(32,96),vx1,vx2,regs);
	insert_edge(po::bound(16,32),vx1,vx3,regs);
	insert_edge(po::bound(0,32),vx2,vx3,regs);

	rdf::storage store;
	struct_loc s1(uuid{},store), s2(uuid{},store);
	prog_loc p1(uuid{},store), p2(uuid{},store), p3(uuid{},store);
	comment_loc c1(uuid{},store), c2(uuid{},store), c3(uuid{},store);
	dbase_loc db1(uu,new database{
		"db1",
		regs,
		std::unordered_set<struct_loc>({s1,s2}),
		std::unordered_set<prog_loc>({p1,p2,p3}),
		{
			std::make_pair(ref{"base",1},c1),
			std::make_pair(ref{"zlib",0},c2),
			std::make_pair(ref{"base",55},c3)
		}
	});

	save_point(store);
	ASSERT_GT(store.count(),0);

	std::unique_ptr<database> db1b(unmarshal<database>(uu,store));

	for(auto x: store.all())
		std::cout << x << std::endl;

	for(auto x: db1b->structures)
		std::cout << x.tag() << std::endl;

	ASSERT_EQ(db1->title, db1b->title);
	ASSERT_EQ(num_vertices(db1->data), num_vertices(db1b->data));
	ASSERT_EQ(num_edges(db1->data), num_edges(db1b->data));
	ASSERT_EQ(db1->structures.count(s1), db1b->structures.count(s1));
	ASSERT_EQ(db1->structures.count(s2), db1b->structures.count(s2));
	ASSERT_EQ(db1->structures.size(), db1b->structures.size());
	ASSERT_EQ(db1->programs.count(p1), db1b->programs.count(p1));
	ASSERT_EQ(db1->programs.count(p2), db1b->programs.count(p2));
	ASSERT_EQ(db1->programs.count(p3), db1b->programs.count(p3));
	ASSERT_EQ(db1->programs.size(), db1b->programs.size());
	ASSERT_EQ(db1->comments.at(ref{"base",1}), c1);
	ASSERT_EQ(db1->comments.at(ref{"zlib",0}), c2);
	ASSERT_EQ(db1->comments.at(ref{"base",55}), c3);
}

TEST(database,comment_marshal)
{
	uuid uu;
	rdf::storage store;
	comment_loc c1(uu,new std::string("Hello, World"));

	save_point(store);
	ASSERT_GT(store.count(),0);

	for(auto x: store.all())
		std::cout << x << std::endl;

	std::unique_ptr<std::string> c1b(unmarshal<std::string>(uu,store));

	ASSERT_EQ(*c1, *c1b);
}

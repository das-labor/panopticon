#include <iostream>
#include <numeric>

#include <gtest/gtest.h>
#include <panopticon/region.hh>

using namespace po;
using namespace std;

TEST(slab,copy)
{
	layer_loc l1(new layer("anon 1",6));
	layer_loc l2(new layer("anon 2",{1,2,3}));
	layer_loc l3(new layer("anon 2",{1,2,3}));
	layer_loc l4(new layer("anon 2",{13,23,33,6,7}));
	std::list<std::pair<bound,layer_wloc>> src;

	src.emplace_back(bound(2,4),layer_wloc(l1));
	src.emplace_back(bound(0,3),layer_wloc(l2));
	src.emplace_back(bound(1,3),layer_wloc(l3));
	src.emplace_back(bound(0,5),layer_wloc(l4));

	auto a = std::accumulate(src.begin(),src.end(),slab(),[&](slab acc, const pair<bound,layer_wloc>& s)
	{
		slab all = s.second.lock()->filter(slab());
		cout << "add " << boost::icl::first(s.first) << "-" << boost::icl::upper(s.first) << endl;

		cout << acc.size() << endl;
		auto r = combine(acc,slab(all.begin() + boost::icl::first(s.first),all.begin() + boost::icl::upper(s.first)));
		cout << "new: " << r.size() << endl;
		return r;
	});

	cout << "res: " << a.size() << endl;
}

TEST(slab,iterator)
{
	vector<uint8_t> d = {1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16};
	slab s(d.data(),d.size());

	ASSERT_EQ(s.size(), d.size());
	ASSERT_EQ(s.read(0), tryte(1));
	ASSERT_EQ(s.read(15), tryte(16));

	slab::iterator i = s.begin();

	ASSERT_EQ(std::distance(s.begin(),s.end()), 16);
	ASSERT_EQ(*i, tryte(1));
	ASSERT_EQ(*(++i), tryte(2));
	ASSERT_EQ(i + 15,s.end());
}

TEST(slab,undefined)
{
	slab s(128);

	ASSERT_EQ(s.size(), 128);
	ASSERT_EQ(s.read(0), boost::none);
	ASSERT_EQ(s.read(127), boost::none);
}

TEST(layer,anonymous_layer)
{
	layer l1 = layer("anon 1",128);
	layer l2 = layer("anon 2",{1,2,3,4,5,6});
	vector<tryte> r;

	ASSERT_EQ(128,l1.filter(slab()).size());
	ASSERT_EQ(6,l2.filter(slab()).size());

	slab s = l2.filter(slab());
	std::copy(s.begin(),s.end(),back_inserter(r));
	ASSERT_EQ(r,vector<tryte>({1,2,3,4,5,6}));
}

TEST(layer,mutable_layer)
{
	vector<uint8_t> d = {1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16};
	vector<tryte> r, e({1,2,3,4,5,1,1,8,9,10,11,12,13,1,15,16});
	layer l1("mut",std::unordered_map<offset,tryte>());

	l1.write(5,1);
	l1.write(6,1);
	l1.write(13,1);

	slab s = l1.filter(slab(d.data(),d.size()));
	ASSERT_EQ(s.size(), 16);
	std::copy(s.begin(),s.end(),back_inserter(r));
	ASSERT_EQ(r,e);
}

TEST(layer,add)
{
	region_loc st = region::undefined("",12);

	st.write().add(bound(0,6),layer_loc(new layer("anon 2", {1,2,3,4,5,6})));
	st.write().add(bound(10,40),layer_loc(new layer("anon 3", {1,2,3,4,5,6})));
	st.write().add(bound(4,12),layer_loc(new layer("anon 4", {1,2,3,4,5,6})));
	auto proj = st->flatten();

	for(const std::pair<bound,layer_wloc> &p: proj)
		std::cout << p.first << ": " << p.second->name() << std::endl;
}

TEST(layer,projection)
{
	region_loc st = region::undefined("",134);
	layer_loc base(new layer("base",128));
	layer_loc xor1(new layer("xor",64));
	layer_loc add(new layer("add",27));
	layer_loc zlib(new layer("zlib",48));
	layer_loc aes(new layer("aes",32));

	st.write().add(bound(0,128),base);
	st.write().add(bound(0,64),xor1);
	st.write().add(bound(45,72),add);
	st.write().add(bound(80,128),zlib);
	st.write().add(bound(102,134),aes);

	auto proj = st->flatten();
	list<pair<bound,layer_wloc>> expect;

	expect.emplace_back(bound(0,45),layer_wloc(xor1));
	expect.emplace_back(bound(45,72),layer_wloc(add));
	expect.emplace_back(bound(72,80),layer_wloc(base));
	expect.emplace_back(bound(80,102),layer_wloc(zlib));
	expect.emplace_back(bound(102,134),layer_wloc(aes));

	std::cerr << "proj:" << std::endl;
	for(const std::pair<bound,layer_wloc> &p: proj)
		std::cerr << p.first << " => " << p.second->name() << std::endl;
	std::cerr << "expect:" << std::endl;
	for(const std::pair<bound,layer_wloc> &p: expect)
		std::cerr << p.first << " => " << p.second->name() << std::endl;
	ASSERT_TRUE(proj == expect);
}

TEST(layer,flatten)
{
	po::region_loc r1 = po::region::undefined("test",128);

	r1.write().add(po::bound(2,8),po::layer_loc(new po::layer("anon 2",{1,2,3,4,5,6,7})));
	r1.write().add(po::bound(50,62),po::layer_loc(new po::layer("anon 2",{1,2,3,4,5,6,6,5,4,3,2,1})));
	r1.write().add(po::bound(62,63),po::layer_loc(new po::layer("anon 2",{po::byte(1)})));

	auto proj = r1->flatten();
	list<po::bound> expect({
		po::bound(0,2),
		po::bound(2,8),
		po::bound(8,50),
		po::bound(50,62),
		po::bound(62,63),
		po::bound(63,128)
	});

	for(auto i: proj)
		std::cout << i.first << ": " << i.second->name() << std::endl;

	unsigned long i = 0;
	while(i < expect.size())
	{
		std::cout << next(proj.begin(),i)->first << " vs " << *next(expect.begin(),i) << std::endl;
		ASSERT_TRUE(next(proj.begin(),i)->first == *next(expect.begin(),i));
		++i;
	}
}

TEST(layer,marshal)
{
	boost::filesystem::path p1 = boost::filesystem::unique_path(boost::filesystem::temp_directory_path() / "test-panop-%%%%-%%%%-%%%%");

	{
		std::ofstream s1(p1.string());

		ASSERT_TRUE(s1.is_open());
		s1 << "Hello, World" << std::flush;
		s1.close();

		layer_loc l1(new layer("l1",33));
		layer_loc l2(new layer("l2",vector<po::byte>({1,2,3,4,5})));
		layer_loc l3(new layer("l3",std::unordered_map<offset,tryte>({
			make_pair(0,5),
			make_pair(1,5),
			make_pair(2,boost::none),
			make_pair(3,0xff),
			make_pair(4,boost::none)
		})));
		po::layer_loc l4(new po::layer("anon 2",po::blob(p1)));

		rdf::storage st;
		save_point(st);

		std::unique_ptr<layer> l1b(unmarshal<layer>(l1.tag(),st));
		std::unique_ptr<layer> l2b(unmarshal<layer>(l2.tag(),st));
		std::unique_ptr<layer> l3b(unmarshal<layer>(l3.tag(),st));
		std::unique_ptr<layer> l4b(unmarshal<layer>(l4.tag(),st));

		ASSERT_TRUE(*l1b == *l1);
		ASSERT_TRUE(*l2b == *l2);
		ASSERT_TRUE(*l3b == *l3);
		ASSERT_TRUE(*l4b == *l4);

		discard_changes();
	}

	boost::filesystem::remove(p1);
}

TEST(layer,blob)
{
	boost::filesystem::path p1 = boost::filesystem::unique_path(boost::filesystem::temp_directory_path() / "test-panop-%%%%-%%%%-%%%%");

	{
		std::ofstream s1(p1.string());

		ASSERT_TRUE(s1.is_open());
		s1 << "Hello, World" << std::flush;
		s1.close();

		po::blob mf(p1);

		po::layer_loc l1(new po::layer("anon",mf));

		po::slab s = l1->filter(slab());
		ASSERT_EQ(s.size(),12);

		auto i = s.begin();
		int idx = 0;

		while(i != s.end())
		{
			ASSERT_EQ(**i, mf.data()[idx]);
			++i;
			++idx;
		}

		discard_changes();
	}

	boost::filesystem::remove(p1);
}

TEST(layer,random_access_iter)
{
	layer_loc l1(new layer("l1",0xffffffff));
	slab sl = l1->filter(slab());
	auto i = sl.begin();

	// unused -> slab::iterator j = i + 0xc0000000;

	size_t k = 100;
	while(k--)
		slab s2 = combine(sl,sl);
}

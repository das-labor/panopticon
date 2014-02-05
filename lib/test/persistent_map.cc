#include <iostream>
#include <algorithm>

#include <gtest/gtest.h>
//#include <panopticon/persistent_map.hh>

/*template<typename K, typename V>
void dump(string name, const persistent_map<K,V> &pm, bool has = false, bool get = false)
{
	cout << name << endl;
	for_each(pm.begin(),pm.end(),[&](const pair<char,int> &p)
	{
		cout << p.first << ": " << p.second;
		if(get) cout << "; get: " << pm.get(p.first);
		if(has) cout << "; has: " << pm.has(p.first);
		cout << endl;
	});
}*/

TEST(persistent_map,read_write)
{
	/*persistent_map<char,int> *pm1, *pm2, *pm3;

	char a = 'a',b = 'b',c = 'c';
	int x = 1, y = 2, z = 3, w = 4;

	pm1 = new persistent_map<char,int>();
	pm1->mutate(a,x); // a=x
	CPPUNIT_ASSERT_EQUAL(x,pm1->get(a));
	CPPUNIT_ASSERT(pm1->has(a) && !pm1->has(b) && !pm1->has(c));

	pm2 = new persistent_map<char,int>(*pm1);	// a=x
	CPPUNIT_ASSERT(pm2->get(a) == x);
	CPPUNIT_ASSERT(pm2->has(a) && !pm2->has(b) && !pm2->has(c));
	pm2->mutate(b,y);	// a=x, b=y
	CPPUNIT_ASSERT(pm2->get(a) == x && pm2->get(b) == y);
	CPPUNIT_ASSERT(pm2->has(a) && pm2->has(b) && !pm2->has(c));

	pm3 = new persistent_map<char,int>(*pm2);	// a=x, b=y
	CPPUNIT_ASSERT(pm3->get(a) == x && pm3->get(b) == y);
	CPPUNIT_ASSERT(pm3->has(a) && pm3->has(b) && !pm3->has(c));
	CPPUNIT_ASSERT(pm2->has(a) && pm2->has(b) && !pm2->has(c));
	pm3->mutate(c,z);	// a=x, b=y, c=z
	CPPUNIT_ASSERT(pm3->get(a) == x && pm3->get(b) == y && pm3->get(c) == z);
	CPPUNIT_ASSERT(pm2->has(a) && pm2->has(b) && !pm2->has(c));
	CPPUNIT_ASSERT(pm3->has(a) && pm3->has(b) && pm3->has(c));

	pm2->mutate(b,w);	// a=x, b=w
	CPPUNIT_ASSERT(pm1->get(a) == x);
	CPPUNIT_ASSERT(pm1->has(a) && !pm1->has(b) && !pm1->has(c));
	CPPUNIT_ASSERT_EQUAL(x,pm2->get(a));
	CPPUNIT_ASSERT_EQUAL(w,pm2->get(b));
	CPPUNIT_ASSERT(pm2->has(a) && pm2->has(b) && !pm2->has(c));
	CPPUNIT_ASSERT_EQUAL(x,pm3->get(a));
	CPPUNIT_ASSERT_EQUAL(y,pm3->get(b));
	CPPUNIT_ASSERT_EQUAL(z,pm3->get(c));
	CPPUNIT_ASSERT(pm3->has(a) && pm3->has(b) && pm3->has(c));

	pm2->mutate(c,w);
	CPPUNIT_ASSERT(pm1->get(a) == x);
	CPPUNIT_ASSERT(pm1->has(a) && !pm1->has(b) && !pm1->has(c));
	CPPUNIT_ASSERT(pm2->get(a) == x && pm2->get(b) == w && pm2->get(c) == w);
	CPPUNIT_ASSERT(pm2->has(a) && pm2->has(b) && pm2->has(c));
	CPPUNIT_ASSERT(pm3->get(a) == x && pm3->get(b) == y && pm3->get(c) == z);
	CPPUNIT_ASSERT(pm3->has(a) && pm3->has(b) && pm3->has(c));

	delete pm3;
	delete pm2;
	delete pm1;*/

	ASSERT_TRUE(false);
}

TEST(persistent_map,iterators)
{
	/*persistent_map<char,int> pm;

	char a = 'a',b = 'b',c = 'c';
	int x = 1, y = 2, z = 3;

	pm.mutate(a,x);
	pm.mutate(b,y);
	pm.mutate(c,z);

	auto i = pm.begin();
	++i; ++i; ++i;
	CPPUNIT_ASSERT_EQUAL(distance(pm.begin(),pm.end()),3);
	CPPUNIT_ASSERT(i == pm.end());*/

	ASSERT_TRUE(false);
}

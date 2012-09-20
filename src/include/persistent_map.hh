#ifndef PERSISTENT_MAP
#define PERSISTENT_MAP

#include <unordered_map>
#include <map>
#include <set>
#include <memory>
#include <cassert>

#include <boost/iterator/iterator_facade.hpp>

using namespace std;

template<typename K,typename V>
class persistent_map
{
public:
	typedef map<unsigned int,V> fatnode;

	class iterator : public boost::iterator_facade<
				iterator,
				const pair<K,V>,
				boost::forward_traversal_tag,
				const pair<K,V>>
	{
	public:
		iterator(void) {};
		explicit iterator(typename unordered_map<K,fatnode>::iterator i, 
											typename unordered_map<K,fatnode>::iterator b, 
											typename unordered_map<K,fatnode>::iterator e, int v)
		: version(v), adaptee(i), begin(b), end(e)
		{
			while(adaptee != end)
			{
				if(any_of(adaptee->second.begin(),adaptee->second.end(),[&](const pair<unsigned int, V> &p) { return p.first <= version; }))
					break;
				++adaptee;
			}
		}

		iterator &increment(void) 
		{ 
			typename fatnode::iterator i;
			while(adaptee != end)
			{
				if(++adaptee == end)
					break;
				if(any_of(adaptee->second.begin(),adaptee->second.end(),[&](const pair<unsigned int, V> &p) { return p.first <= version; }))
					break;
			}
			return *this;
		};

		iterator &decrement(void)
		{ 
			return *this;
		};

		const pair<K,V> dereference(void) const 
		{ 
			assert(adaptee != end);
			auto i = adaptee->second.lower_bound(version);

			if(i == adaptee->second.end() || i->first > version) --i;
			return make_pair(adaptee->first,i->second);
		};

		bool equal(const iterator &a) const { return version == a.version && adaptee == a.adaptee; }

	private:
		unsigned int version;
		typename unordered_map<K,fatnode>::iterator adaptee, begin, end;
	};

	persistent_map(void) : version(0), mod(false), data(new unordered_map<K,fatnode>()), parent(0), child(0) {};
	
	explicit persistent_map(persistent_map &p) : version(p.version+1), mod(false), data(p.data), parent(&p), child(0) 
	{ 
		if(p.shared())
		{
			shared_ptr<unordered_map<K,fatnode>> new_data(new unordered_map<K,fatnode>());

			for_each(p.begin(),p.end(),[&](const pair<K,V> q)
				{ new_data->insert(make_pair(q.first,fatnode({make_pair(0,q.second)}))); });
			parent = 0;
			data = new_data;
			version = 0;
			
			cout << this << ": copy construct from " << &p << endl;
		}
		else
		{
			p.child = this;
			//cout << this << ": attach to " << &p << endl;
		}
	};

	void operator=(persistent_map &p)
	{
		if(shared()) alienate();

		if(p.shared())
		{
			shared_ptr<unordered_map<K,fatnode>> new_data(new unordered_map<K,fatnode>());

			for_each(p.begin(),p.end(),[&](const pair<K,V> p)
				{ new_data->insert(make_pair(p.first,fatnode({make_pair(0,p.second)}))); });
			parent = 0;
			data = new_data;
			version = 0;
			mod = false;
			
			cout << this << ": copy construct from " << &p << endl;
		}
		else
		{
			p.child = this;
			mod = false;
			version = p.version + 1;
			data = p.data;
			parent = &p;
		}
	};

	virtual ~persistent_map(void) 
	{ 
		if(shared()) alienate(true);
		if(parent) parent->child = 0;
	};
	
	bool mutate(const K &k,const V &v)
	{
		auto i = data->find(k);
		typename fatnode::iterator j;
		
		if(i == data->end())		// no such key in `data'
		{
			// the new kv-pair would show up on `child'
			if(shared()) alienate();

			data->insert(make_pair(K(k),fatnode({make_pair(version,V(v))})));
			mod = true;
			return true;
		} 
		else if((j = max_version(i)) == i->second.end())	// key found but wrong (higher) version
		{
			i->second.insert(make_pair(version,V(v)));
			mod = true;
			return true;
		}
		else		// key w/ version <= our version
		{
			// reinsert old value w/ child's versions if `child' doesn't redefine it
			if(shared() && (next(j) == i->second.end() || next(j)->first != child->version))
			{
				child->mutate(k,j->second);
				i = data->find(k);
				j = max_version(i);
			}

			if(j->first == version)
				j->second = v;
			else
				i->second.insert(make_pair(version,V(v)));
			mod = true;
			return true;
		}
	}

	bool has(const K &k) const
	{
		auto i = data->find(k); 
		
		if(i == data->end())
			return false;

		return max_version(i) != i->second.end();
	};
		
	const V &get(const K &k) const
	{	
		auto i = data->find(k); assert(i != data->end());
		auto j = max_version(i);

		return j->second;
	};	

	bool modified(void) const { return mod; };

	iterator begin(void) const { return iterator(data->begin(),data->begin(),data->end(),version); };
	iterator end(void) const { return iterator(data->end(),data->begin(),data->end(),version); };
	
	bool operator==(const persistent_map &p) const { return (data == data && !mod && !p.mod) || *data == *data; };

private:
	bool shared(void) const { return child; };

	// returns fatnode element with version <= our version
	typename fatnode::iterator max_version(typename unordered_map<K,fatnode>::iterator i) const
	{
		typename fatnode::iterator ret = i->second.lower_bound(version);

		if(ret == i->second.end() || ret->first > version)
		{
			if(ret == i->second.begin())
				return i->second.end();
			else
				--ret;
		}

		if(ret->first > version)
			return i->second.end();
		else 
			return ret;
	};

	// detach from `parent', copy whole `data' container
	void emancipate(void)
	{
		assert(parent && parent->shared() && parent->child == this);
		shared_ptr<unordered_map<K,fatnode>> new_data(new unordered_map<K,fatnode>());

		for_each(data->begin(),data->end(),[&](const pair<K,fatnode> p)
			{ new_data->insert(make_pair(p.first,fatnode({make_pair(0,this->get(p.first))}))); });

		parent->child = 0;
		cout << this << ": emancipate from " << parent << endl;
		parent = 0;
		data = new_data;
		version = 0;
	};

	// take ownership of `data' from `parent'
	void inherit(void)
	{		
		assert(parent && parent->shared() && parent->child == this);

		//cout << this << ": inherit from " << parent << endl;
		parent->child = 0;
		parent = 0;
	};

	// detach client
	void alienate(bool dying = false)
	{
		assert(shared());

		//cout << this << ": alienating " << child << endl;
		if(!dying || parent)
			child->emancipate();
		else
			child->inherit();
		
		child = 0;

		if(dying)
			return;

		// delete everything that can't be read w/ this version
		typename unordered_map<K,fatnode>::iterator i = data->begin();
		while(i != data->end())
		{
			fatnode &fn(i->second);
			
			while(!fn.empty() && (--fn.end())->first > version)
				fn.erase(--fn.end());

			if(fn.empty())
				i = data->erase(i);
			else
				++i;
		}
	};

	unsigned int version;
	bool mod, dying;
	shared_ptr<unordered_map<K,fatnode>> data;	// parent.data
	persistent_map<K,V> *parent, *child;
};

#endif

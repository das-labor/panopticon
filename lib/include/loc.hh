#include <unordered_map>
#include <mutex>
#include <memory>
#include <atomic>
#include <stdexcept>
#include <functional>

#include <boost/uuid/uuid.hpp>
#include <boost/uuid/uuid_generators.hpp>
#include <boost/uuid/uuid_io.hpp>
#include <boost/optional.hpp>
#include <boost/functional/hash.hpp>

#include <tbb/concurrent_hash_map.h>

#pragma once

/**
 * SerializableConcept
 *
 * void marshal(const T*, const uuid &);
 * T* unmarshall(const uuid&);
 */

namespace po
{
	using uuid = boost::uuids::uuid;

	extern std::unordered_map<uuid,std::function<void(void)>> dirty_locations;
	extern std::mutex dirty_locations_mutex;
}

namespace std
{
	template<>
	struct hash<po::uuid>
	{
		size_t operator()(const po::uuid &u) const { return boost::hash<boost::uuids::uuid>()(u); }
	};
}

namespace po
{
	template<typename T>
	T* unmarshal(const uuid&);

	template<typename T>
	void marshal(const T*, const uuid&);

	template<typename T>
	struct loc;

	template<typename T>
	struct wloc
	{
		wloc(loc<T> &l) : _pointed(&l)
		{
			l._references.insert(std::make_pair(this,true));
		}

		~wloc(void)
		{
			loc<T> *t = _pointed.load();
			if(t)
				t->_references.erase(this);
		}

		const T* operator->(void) const { return read(); }
		const T& operator*(void) const { return *read(); }

		const T* read(void) const { return pointed()->read(); }
		T& write(void) { return pointed()->write(); }
		const uuid& tag(void) const { return pointed()->_uuid; }

	private:
		std::atomic<struct loc<T>*> _pointed;

		loc<T> *pointed(void) const
		{
			loc<T> *t = _pointed.load();
			if(!t)
				throw std::runtime_error("expired wloc");
			return t;
		}

		friend struct loc<T>;
	};

	template<typename T>
	struct loc
	{
		loc(void) = delete;
		loc(const uuid &u) : _uuid(u), _current(boost::none) {}
		loc(const uuid &u, T* t) : _uuid(u), _current(t) { write(); }
		loc(const loc &) = delete;
		loc(loc<T> &&l) : _uuid(l._uuid), _current(l._current) { l._uuid = boost::uuids::nil_generator()(); l._current.reset(); }

		~loc(void)
		{

			for(std::pair< wloc<T>*,bool> p: _references)
				p.first->_pointed.store(nullptr);
			_references.clear();
		}

		void remove(void)
		{
			std::lock_guard<std::mutex> g(dirty_locations_mutex);
			dirty_locations.erase(_uuid);
		}

		loc<T>& operator=(const loc<T> &) = delete;
		loc<T>& operator=(const loc<T> &&l)
		{
			{
				std::lock_guard<std::mutex> g(dirty_locations_mutex);
				dirty_locations.erase(_uuid);
			}

			_uuid = std::move(l._uuid);
			_current = std::move(l._current);

			for(std::pair<wloc<T>*,bool> p: _references)
				p.first->_pointed.store(nullptr);
			_references.clear();
			return *this;
		}

		const T* operator->(void) const { return read(); }
		const T& operator*(void) const { return *read(); }
		//T* operator->(void) { return write(); }
		//T& operator*(void) { return *write(); }

		const T* read(void) const
		{
			if(!_current)
				 _current.reset(unmarshal<T>(_uuid));
			return _current.get();
		}

		T& write(void)
		{
			if(!_current)
				 _current.reset(unmarshal<T>(_uuid));

			{
				std::lock_guard<std::mutex> g(dirty_locations_mutex);
				dirty_locations.insert(make_pair(_uuid,std::bind(marshal<T>,_current.get(),_uuid)));
			}

			return *(_current.get());
		}

		const uuid& tag(void) const { return _uuid; }

	private:
		uuid _uuid;
		mutable boost::optional<T*> _current;
		tbb::interface5::concurrent_hash_map<wloc<T>*,bool> _references;

		friend struct wloc<T>;
	};

	void save_point(void);
}

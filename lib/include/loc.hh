#include <unordered_map>
#include <unordered_set>
#include <mutex>
#include <memory>
#include <atomic>
#include <stdexcept>
#include <functional>

#include <boost/optional.hpp>
#include <boost/type_erasure/any.hpp>
#include <boost/type_erasure/any_cast.hpp>
#include <boost/type_erasure/builtin.hpp>
#include <boost/type_erasure/operators.hpp>
#include <boost/type_erasure/member.hpp>
#include <boost/type_erasure/free.hpp>
#include <boost/mpl/vector.hpp>
#include <boost/variant.hpp>

#include <panopticon/marshal.hh>

#pragma once

namespace po
{
	using marshal_poly = std::function<rdf::statements(void)>;

	// pair<to delete,to write>
	extern std::unordered_map<uuid,std::pair<marshal_poly,marshal_poly>> dirty_locations;
	extern std::mutex dirty_locations_mutex;

	template<typename T>
	struct loc_control
	{
		loc_control(void) = delete;
		loc_control(T *t) : inner(t) {}
		loc_control(const rdf::storage &s) : inner(&s) {}

		~loc_control(void)
		{
			T** t = boost::get<T*>(&inner);
			if(t && *t)
				delete *t;
		}

		bool has_object(void) const { return !!boost::get<T*>(&inner); }

		T* object(void) { return boost::get<T*>(inner); }
		const rdf::storage &storage(void) { return *boost::get<const rdf::storage*>(inner); }

		boost::variant<T*,const rdf::storage*> inner;
	};

	template<typename T>
	marshal_poly make_marshal_poly(std::shared_ptr<loc_control<T>> t, const uuid u)
	{
		std::function<rdf::statements(void)> ret = [t,u](void)
		{
			rdf::statements ret = (t ? marshal<T>(t->object(),u) : rdf::statements());
			return ret;
		};

		return ret;
	}

	template<typename T,typename D>
	struct basic_loc
	{
		basic_loc(void) = delete;
		basic_loc(const basic_loc<T,D>&) = delete;
		basic_loc(const uuid &u) : _uuid(u) {}

		const T* operator->(void) const { return read(); }
		const T& operator*(void) const { return *read(); }
		//T* operator->(void) { return write(); }
		//T& operator*(void) { return *write(); }

		const T* read(void) const
		{
			std::shared_ptr<loc_control<T>> cb = static_cast<const D*>(this)->control();

			if(!cb->has_object())
				cb->inner = unmarshal<T>(_uuid,cb->storage());

			if(!cb->object())
				throw std::runtime_error("reading deleted loc");

			return cb->object();
		}

		T& write(void)
		{
			read();

			std::shared_ptr<loc_control<T>> cb = static_cast<const D*>(this)->control();

			{
				std::lock_guard<std::mutex> guard(dirty_locations_mutex);
				marshal_poly prev;

				if(dirty_locations.count(_uuid))
				{
					prev = dirty_locations.at(_uuid).first;
					dirty_locations.erase(_uuid);
				}
				else
				{
					prev = make_marshal_poly(std::make_shared<loc_control<T>>(new T(*(cb->object()))),_uuid);
				}

				assert(dirty_locations.emplace(_uuid,std::make_pair(prev,make_marshal_poly(cb,_uuid))).second);
			}
			return *cb->object();
		}

		void remove(void)
		{
			read();

			std::shared_ptr<loc_control<T>> cb = static_cast<const D*>(this)->control();

			{
				std::lock_guard<std::mutex> guard(dirty_locations_mutex);
				marshal_poly prev;

				if(dirty_locations.count(_uuid))
				{
					prev = dirty_locations.at(_uuid).first;
					dirty_locations.erase(_uuid);
				}
				else
				{
					prev = make_marshal_poly(std::make_shared<loc_control<T>>(new T(*(cb->object()))),_uuid);
				}

				assert(dirty_locations.emplace(_uuid,std::make_pair(prev,make_marshal_poly(std::shared_ptr<loc_control<T>>(),_uuid))).second);
			}

			cb->inner = static_cast<T*>(nullptr);
		}

		const uuid& tag(void) const { return _uuid; }

	private:
		uuid _uuid;
	};

	template<typename T>
	struct wloc;

	template<typename T>
	struct loc : public basic_loc<T,loc<T>>
	{
	public:
		using basic_loc<T,loc<T>>::tag;

		loc(const loc<T> &l) : basic_loc<T,loc<T>>(l.tag()), _control(l._control) {}
		loc(const uuid &u, T* t) : basic_loc<T,loc<T>>(u), _control(new loc_control<T>(t))
		{
			std::lock_guard<std::mutex> guard(dirty_locations_mutex);
			assert(dirty_locations.emplace(tag(),std::make_pair(make_marshal_poly(std::shared_ptr<loc_control<T>>(),tag()),make_marshal_poly(_control,tag()))).second);
		}

		loc(const uuid &u, const rdf::storage &s) : basic_loc<T,loc<T>>(u), _control(new loc_control<T>(s)) {}

	protected:
		std::shared_ptr<loc_control<T>> control(void) const { return _control; }

		mutable std::shared_ptr<loc_control<T>> _control;

		friend struct basic_loc<T,loc<T>>;
		friend struct wloc<T>;
	};

	template<typename T>
	struct wloc : public basic_loc<T,wloc<T>>
	{
	public:
		wloc(loc<T> &l) : basic_loc<T,wloc<T>>(l.tag()), _control(l.control()) {}

	protected:
		std::shared_ptr<loc_control<T>> control(void) const
		{
			if(_control.expired())
				throw std::runtime_error("expired wloc");
			return _control.lock();
		}

		mutable std::weak_ptr<loc_control<T>> _control;

		friend struct basic_loc<T,wloc<T>>;
	};

	void save_point(rdf::storage &);
}

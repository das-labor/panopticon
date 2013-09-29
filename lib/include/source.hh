#include <iostream>
#include <vector>
#include <functional>

#pragma once

namespace po
{
	typedef uint32_t addr_t;
	extern const addr_t naddr;

	template<typename T>
	struct range
	{
		range(void) : begin(0), end(0) {}
		range(T b) : begin(b), end(b) {}
		range(T b, T e) : begin(b), end(e) { assert(begin <= end); }

		size_t size(void) const { return end - begin; }
		bool includes(const range<T> &a) const { return size() && a.size() && begin <= a.begin && end > a.end; }
		bool includes(T a) const { return size() && begin <= a && end > a; }
		bool overlap(const range<T> &a) const { return size() && a.size() && !(begin >= a.end || end <= a.begin); }
		T last(void) const { return size() ? end - 1 : begin; }

		bool operator==(const range<T> &b) const { return size() == b.size() && (!size() || (begin == b.begin && end == b.end)); }
		bool operator!=(const range<T> &b) const { return !(*this == b); }
		bool operator<(const range<T> &b) const { return begin < b.begin; }

		T begin;
		T end;
	};

	template<typename T>
	std::ostream& operator<<(std::ostream &os, const range<T> &r)
	{
		if(r.size())
		{
			os << r.begin;
			if(r.size() > 1)
				os << "-" << r.end-1;
		}
		else
			os << "nil";
		return os;
	}
}

namespace std
{
	template<typename T>
	struct hash<po::range<T>>
	{
		size_t operator()(const po::range<T> &r) const
		{
			return hash<T>()(r.begin) xor hash<T>()(r.end);
		}
	};
}

namespace po
{
	struct address_space
	{
		using bytes = std::vector<uint8_t>;

		address_space(const std::string &n, std::function<bytes(const bytes&)> fn);
		bytes map(const bytes &bs, const range<addr_t> &a) const;

		bool operator==(const address_space &as) const;

		std::string name;

	private:
		std::function<bytes(const bytes&)> m_map;

		friend struct std::hash<address_space>;
	};
}

namespace std
{
	template<>
	struct hash<po::address_space>
	{
		size_t operator()(const po::address_space &as) const
		{
			return hash<string>()(as.name);
		}
	};

	template<typename X, typename Y>
	struct hash<pair<X,Y>>
	{
		size_t operator()(const std::pair<X,Y> &p) const
		{
			return std::hash<X>()(p.first) xor std::hash<Y>()(p.second);
		}
	};
}

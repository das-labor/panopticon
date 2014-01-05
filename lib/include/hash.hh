#include <unordered_set>

#pragma once

namespace po
{
	inline size_t hash_struct(void)
	{
		return 0;
	}

	/// Hashes a sequence of fields and combines them.
	template<typename Car, typename... Cdr>
	size_t hash_struct(const Car &c, const Cdr&... parameters)
	{
		size_t seed = std::hash<Car>()(c);
		return seed ^ (hash_struct(parameters...) + 0x9e3779b9 + (seed << 6) + (seed >> 2));
	}
}

namespace std
{
	template<typename A, typename B>
	struct hash<pair<A,B>>
	{
		size_t operator()(const pair<A,B> &p) const
		{
			return po::hash_struct(p.first,p.second);
		}
	};
}

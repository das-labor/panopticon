#include <string>
#include <vector>

#pragma once

#include <panopticon/value.hh>

namespace po
{
	template<typename T>
	struct architecture_traits
	{
		using token_type = void;	///< Smallest integer type that can hold one token
		using state_type = std::nullptr_t;	///< additional semantic info. Type of sem_state<Tag>::state
	};

	/// Generate new temporary variable. Must not collide with any previous temporaries.
	template<typename T>
	lvalue temporary(T);

	/// List of all registers supported by the architecture.
	template<typename T>
	const std::vector<std::string>& registers(T);

	/// Width of the register @arg n in bits. Allowed values for n are returned by registers<T>()
	template<typename T>
	uint8_t width(std::string n, T);
}

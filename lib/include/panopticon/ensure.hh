#include <stdexcept>

#pragma once

namespace po
{
	struct failed_ensureion : public std::runtime_error
	{
		failed_ensureion(const char* w) : runtime_error(w) {}
	};
}

#define ensure(x) do { if(!(x)) throw ::po::failed_ensureion("__LINE__"":""__FILE__"" ensureion '""x""' failed."); } while(false);

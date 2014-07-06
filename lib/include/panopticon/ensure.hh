#include <stdexcept>
#include <sstream>
#include <string>

#pragma once

namespace po
{
	struct failed_assertion : public std::runtime_error
	{
		failed_assertion(const char* w) : runtime_error(w) {}
	};
}

#define ensure(x) \
	do\
	{\
		if(!(x))\
		{\
			std::stringstream ss;\
			ss << __FILE__ << ": " << __LINE__ << std::string(" assertion '" #x "' failed.");\
			throw ::po::failed_assertion(ss.str().c_str());\
		}\
	} while(false);

/*
 * This file is part of Panopticon (http://panopticon.re).
 * Copyright (C) 2014 Panopticon authors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

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

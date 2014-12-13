/*
 * This file is part of Panopticon (http://panopticon.re).
 * Copyright (C) 2014 Kai Michaelis
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

#include <gtest/gtest.h>

#if defined(WIN32) || defined(_WINDOWS)
#include <windows.h>
#endif

using namespace testing;

int main(int argc, char **argv)
{
#if defined(WIN32) || defined(_WINDOWS)
	// prevent failed assertions from opening a popup
	_CrtSetReportMode(_CRT_ASSERT, _CRTDBG_MODE_DEBUG);
	_CrtSetReportMode(_CRT_ERROR, _CRTDBG_MODE_DEBUG);
	_CrtSetReportMode(_CRT_WARN, _CRTDBG_MODE_DEBUG);
	SetErrorMode(SEM_FAILCRITICALERRORS | SEM_NOGPFAULTERRORBOX | SEM_NOOPENFILEERRORBOX);
#endif

	InitGoogleTest(&argc, argv);
  return RUN_ALL_TESTS();
}

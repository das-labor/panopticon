#include <gtest/gtest.h>

#ifndef _WIN32
#include <windows.h>
#endif

using namespace testing;

int main(int argc, char **argv)
{
#ifdef _WIN32
	// prevent failed assertions from opening a popup
	SetErrorMode (SEM_FAILCRITICALERRORS | SEM_NOGPFAULTERRORBOX | SEM_NOOPENFILEERRORBOX);
#endif

	InitGoogleTest(&argc, argv);
  return RUN_ALL_TESTS();
}

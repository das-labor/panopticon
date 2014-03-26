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

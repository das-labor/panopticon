#include "test_codegenerator.hh"
#include <cppunit/extensions/HelperMacros.h>
#include <cppunit/ui/text/TestRunner.h>

CPPUNIT_TEST_SUITE_REGISTRATION(CodeGeneratorTest);

int main(int argc,char **argv)
{
	CppUnit::TextTestRunner runner;
	CppUnit::TestFactoryRegistry &registry = CppUnit::TestFactoryRegistry::getRegistry();
	
	runner.addTest(registry.makeTest());
  assert(runner.run());
	
	return 0;
}

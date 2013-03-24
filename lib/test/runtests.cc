#include "test_disassembler.hh"
#include "test_procedure.hh"

#include <cppunit/extensions/HelperMacros.h>
#include <cppunit/ui/text/TestRunner.h>

CPPUNIT_TEST_SUITE_REGISTRATION(CodeGeneratorTest);
CPPUNIT_TEST_SUITE_REGISTRATION(ProcedureTest);

int main(int argc,char **argv)
{
	CppUnit::TextTestRunner runner;
	CppUnit::TestFactoryRegistry &registry = CppUnit::TestFactoryRegistry::getRegistry();
	
	runner.addTest(registry.makeTest());
  runner.run();
	
	return 0;
}

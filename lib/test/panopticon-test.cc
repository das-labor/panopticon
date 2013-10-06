#include "test_disassembler.hh"
#include "test_procedure.hh"
#include "test_value.hh"
#include "test_graph.hh"
#include "test_source.hh"
#include <cppunit/extensions/HelperMacros.h>
#include <cppunit/ui/text/TestRunner.h>

CPPUNIT_TEST_SUITE_REGISTRATION(DisassemblerTest);
CPPUNIT_TEST_SUITE_REGISTRATION(ProcedureTest);
CPPUNIT_TEST_SUITE_REGISTRATION(ValueTest);
CPPUNIT_TEST_SUITE_REGISTRATION(GraphTest);
CPPUNIT_TEST_SUITE_REGISTRATION(SourceTest);

int main(int argc,char **argv)
{
	CppUnit::TextTestRunner runner;
	CppUnit::TestFactoryRegistry &registry = CppUnit::TestFactoryRegistry::getRegistry();

	runner.addTest(registry.makeTest());
  runner.run();

	return 0;
}

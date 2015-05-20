set(panopticon_test_HEADER architecture.hh ${panopticon_HEADERS})
set(panopticon_test_SOURCES layer.cc loc.cc region.cc testsuite.cc
	value.cc digraph.cc disassembler.cc procedure.cc
	architecture.cc store.cc marshal.cc tree.cc mnemonic.cc basic_block.cc
	program.cc dflow.cc interpreter.cc structure.cc database.cc session.cc
	amd64.cc avr.cc)

include_directories(${GTEST_INCLUDE_DIRS})
link_directories(${panopticon_LIBRARY_DIRS} ${GTEST_LIBRARY_DIRS})

add_subdirectory(data)

add_executable(testsuite ${panopticon_test_HEADERS}
	${panopticon_test_SOURCES})
add_dependencies(testsuite testdata)
add_definitions("-DTESTDATA_DIR=\"${CMAKE_CURRENT_BINARY_DIR}/data/\"")

target_link_libraries(testsuite gtest_main gtest
	${panopticon_STATIC_LIBRARIES})
panopticon_add_tests(testsuite "${CMAKE_CURRENT_BINARY_DIR}" "" "${panopticon_test_SOURCES}")

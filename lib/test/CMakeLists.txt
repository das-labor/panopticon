set(panopticon_test_HEADER architecture.hh)
set(panopticon_test_SOURCES layer.cc loc.cc region.cc testsuite.cc
	value.cc digraph.cc disassembler.cc persistent_map.cc procedure.cc
	architecture.cc store.cc marshal.cc tree.cc)

include_directories(${GTEST_INCLUDE_DIRS})
link_directories(${panopticon_LIBRARY_DIRS} ${GTEST_LIBRARY_DIRS})

add_executable(testsuite ${panopticon_test_HEADERS}
	${panopticon_test_SOURCES})
target_link_libraries(testsuite ${GTEST_LIBRARIES}
	${panopticon_LIBRARIES})
gtest_add_tests(testsuite "" "${panopticon_test_SOURCES}")

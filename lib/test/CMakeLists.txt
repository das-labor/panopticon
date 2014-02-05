set(panopticon_test_SOURCES layer.cc loc.cc region.cc testsuite.cc
	value.cc digraph.cc)

link_directories(${panopticon_LIBRARY_DIRS} ${GTEST_LIBRARY_DIRS})
add_executable(testsuite ${panopticon_test_HEADERS}
	${panopticon_test_SOURCES})
target_link_libraries(testsuite ${GTEST_LIBRARIES}
	${panopticon_LIBRARIES})
gtest_add_tests(testsuite "" "${panopticon_test_SOURCES}")

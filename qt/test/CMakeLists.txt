set(panopticon_test_HEADER architecture.hh ${panopticon_HEADERS})
set(panopticon_test_SOURCES order.cc rank.cc qttestsuite.cc)

include_directories(${GTEST_INCLUDE_DIRS})
link_directories(${panopticon_LIBRARY_DIRS} ${GTEST_LIBRARY_DIRS})

add_executable(qttestsuite ${panopticon_test_HEADERS}
	${panopticon_test_SOURCES})
target_link_libraries(qttestsuite gtest_main gtest
	${panopticon_STATIC_LIBRARIES})
panopticon_add_tests(qttestsuite "${CMAKE_CURRENT_SOURCE_DIR}" "" "${panopticon_test_SOURCES}")

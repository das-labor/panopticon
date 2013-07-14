file(GLOB panopticon_test_SOURCES *.cc)
file(GLOB panopticon_test_HEADERS *.hh)
include_directories(${panopticon_INCLUDE_DIRS} ${cppunit_INCLUDE_DIRS})
link_directories(${panopticon_LIBRARY_DIRS} ${cppunit_LIBRARY_DIRS})
remove_definitions("-Weffc++")
add_executable(panopticon-test ${panopticon_test_HEADERS}
	${panopticon_test_SOURCES})
target_link_libraries(panopticon-test ${cppunit_LIBRARIES}
	${panopticon_LIBRARIES})
add_test(panopticon panopticon-test)

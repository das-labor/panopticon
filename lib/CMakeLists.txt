set(panopticon_SOURCES src/loc.cc src/value.cc src/region.cc src/marshal.cc)
set(panopticon_HEADERS include/panopticon/loc.hh include/panopticon/value.hh
	include/panopticon/marshal.hh include/panopticon/digraph.hh
	include/panopticon/region.hh include/panopticon/hash.hh)

set(panopticon_INCLUDE_DIRS ${CMAKE_CURRENT_SOURCE_DIR}/include
	${KyotoCabinet_INCLUDE_DIRS} ${LibArchive_INCLUDE_DIRS}
	${Boost_INCLUDE_DIRS})
set(panopticon_INCLUDE_DIRS ${panopticon_INCLUDE_DIRS} PARENT_SCOPE)

set(panopticon_LIBRARY_DIRS ${LibArchive_LIBRARY_DIRS} ${Boost_LIBRARY_DIRS}
	${KyotoCabinet_LIBRARY_DIRS})
set(panopticon_LIBRARY_DIRS ${panopticon_LIBRARY_DIRS} PARENT_SCOPE)

set(panopticon_LIBRARIES panopticon ${KyotoCabinet_LIBRARIES}
	${LibArchive_LIBRARIES} ${Boost_LIBRARIES})
set(panopticon_LIBRARIES ${panopticon_LIBRARIES} PARENT_SCOPE)

set(panopticon_FOUND 1)
set(panopticon_FOUND ${panopticon_FOUND} PARENT_SCOPE)

include_directories(${panopticon_INCLUDE_DIRS})
link_directories(${panopticon_LIBRARY_DIRS})

add_library(panopticon SHARED ${panopticon_SOURCES} ${panopticon_HEADERS})
target_link_libraries(panopticon ${panopticon_LIBRARIES})

set_target_properties(panopticon PROPERTIES VERSION
	${panopticon_MAJOR}.${panopticon_MINOR}.${panopticon_PATCH} SOVERSION
	${panopticon_MAJOR}.${panopticon_MINOR}.${panopticon_PATCH})

install(TARGETS panopticon LIBRARY DESTINATION lib RUNTIME DESTINATION bin)
install(FILES ${panopticon_HEADERS} DESTINATION include/panopticon)

if(UNIX AND NOT APPLE)
  configure_file("panopticon.pc.in" "panopticon.pc" @ONLY)
  install(FILES "${CMAKE_CURRENT_BINARY_DIR}/panopticon.pc"
		DESTINATION "lib/pkgconfig")
endif()

if(${GTEST_FOUND})
	add_subdirectory("test")
endif()

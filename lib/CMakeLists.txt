set(panopticon_SOURCES src/loc.cc src/value.cc src/marshal.cc src/region.cc)
set(panopticon_HEADERS include/loc.hh include/value.hh include/marshal.hh
	include/region.hh include/hash.hh)

set(panopticon_INCLUDE_DIRS ${CMAKE_CURRENT_SOURCE_DIR} ${redland_INCLUDE_DIRS}
	${raptor_INCLUDE_DIRS} ${LibArchive_INCLUDE_DIRS})
set(panopticon_INCLUDE_DIRS ${panopticon_INCLUDE_DIRS} PARENT_SCOPE)
set(panopticon_LIBRARY_DIRS ${redland_LIBRARY_DIRS}
	${raptor_LIBRARY_DIRS} ${LibArchive_LIBRARY_DIRS} ${tbb_LIBRARY_DIRS})
set(panopticon_LIBRARY_DIRS ${panopticon_LIBRARY_DIRS} PARENT_SCOPE)
set(panopticon_LIBRARIES panopticon ${redland_LIBRARIES}
	${raptor_LIBRARIES} ${LibArchive_LIBRARIES} ${tbb_LIBRARIES})
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

install(TARGETS panopticon LIBRARY DESTINATION lib)
install(FILES ${panopticon_HEADERS} DESTINATION include/panopticon)

if(UNIX AND NOT APPLE)
  configure_file("panopticon.pc.in" "panopticon.pc" @ONLY)
  install(FILES "${CMAKE_CURRENT_BINARY_DIR}/panopticon.pc"
		DESTINATION "lib/pkgconfig")
endif()

if(${GTEST_FOUND})
	add_subdirectory("test")
endif()

#file(GLOB panopticon_SOURCES src/*.cc src/avr/*.cc)
#file(GLOB panopticon_HEADERS include/*.hh include/avr/*.hh)

set(panopticon_SOURCES src/loc.cc src/value.cc src/marshal.cc src/layer.cc)
set(panopticon_HEADERS include/loc.hh include/value.hh include/marshal.hh
	include/layer.hh)

set(panopticon_INCLUDE_DIRS . ${redland_INCLUDE_DIRS}
	${raptor_INCLUDE_DIRS} ${minizip_INCLUDE_DIRS} ${tbb_INCLUDE_DIRS} ${cvc4_INCLUDE_DIRS})
set(panopticon_INCLUDE_DIRS ${panopticon_INCLUDE_DIRS} PARENT_SCOPE)
set(panopticon_LIBRARY_DIRS ${redland_LIBRARY_DIRS}
	${raptor_LIBRARY_DIRS} ${minizip_LIBRARY_DIRS} ${tbb_LIBRARY_DIRS})
set(panopticon_LIBRARY_DIRS ${panopticon_LIBRARY_DIRS} PARENT_SCOPE)
set(panopticon_LIBRARIES panopticon ${cvc4_LIBRARIES} ${redland_LIBRARIES}
	${raptor_LIBRARIES} ${minizip_LIBRARIES} ${tbb_LIBRARIES})
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
	string(REGEX REPLACE "([^ ]+)" "-l\\1" "cvc4_pc_LIBRARIES" "${cvc4_LIBRARIES}")
	string(REGEX REPLACE "([^ ]+)" "-L\\1" "cvc4_pc_LIBRARY_DIRS"
		"${cvc4_LIBRARY_DIRS}")
	string(REGEX REPLACE "([^ ]+)" "-I\\1" "cvc4_pc_INCLUDE_DIRS"
		"${cvc4_INCLUDE_DIRS}")
  configure_file("panopticon.pc.in" "panopticon.pc" @ONLY)
  install(FILES "${CMAKE_CURRENT_BINARY_DIR}/panopticon.pc"
		DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/pkgconfig")
endif()

if(${GTEST_FOUND})
	add_subdirectory("test")
endif()

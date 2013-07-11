cmake_minimum_required(VERSION 2.8)
project(PanopticonLibrary)

file(GLOB panopticon_SOURCES src/*.cc src/avr/*.cc)
file(GLOB panopticon_HEADERS include/*.hh include/avr/*.hh)

set(panopticon_INCLUDE_DIRS ${CMAKE_CURRENT_SOURCE_DIR}/include ${redland_INCLUDE_DIRS}
	${raptor_INCLUDE_DIRS} ${minizip_INCLUDE_DIRS} ${cvc4_INCLUDE_DIRS})
set(panopticon_INCLUDE_DIRS ${panopticon_INCLUDE_DIRS} PARENT_SCOPE)
set(panopticon_LIBRARY_DIRS ${redland_LIBRARY_DIRS}
	${raptor_LIBRARY_DIRS} ${minizip_LIBRARY_DIRS})
set(panopticon_LIBRARY_DIRS ${panopticon_LIBRARY_DIRS} PARENT_SCOPE)
set(panopticon_LIBRARIES panopticon ${cvc4_LIBRARIES} ${redland_LIBRARIES}
	${raptor_LIBRARIES} ${minizip_LIBRARIES})
set(panopticon_LIBRARIES ${panopticon_LIBRARIES} PARENT_SCOPE)

include_directories(${panopticon_INCLUDE_DIRS})
link_directories(${panopticon_LIBRARY_DIRS})
add_library(panopticon SHARED ${panopticon_SOURCES} ${panopticon_HEADERS})
target_link_libraries(panopticon ${panopticon_LIBRARIES})

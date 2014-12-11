cmake_minimum_required(VERSION 2.8.11)
project(PanopticonQt CXX)

set(qtpanopticon_SOURCES src/session.cc src/main.cc
	src/panopticon.cc src/sugiyama.cc)
set(qtpanopticon_HEADERS
	include/dot/dot.hh include/dot/rank.hh include/dot/order.hh
	include/dot/net_flow.hh ${CMAKE_CURRENT_BINARY_DIR}/config.hh)
set(qtpanopticon_HEADERS_QT
	include/panopticon.hh include/session.hh
	include/sugiyama.hh)

set(qtpanopticon_RESOURCES res/resources.qrc)

string(TIMESTAMP qtpanopticon_BUILD_TIME "%Y-%m-%d %H:%M:%S UTC" UTC)
configure_file(include/config.hh.in config.hh)

include_directories(${Boost_INCLUDE_DIRS} ${panopticon_INCLUDE_DIRS})
link_directories(${Boost_LIBRARY_DIRS} ${panopticon_LIBRARY_DIRS})

set(CMAKE_AUTOMOC ON)
set(CMAKE_INCLUDE_CURRENT_DIR ON)
qt5_wrap_cpp(qtpanopticon_HEADERS_MOC ${qtpanopticon_HEADERS_QT})
qt5_add_resources(qtpanopticon_RCCS ${qtpanopticon_RESOURCES})

if("${CMAKE_CXX_COMPILER_ID}" STREQUAL "GNU" OR
	 "${CMAKE_CXX_COMPILER_ID}" STREQUAL "Clang")
	# remove when fixed in Qt5
	add_definitions("-Wno-nested-anon-types")
	set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -iquote ${CMAKE_CURRENT_SOURCE_DIR}/include")
else()
	# MSVC
	include_directories("${CMAKE_CURRENT_SOURCE_DIR}/include")
endif()

add_executable(qtpanopticon ${qtpanopticon_SOURCES} ${qtpanopticon_HEADERS}
	${qtpanopticon_HEADERS_QT} ${qtpanopticon_HEADERS_MOC} ${qtpanopticon_RCCS})
qt5_use_modules(qtpanopticon Widgets Quick Qml Concurrent)
target_link_libraries(qtpanopticon ${QT_LIBRARIES} ${panopticon_LIBRARIES})
set_target_properties(qtpanopticon PROPERTIES VERSION
	${panopticon_MAJOR}.${panopticon_MINOR}.${panopticon_PATCH})
install(TARGETS qtpanopticon RUNTIME DESTINATION bin)

add_subdirectory("test")

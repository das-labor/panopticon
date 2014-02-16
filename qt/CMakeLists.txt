cmake_minimum_required(VERSION 2.8)
project(PanopticonQt CXX)

set(qtpanopticon_SOURCES src/dot/dot.cc src/delegate.cc
	src/selection.cc src/linearview.cc src/main.cc src/session.cc)
set(qtpanopticon_HEADERS include/delegate.hh include/selection.hh
	include/pen.hh
	include/linearview.hh include/session.hh include/dot/adaptor.hh include/dot/astar.hh
	include/dot/dot.hh include/dot/layout.hh include/dot/net_flow.hh
	include/dot/phase1.hh include/dot/phase2.hh include/dot/phase3.hh
	include/dot/traits.hh include/dot/types.hh)
set(qtpanopticon_RESOURCES res/resources.qrc)

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -iquote ${CMAKE_CURRENT_SOURCE_DIR}/include")
include_directories(${Boost_INCLUDE_DIRS} ${panopticon_INCLUDE_DIRS})
link_directories(${Boost_LIBRARY_DIRS} ${panopticon_LIBRARY_DIRS})

set(CMAKE_AUTOMOC ON)
set(CMAKE_INCLUDE_CURRENT_DIR ON)
qt5_wrap_cpp(qtpanopticon_HEADERS_MOC ${qtpanopticon_HEADERS})
qt5_add_resources(qtpanopticon_RCCS ${qtpanopticon_RESOURCES})

# remove when fixed in Qt5
add_definitions("-Wno-nested-anon-types")

add_executable(qtpanopticon ${qtpanopticon_SOURCES} ${qtpanopticon_HEADERS}
	${qtpanopticon_HEADERS_MOC} ${qtpanopticon_RCCS})
qt5_use_modules(qtpanopticon Widgets Quick Qml)
target_link_libraries(qtpanopticon ${QT_LIBRARIES}
	${panopticon_LIBRARIES})
set_target_properties(qtpanopticon PROPERTIES VERSION
	${panopticon_MAJOR}.${panopticon_MINOR}.${panopticon_PATCH})
install(TARGETS qtpanopticon RUNTIME DESTINATION bin)

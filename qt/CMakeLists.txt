cmake_minimum_required(VERSION 2.8)
project(PanopticonQt CXX)

file(GLOB qtpanopticon_SOURCES src/*.cc src/dot/*.cc)
file(GLOB qtpanopticon_HEADERS include/*.hh include/dot/*.hh)
file(GLOB qtpanopticon_RESOURCES res/resources.qrc)

include_directories(include ${Boost_INCLUDE_DIRS} ${libgvc_INCLUDE_DIRS} ${panopticon_INCLUDE_DIRS})
link_directories(${Boost_LIBRARY_DIRS} ${libgvc_LIBRARY_DIRS} ${panopticon_LIBRARY_DIRS})

set(CMAKE_AUTOMOC ON)
set(CMAKE_INCLUDE_CURRENT_DIR ON)
qt5_wrap_cpp(qtpanopticon_HEADERS_MOC ${qtpanopticon_HEADERS})
qt5_add_resources(qtpanopticon_RCCS ${qtpanopticon_RESOURCES})
#include_directories(${Qt5Widgets_INCLUDES})
#add_definitions(${Qt5Widgets_DEFINITIONS})

add_executable(qtpanopticon ${qtpanopticon_SOURCES} ${qtpanopticon_HEADERS}
	${qtpanopticon_HEADERS_MOC} ${qtpanopticon_RCCS})
qt5_use_modules(qtpanopticon Widgets Quick Qml)
target_link_libraries(qtpanopticon ${libgvc_LIBRARIES} ${QT_LIBRARIES}
	${panopticon_LIBRARIES})
set_target_properties(qtpanopticon PROPERTIES VERSION
	${panopticon_MAJOR}.${panopticon_MINOR}.${panopticon_PATCH})
install(TARGETS qtpanopticon RUNTIME DESTINATION bin)

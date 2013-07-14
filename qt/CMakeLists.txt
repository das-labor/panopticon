cmake_minimum_required(VERSION 2.8)
project(PanopticonQt CXX)

file(GLOB qtpanopticon_SOURCES src/*.cc src/dot/*.cc)
file(GLOB qtpanopticon_HEADERS include/*.hh include/dot/*.hh)
file(GLOB qtpanopticon_RESOURCES res/resources.qrc)

include_directories(include ${Boost_INCLUDE_DIRS} ${libgvc_INCLUDE_DIRS} ${panopticon_INCLUDE_DIRS})
link_directories(${Boost_LIBRARY_DIRS} ${libgvc_LIBRARY_DIRS} ${panopticon_LIBRARY_DIRS})

SET(QT_USE_QTDECLARATIVE TRUE)
qt4_wrap_cpp(qtpanopticon_HEADERS_MOC ${qtpanopticon_HEADERS})
qt4_add_resources(qtpanopticon_RCCS ${qtpanopticon_RESOURCES})
include(${QT_USE_FILE})
add_definitions(${QT_DEFINITIONS})

add_executable(qtpanopticon ${qtpanopticon_SOURCES} ${qtpanopticon_HEADERS}
	${qtpanopticon_HEADERS_MOC} ${qtpanopticon_RCCS})
target_link_libraries(qtpanopticon ${libgvc_LIBRARIES} ${QT_LIBRARIES}
	${panopticon_LIBRARIES})
set_target_properties(qtpanopticon PROPERTIES VERSION
	${panopticon_MAJOR}.${panopticon_MINOR}.${panopticon_PATCH})
install(TARGETS qtpanopticon RUNTIME DESTINATION bin)

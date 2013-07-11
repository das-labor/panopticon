cmake_minimum_required(VERSION 2.8)
project(PanopticonQt CXX)

file(GLOB qtpanopticon_SOURCES src/*.cc)
file(GLOB qtpanopticon_HEADERS include/*.hh)

include_directories(include ${Boost_INCLUDE_DIRS} ${libgvc_INCLUDE_DIRS} ${panopticon_INCLUDE_DIRS})
link_directories(${Boost_LIBRARY_DIRS} ${libgvc_LIBRARY_DIRS} ${panopticon_LIBRARY_DIRS})

SET(QT_USE_QTDECLARATIVE TRUE)
qt4_wrap_cpp(qtpanopticon_HEADERS_MOC ${qtpanopticon_HEADERS})
include(${QT_USE_FILE})
add_definitions(${QT_DEFINITIONS})

# Targets
add_executable(qtpanopticon ${qtpanopticon_SOURCES} ${qtpanopticon_HEADERS} ${qtpanopticon_HEADERS_MOC})
target_link_libraries(qtpanopticon ${libgvc_LIBRARIES} ${QT_LIBRARIES} ${panopticon_LIBRARIES})

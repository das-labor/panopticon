cmake_minimum_required(VERSION 2.8)
project(Panopticon CXX)

set(PROJECT_BINARY_DIR build)
set(panopticon_MAJOR 0)
set(panopticon_MINOR 8)
set(panopticon_PATCH 0)
set(panopticon_VERSION
  ${panopticon_MAJOR}.${panopticon_MINOR}.${panopticon_PATCH})
 
# Options
# TODO

# Compiler flags
add_definitions("-std=c++11 -Wall -Wextra -Weffc++ -pedantic
-Wno-unused-parameter")
# remove when fixed in CVC4
add_definitions("-Wno-deprecated") 

if(${CMAKE_BUILD_TYPE} STREQUAL "Release")
	add_definitions("-Werror")
endif()

if(${CMAKE_BUILD_TYPE} STREQUAL "Debug")
	include(CTest)
endif()

# Dependencies
find_package(PkgConfig)
pkg_check_modules(redland REQUIRED redland)
pkg_check_modules(raptor REQUIRED raptor2)
pkg_check_modules(minizip REQUIRED minizip)
find_path(cvc4_INCLUDE_DIRS cvc4/cvc4.h)
find_library(cvc4_LIBRARIES cvc4)
find_package_handle_standard_args(CVC4 REQUIRED_VARS
	cvc4_LIBRARIES cvc4_INCLUDE_DIRS)
find_package(Boost COMPONENTS program_options)
find_package(Qt4 4.7 COMPONENTS QtCore QtDeclarative QtGui)
pkg_check_modules(libgvc libgvc)
pkg_check_modules(cppunit cppunit)

# Targets
add_subdirectory("lib")

if(${Boost_FOUND})
	add_subdirectory("cli")
	add_dependencies(panop panopticon)
endif()

if(${QT4_FOUND} AND ${libgvc_FOUND})
	add_subdirectory("qt")
	add_dependencies(qtpanopticon panopticon)
endif()

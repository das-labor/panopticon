cmake_minimum_required(VERSION 2.8)
project(Panopticon)

set(PROJECT_BINARY_DIR build)
set(PANOPTICON_MAJOR_VERSION 0)
set(PANOPTICON_MINOR_VERSION 8)
set(PANOPTICON_PATCH_VERSION 0)
set(PANOPTICON_VERSION
  ${PANOPTICON_MAJOR_VERSION}.${PANOPTICON_MINOR_VERSION}.${PANOPTICON_PATCH_VERSION})
 
# Offer the user the choice of overriding the installation directories
set(INSTALL_LIB_DIR lib CACHE PATH "/usr/lib")
set(INSTALL_BIN_DIR bin CACHE PATH "/usr/bin")
set(INSTALL_INCLUDE_DIR include CACHE PATH
  "Installation directory for header files")
if(WIN32 AND NOT CYGWIN)
  set(DEF_INSTALL_CMAKE_DIR CMake)
else()
  set(DEF_INSTALL_CMAKE_DIR lib/CMake/FooBar)
endif()
 
# Make relative paths absolute (needed later on)
foreach(p LIB BIN INCLUDE CMAKE)
  set(var INSTALL_${p}_DIR)
  if(NOT IS_ABSOLUTE "${${var}}")
    set(${var} "${CMAKE_INSTALL_PREFIX}/${${var}}")
  endif()
endforeach()

add_definitions("-std=c++11 -Wall -pedantic")
# remove when fixed in CVC4
add_definitions("-Wno-deprecated") 

if(${CMAKE_BUILD_TYPE} EQUAL "Release")
	add_definitions("-Werror")
endif(${CMAKE_BUILD_TYPE} EQUAL "Release")

# lib
file(GLOB lib_SOURCES lib/src/*.cc lib/src/avr/*.cc)
file(GLOB lib_HEADERS lib/include/*.hh lib/include/avr/*.hh)

find_package(PkgConfig)
pkg_check_modules(redland REQUIRED redland)
pkg_check_modules(raptor REQUIRED raptor2)
pkg_check_modules(minizip REQUIRED minizip)
find_path(cvc4_INCLUDE_DIRS cvc4/cvc4.h)
find_library(cvc4_LIBRARY cvc4)
find_package_handle_standard_args(CVC4 REQUIRED_VARS
	cvc4_LIBRARY cvc4_INCLUDE_DIRS)
include_directories(lib/include ${redland_INCLUDE_DIRS} ${raptor_INCLUDE_DIRS}
	${minizip_INCLUDE_DIRS} ${cvc4_INCLUDE_DIRS})

add_library(panopticon SHARED ${lib_SOURCES} ${lib_HEADERS})

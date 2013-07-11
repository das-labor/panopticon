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

# Compiler flags
add_definitions("-std=c++11 -Wall -Wextra -Weffc++ -pedantic")
# remove when fixed in CVC4
add_definitions("-Wno-deprecated") 

if(${CMAKE_BUILD_TYPE} EQUAL "Release")
	add_definitions("-Werror")
endif(${CMAKE_BUILD_TYPE} EQUAL "Release")

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
pkg_check_modules(libgvc REQUIRED libgvc)

# lib
add_subdirectory("lib")

if(${Boost_FOUND})
	add_subdirectory("cli")
	add_dependencies(panop panopticon)
endif()

if(${QT4_FOUND} AND ${libgvc_FOUND})
	add_subdirectory("qt")
	add_dependencies(qtpanopticon panopticon)
endif()

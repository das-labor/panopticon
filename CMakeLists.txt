cmake_minimum_required(VERSION 2.8)
project(Panopticon CXX C)
include(CheckCXXCompilerFlag)

set(panopticon_MAJOR 0)
set(panopticon_MINOR 8)
set(panopticon_PATCH 0)
set(panopticon_VERSION
  ${panopticon_MAJOR}.${panopticon_MINOR}.${panopticon_PATCH})

# Options
option(panopticon_BUILD "Build Panopticon Library (libpanopticon)" ON)
option(qtpanopticon_BUILD "Build Qt UI (qtpanopticon)" ON)
option(panop_BUILD "Build cli tool (panop)" OFF)

set(CMAKE_BUILD_TYPE "Debug")

# Compiler flags
add_definitions("-Wall -Wextra -pedantic
-Wno-unused-parameter")
# remove when fixed in CVC4
add_definitions("-Wno-deprecated")

# Release mode is default
if(NOT DEFINED "CMAKE_BUILD_TYPE" OR "${CMAKE_BUILD_TYPE}" STREQUAL "Release")
	add_definitions("-Werror")
	message("Build release")
elseif("${CMAKE_BUILD_TYPE}" STREQUAL "Debug")
	include(CTest)
	find_package(GTest REQUIRED)
	message(STATUS "Debug build")
endif()

# Check for C++11
CHECK_CXX_COMPILER_FLAG("-std=c++11" CXXFLAG_CPP11)
if(NOT DEFINED "CXXFLAG_CPP11")
	CHECK_CXX_COMPILER_FLAG("-std=c++0x" CXXFLAG_CPP0X)
	if(NOT DEFINED "CXXFLAG_CPP0X")
		message(FATAL_ERROR "Your compiler does not support C++11. Please use clang 3.0 or GCC 4.7.")
	else()
		add_definitions("-std=c++0x")
	endif()
else()
	add_definitions("-std=c++11")
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
find_package(Boost COMPONENTS system REQUIRED)
pkg_check_modules(tbb REQUIRED tbb)
pkg_check_modules(panopticon panopticon)

# Targets
if(${panopticon_BUILD})
	message("** Building Panopticon library")
	add_subdirectory("lib")
else()
	set(panopticon_FOUND FALSE)
endif()

if(${panop_BUILD} AND ${panopticon_FOUND})
	message("** Building Panopticon command line tool")
	add_subdirectory("cli")
	add_dependencies(panop panopticon)
endif()

if(${qtpanopticon_BUILD} AND ${panopticon_FOUND})
	message("** Building Panopticon Qt UI")
	find_package(Qt5 COMPONENTS Widgets Qml Quick REQUIRED)
	add_subdirectory("qt")
	add_dependencies(qtpanopticon panopticon)
endif()

message("** Type 'make' to build.")
message("** You can run the test suite with 'ctest'")

cmake_minimum_required(VERSION 2.8.9)
project(Panopticon CXX)
include(CheckCXXCompilerFlag)

set(panopticon_MAJOR 0)
set(panopticon_MINOR 8)
set(panopticon_PATCH 0)
set(panopticon_VERSION
  ${panopticon_MAJOR}.${panopticon_MINOR}.${panopticon_PATCH})

# Options
option(BUILD_lib "Build Panopticon Library (libpanopticon)" ON)
option(BUILD_qt "Build Qt UI (qtpanopticon)" ON)

# Packaging
set(CPACK_PACKAGE_CONTACT "seu@panopticon.re")
SET(CPACK_PACKAGE_DESCRIPTION_SUMMARY "Cross platform disassembler")
SET(CPACK_PACKAGE_VENDOR "http://panopticon.re")
SET(CPACK_PACKAGE_DESCRIPTION_FILE "${CMAKE_CURRENT_SOURCE_DIR}/README")
SET(CPACK_RESOURCE_FILE_LICENSE "${CMAKE_CURRENT_SOURCE_DIR}/LICENSE")
SET(CPACK_PACKAGE_VERSION_MAJOR ${panopticon_MAJOR})
SET(CPACK_PACKAGE_VERSION_MINOR ${panopticon_MINOR})
SET(CPACK_PACKAGE_VERSION_PATCH ${panopticon_PATCH})
set(CPACK_GENERATOR "STGZ;TGZ;DEB;RPM")
include(CPack)

# Default to debug build
set(CMAKE_BUILD_TYPE "Debug")

# find_package macros for librdf, raptor and panopticon
set(CMAKE_MODULE_PATH "${CMAKE_MODULE_PATH};${CMAKE_CURRENT_SOURCE_DIR}/cmake")

# Compiler flags
if("${CMAKE_CXX_COMPILER_ID}" STREQUAL "GNU" OR
	 "${CMAKE_CXX_COMPILER_ID}" STREQUAL "Clang")
	set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Wextra -pedantic -Wno-unused-parameter")
endif()

set(CMAKE_SHARED_LINKER_FLAGS "${CMAKE_SHARED_LINKER_FLAGS} -Wl,--warn-unresolved-symbols,--warn-once")

if("${CMAKE_BUILD_TYPE}" STREQUAL "Release")
 	set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Werror")
	message(STATUS "Build release")
elseif("${CMAKE_BUILD_TYPE}" STREQUAL "Debug")

	# enable tests
	include(CTest)
	find_package(GTest REQUIRED)
	include(PanopticonTest)

	# compile and link with coverage info
	if("${CMAKE_CXX_COMPILER_ID}" STREQUAL "GNU" AND ${UNIX})
		set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} --coverage")
	else()
		message(STATUS "Coverage testing disabled when not using GCC on Unix")
	endif()

	message(STATUS "Debug build")
endif()

# Check for C++11
if(NOT "${CMAKE_CXX_COMPILER_ID}" STREQUAL "MSVC")
	CHECK_CXX_COMPILER_FLAG("-std=c++11" CXXFLAG_CPP11)
	if(NOT DEFINED "CXXFLAG_CPP11")
		message(FATAL_ERROR "Your compiler does not support C++11. Please use clang 3.4 or GCC 4.7.")
	else()
 		set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++11")
	endif()
endif()

# Dependencies
find_package(LibArchive REQUIRED)
find_package(Boost 1.52 COMPONENTS system filesystem graph REQUIRED)
find_package(KyotoCabinet REQUIRED)

# Targets
if(${BUILD_lib})
	message(STATUS "Building Panopticon library")
	add_subdirectory("lib")
	set(panopticon_FOUND TRUE)
else()
	find_package(panopticon)
endif()

if(${BUILD_qt} AND ${panopticon_FOUND})
	message(STATUS "Building Panopticon Qt UI")
	find_package(Qt5 COMPONENTS Widgets Qml Quick REQUIRED)
	add_subdirectory("qt")
	add_dependencies(qtpanopticon panopticon)
endif()

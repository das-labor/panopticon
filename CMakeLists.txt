cmake_minimum_required(VERSION 2.8.9)
project(Panopticon CXX)
include(CheckCXXCompilerFlag)

set(panopticon_MILESTONE "Bastille")
set(panopticon_MAJOR 0)
set(panopticon_MINOR 10)
set(panopticon_PATCH 0)
set(panopticon_VERSION
  ${panopticon_MAJOR}.${panopticon_MINOR}.${panopticon_PATCH}-dev)

# Options
set(BUILD_lib "AUTO" CACHE STRING "Build Panopticon Library (libpanopticon)")
set(BUILD_qt "AUTO" CACHE STRING "Build Qt UI (qtpanopticon)")
set(BUILD_doc "AUTO" CACHE STRING "Build documentation")

# Default to release build
if(NOT CMAKE_BUILD_TYPE AND NOT CMAKE_CONFIGURATION_TYPES)
	set(CMAKE_BUILD_TYPE Release)
endif()

# find_package macros for kyotocabinet, sphinx and panopticon
set(CMAKE_MODULE_PATH "${CMAKE_MODULE_PATH};${CMAKE_CURRENT_SOURCE_DIR}/cmake")

set_property(GLOBAL PROPERTY USE_FOLDERS ON)

# build type-depend
if("${CMAKE_BUILD_TYPE}" STREQUAL "Release")
 	set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Werror")
	message(STATUS "Build release")
elseif("${CMAKE_BUILD_TYPE}" STREQUAL "Debug")
	# enable tests
	include(CTest)
	set(gtest_force_shared_crt ON CACHE FORCE BOOL)
	add_subdirectory("3rdparty/gtest")
	include(PanopticonTest)
	include_directories("3rdparty/gtest/include")
	link_directories(${gtest_BINARY_DIR}/src)

	# compile and link with coverage info
	if("${CMAKE_CXX_COMPILER_ID}" STREQUAL "GNU" AND "${UNIX}")
		set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} --coverage")
	else()
		message(STATUS "Coverage testing disabled when not using GCC on Unix")
	endif()

	message(STATUS "Debug build")
endif()

# Compiler flags
if("${CMAKE_CXX_COMPILER_ID}" STREQUAL "GNU" OR
	"${CMAKE_CXX_COMPILER_ID}" STREQUAL "Clang")
 set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Wextra -pedantic")
 set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wno-unused-parameter")
 set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wno-maybe-uninitialized")
 set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wno-unknown-warning-option")
	set(CMAKE_SHARED_LINKER_FLAGS "${CMAKE_SHARED_LINKER_FLAGS} -Wl,--warn-unresolved-symbols,--warn-once")
endif()

if("${CMAKE_CXX_COMPILER_ID}" STREQUAL "MSVC")
	set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} /bigobj")
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

# Defines
add_definitions("-DBOOST_MPL_CFG_NO_PREPROCESSED_HEADERS")
add_definitions("-DBOOST_MPL_LIMIT_LIST_SIZE=30")
if("${CMAKE_CXX_COMPILER_ID}" STREQUAL "MSVC")
	add_definitions("-DNOMINMAX")
endif()


# Dependencies
find_package(LibArchive REQUIRED)
find_package(Boost 1.52 COMPONENTS system filesystem graph REQUIRED)
find_package(KyotoCabinet REQUIRED)
find_program(AVRA avra)
find_program(NASM nasm)

if(NOT INCLUDE_INSTALL_DIR)
	set(INCLUDE_INSTALL_DIR "include")
endif()

if(NOT LIB_INSTALL_DIR)
	set(LIB_INSTALL_DIR "lib")
endif()

# Targets
if(${BUILD_lib})
	message(STATUS "Building Panopticon library")
	add_subdirectory("lib")
	set(Panopticon_FOUND TRUE)
elseif(${BUILD_lib} STREQUAL "AUTO")
	find_package(Panopticon)

	if(NOT "${Panopticon_FOUND}")
		message(STATUS "Building Panopticon library")
		add_subdirectory("lib")
		set(Panopticon_FOUND TRUE)
	endif()
else()
	find_package(Panopticon REQUIRED)
endif()

if(${BUILD_qt})
	find_package(Qt5Quick REQUIRED)
	find_package(Qt5Concurrent REQUIRED)
	find_package(Qt5Widgets REQUIRED)
	find_package(Qt5Qml REQUIRED)

	message(STATUS "Building Panopticon Qt UI")
	add_subdirectory("qt")
	add_dependencies(qtpanopticon panopticon)
elseif(${BUILD_qt} STREQUAL "AUTO")
	find_package(Qt5Quick)
	find_package(Qt5Concurrent)
	find_package(Qt5Widgets)
	find_package(Qt5Qml)

	if(${Qt5Qml_FOUND} AND ${Qt5Widgets_FOUND} AND ${Qt5Concurrent_FOUND} AND ${Qt5Quick_FOUND})
		message(STATUS "Building Panopticon Qt UI")
		add_subdirectory("qt")
		add_dependencies(qtpanopticon panopticon)
	endif()
endif()

if(${BUILD_doc})
	find_package(Sphinx REQUIRED)
	message(STATUS "Building Panopticon documentation")
	add_subdirectory(doc)
elseif(${BUILD_doc} STREQUAL "AUTO")
	find_package(Sphinx)

	if(SPHINX_EXECUTABLE)
		message(STATUS "Building Panopticon documentation")
		add_subdirectory(doc)
	endif()
endif()

cmake_minimum_required(VERSION 3.1)

# Add Coverage option
option(ENABLE_COVERAGE "Enable coverage" OFF)

# Add additional source path for cmake
set(CMAKE_MODULE_PATH ${CMAKE_MODULE_PATH} ${PROJECT_SOURCE_DIR}/cmake/)

# Add strict warning checking for C++
if (CMAKE_COMPILER_IS_GNUCC OR CMAKE_COMPILER_IS_GNUCXX)
  set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Wno-long-long -pedantic")
endif()

if (ENABLE_COVERAGE)
  message(STATUS "Enabling coverage")
  set(CMAKE_BUILD_TYPE Debug)
  set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -g -O0")
  set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} --coverage")
endif()

# Add additional source path for cmake
set(CMAKE_MODULE_PATH ${CMAKE_MODULE_PATH} ${PROJECT_SOURCE_DIR}/cmake/)

#add_subdirectory(doc)
add_subdirectory(lib)
#add_subdirectory(test)

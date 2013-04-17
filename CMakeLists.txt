cmake_minimum_required(VERSION 2.8)
project(Panopticum)

set(PROJECT_BINARY_DIR build)
set(PANOPTICUM_MAJOR_VERSION 0)
set(PANOPTICUM_MINOR_VERSION 0)
set(PANOPTICUM_PATCH_VERSION 1)
set(PANOPTICUM_VERSION
  ${PANOPTICUM_MAJOR_VERSION}.${PANOPTICUM_MINOR_VERSION}.${PANOPTICUM_PATCH_VERSION})
 
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
 
# set up include-directories
include_directories(
  "${PROJECT_SOURCE_DIR}"
  "${PROJECT_BINARY_DIR}")
 
# Add sub-directories
add_subdirectory(lib)
add_subdirectory(cli)
add_subdirectory(qt)
add_subdirectory(doc)
 
# The interesting stuff goes here
# ===============================
 
# Add all targets to the build-tree export set
export(TARGETS lib cli qt doc
  FILE "${PROJECT_BINARY_DIR}/build.cmake")
 
# Export the package for use from the build-tree
# (this registers the build-tree with a global CMake-registry)
export(PACKAGE Panopticum)
 
# Create the FooBarConfig.cmake and FooBarConfigVersion files
file(RELATIVE_PATH REL_INCLUDE_DIR "${INSTALL_CMAKE_DIR}"
   "${INSTALL_INCLUDE_DIR}")
# ... for the build tree
set(CONF_INCLUDE_DIRS "${PROJECT_SOURCE_DIR}" "${PROJECT_BINARY_DIR}")
configure_file(FooBarConfig.cmake.in
  "${PROJECT_BINARY_DIR}/FooBarConfig.cmake" @ONLY)
# ... for the install tree
set(CONF_INCLUDE_DIRS "\${FOOBAR_CMAKE_DIR}/${REL_INCLUDE_DIR}")
configure_file(FooBarConfig.cmake.in
  "${PROJECT_BINARY_DIR}${CMAKE_FILES_DIRECTORY}/FooBarConfig.cmake" @ONLY)
# ... for both
configure_file(FooBarConfigVersion.cmake.in
  "${PROJECT_BINARY_DIR}/FooBarConfigVersion.cmake" @ONLY)
 
# Install the FooBarConfig.cmake and FooBarConfigVersion.cmake
install(FILES
  "${PROJECT_BINARY_DIR}${CMAKE_FILES_DIRECTORY}/FooBarConfig.cmake"
  "${PROJECT_BINARY_DIR}/FooBarConfigVersion.cmake"
  DESTINATION "${INSTALL_CMAKE_DIR}" COMPONENT dev)
 
# Install the export set for use with the install-tree
install(EXPORT FooBarTargets DESTINATION
  "${INSTALL_CMAKE_DIR}" COMPONENT dev)

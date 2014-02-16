# - Try to find redland (librdf)
# Once done this will define
#  redland_FOUND - System has redland
#  redland_INCLUDE_DIRS - The redland include directories
#  redland_LIBRARIES - The libraries needed to use redland
#  redland_DEFINITIONS - Compiler switches required for using redland

if(UNIX AND NOT APPLE)
	find_package(PkgConfig)
	pkg_check_modules(PC_redland QUIET redland)
	set(redland_DEFINITIONS ${PC_redland_CFLAGS_OTHER})
endif()

find_path(redland_INCLUDE_DIR librdf.h
          HINTS ${PC_redland_INCLUDEDIR} ${PC_redland_INCLUDE_DIRS})

find_library(redland_LIBRARY NAMES rdf librdf
             HINTS ${PC_redland_LIBDIR} ${PC_redland_LIBRARY_DIRS} )

set(redland_LIBRARIES ${redland_LIBRARY})
set(redland_INCLUDE_DIRS ${redland_INCLUDE_DIR})

include(FindPackageHandleStandardArgs)
# handle the QUIETLY and REQUIRED arguments and set redland_FOUND to TRUE
# if all listed variables are TRUE
find_package_handle_standard_args(redland DEFAULT_MSG
                                  redland_LIBRARY redland_INCLUDE_DIR)

mark_as_advanced(redland_INCLUDE_DIR redland_LIBRARY)

# - Try to find rasqal
# Once done this will define
#  rasqal_FOUND - System has rasqal
#  rasqal_INCLUDE_DIRS - The rasqal include directories
#  rasqal_LIBRARIES - The libraries needed to use rasqal
#  rasqal_DEFINITIONS - Compiler switches required for using rasqal

if(UNIX AND NOT APPLE)
	find_package(PkgConfig)
	pkg_check_modules(PC_rasqal QUIET rasqal)
	set(rasqal_DEFINITIONS ${PC_rasqal_CFLAGS_OTHER})
endif()

find_path(rasqal_INCLUDE_DIR rasqal.h
          HINTS ${PC_rasqal_INCLUDEDIR} ${PC_rasqal_INCLUDE_DIRS})

find_library(rasqal_LIBRARY NAMES rasqal
             HINTS ${PC_rasqal_LIBDIR} ${PC_rasqal_LIBRARY_DIRS} )

set(rasqal_LIBRARIES ${rasqal_LIBRARY})
set(rasqal_INCLUDE_DIRS ${rasqal_INCLUDE_DIR})

include(FindPackageHandleStandardArgs)
# handle the QUIETLY and REQUIRED arguments and set rasqal_FOUND to TRUE
# if all listed variables are TRUE
find_package_handle_standard_args(rasqal DEFAULT_MSG
                                  rasqal_LIBRARY rasqal_INCLUDE_DIR)

mark_as_advanced(rasqal_INCLUDE_DIR rasqal_LIBRARY)

# - Try to find raptor2
# Once done this will define
#  raptor_FOUND - System has raptor
#  raptor_INCLUDE_DIRS - The raptorinclude directories
#  raptor_LIBRARIES - The libraries needed to use raptor
#  raptor_DEFINITIONS - Compiler switches required for using raptor

if(UNIX AND NOT APPLE)
	find_package(PkgConfig)
	pkg_check_modules(PC_raptor QUIET raptor2)
	set(raptor_DEFINITIONS ${PC_raptor_CFLAGS_OTHER})
endif()

find_path(raptor_INCLUDE_DIR raptor2/raptor2.h
          HINTS ${PC_raptor_INCLUDEDIR} ${PC_raptor_INCLUDE_DIRS}
					PATH_SUFFIXES raptor2)

find_library(raptor_LIBRARY NAMES raptor2
             HINTS ${PC_raptor_LIBDIR} ${PC_raptor_LIBRARY_DIRS} )

set(raptor_LIBRARIES ${raptor_LIBRARY})
set(raptor_INCLUDE_DIRS ${raptor_INCLUDE_DIR})

include(FindPackageHandleStandardArgs)
# handle the QUIETLY and REQUIRED arguments and set raptor_FOUND to TRUE
# if all listed variables are TRUE
find_package_handle_standard_args(raptor DEFAULT_MSG
                                  raptor_LIBRARY raptor_INCLUDE_DIR)

mark_as_advanced(raptor_INCLUDE_DIR raptor_LIBRARY)

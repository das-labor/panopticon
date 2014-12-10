# - Try to find Panopticon
# Once done this will define
#  Panopticon_FOUND - System has Panopticon
#  Panopticon_INCLUDE_DIRS - The Panopticon include directories
#  Panopticon_LIBRARIES - The libraries needed to use Panopticon
#  Panopticon_DEFINITIONS - Compiler switches required for using Panopticon

find_package(PkgConfig)
pkg_check_modules(PC_Panopticon QUIET panopticon)
set(Panopticon_DEFINITIONS ${PC_Panopticon_CFLAGS_OTHER})

find_path(Panopticon_INCLUDE_DIR kcdb.h
          HINTS ${PC_Panopticon_INCLUDEDIR} ${PC_Panopticon_INCLUDE_DIRS}
          PATH_SUFFIXES libxml2 )

find_library(Panopticon_LIBRARY NAMES libkyotocabinet kyotocabinet
             HINTS ${PC_Panopticon_LIBDIR} ${PC_Panopticon_LIBRARY_DIRS} )

set(Panopticon_LIBRARIES ${Panopticon_LIBRARY} )
set(Panopticon_INCLUDE_DIRS ${Panopticon_INCLUDE_DIR} )

include(FindPackageHandleStandardArgs)
# handle the QUIETLY and REQUIRED arguments and set Panopticon_FOUND to TRUE
# if all listed variables are TRUE
find_package_handle_standard_args("Panopticon" DEFAULT_MSG
                                  Panopticon_LIBRARY Panopticon_INCLUDE_DIR)

mark_as_advanced(Panopticon_INCLUDE_DIR Panopticon_LIBRARY )

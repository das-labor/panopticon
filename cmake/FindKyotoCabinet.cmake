# - Try to find Kyoto Cabinet
# Once done this will define
#  KyotoCabinet_FOUND - System has Kyoto Cabinet
#  KyotoCabinet_INCLUDE_DIRS - The Kyoto Cabinet include directories
#  KyotoCabinet_LIBRARIES - The libraries needed to use Kyoto Cabinet
#  KyotoCabinet_DEFINITIONS - Compiler switches required for using Kyoto Cabinet

find_package(PkgConfig)
pkg_check_modules(PC_KyotoCabinet QUIET kyotocabinet)
set(KyotoCabinet_DEFINITIONS ${PC_KyotoCabinet_CFLAGS_OTHER})

find_path(KyotoCabinet_INCLUDE_DIR kcdb.h
          HINTS ${PC_KyotoCabinet_INCLUDEDIR} ${PC_KyotoCabinet_INCLUDE_DIRS})

find_library(KyotoCabinet_LIBRARY NAMES libkyotocabinet kyotocabinet
             HINTS ${PC_KyotoCabinet_LIBDIR} ${PC_KyotoCabinet_LIBRARY_DIRS})

set(KyotoCabinet_LIBRARIES ${KyotoCabinet_LIBRARY})
set(KyotoCabinet_INCLUDE_DIRS ${KyotoCabinet_INCLUDE_DIR})

include(FindPackageHandleStandardArgs)
# handle the QUIETLY and REQUIRED arguments and set KyotoCabinet_FOUND to TRUE
# if all listed variables are TRUE
find_package_handle_standard_args("Kyoto Cabinet" DEFAULT_MSG
                                  KyotoCabinet_LIBRARY KyotoCabinet_INCLUDE_DIR)

mark_as_advanced(KyotoCabinet_INCLUDE_DIR KyotoCabinet_LIBRARY)

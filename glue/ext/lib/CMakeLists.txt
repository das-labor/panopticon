project(panopticon-glue)

set(CMAKE_INCLUDE_CURRENT_DIR ON)
set(CMAKE_AUTOMOC ON)

if (WIN32)
  add_definitions(-DWIN32)
endif()

find_package(Qt5Core)
find_package(Qt5Qml)
find_package(Qt5Quick)
find_package(Qt5Svg)

set(HEADERS_LIST
  include/glue.h
  include/qpanopticon.h
  include/qcontrolflowgraph.h
  include/qsidebar.h
  include/qbasicblockline.h
  include/qrecentsession.h
  )

set(SRC_LIST
  src/glue.cpp
  src/qpanopticon.cpp
  src/qcontrolflowgraph.cpp
  src/qsidebar.cpp
  src/qbasicblockline.cpp
  src/qrecentsession.cpp
  )

include_directories(include include/Qt)

set(major 0)
set(minor 0)
set(patch 0)

add_library("${PROJECT_NAME}" ${SRC_LIST} ${HEADERS_LIST})
set_property(TARGET ${PROJECT_NAME} PROPERTY CXX_STANDARD 14)
target_link_libraries("${PROJECT_NAME}" PRIVATE Qt5::Core Qt5::Gui
	Qt5::Svg Qt5::Qml Qt5::Quick)

# Install directive for binaries
install(TARGETS ${PROJECT_NAME}
  RUNTIME DESTINATION bin
  LIBRARY DESTINATION lib
	ARCHIVE DESTINATION lib
  )

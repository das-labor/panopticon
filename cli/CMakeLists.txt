cmake_minimum_required(VERSION 2.8)
project(PanopticonCli)

file(GLOB cli_SOURCES src/*.cc)
file(GLOB cli_HEADERS include/*.hh)

include_directories(include ${Boost_INCLUDE_DIRS} ${panopticon_INCLUDE_DIRS})
link_directories(${Boost_LIBRARY_DIRS} ${panopticon_LIBRARY_DIRS})

add_executable(panop ${cli_SOURCES} ${cli_HEADERS})
target_link_libraries(panop ${panopticon_LIBRARIES} ${Boost_LIBRARIES})

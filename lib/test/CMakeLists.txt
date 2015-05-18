set(panopticon_test_HEADER architecture.hh ${panopticon_HEADERS})
set(panopticon_test_SOURCES layer.cc loc.cc region.cc testsuite.cc
	value.cc digraph.cc disassembler.cc procedure.cc
	architecture.cc store.cc marshal.cc tree.cc mnemonic.cc basic_block.cc
	program.cc dflow.cc interpreter.cc structure.cc database.cc session.cc
	amd64.cc avr.cc)

set(panopticon_test_avr_SOURCES data/avr-all-opcodes-01.asm
	data/avr-all-opcodes-02.asm data/avr-all-opcodes-03.asm
	data/avr-all-opcodes-04.asm data/avr-all-opcodes-05.asm
	data/avr-all-opcodes-06.asm data/avr-all-opcodes-07.asm
	data/avr-all-opcodes-08.asm data/avr-all-opcodes-09.asm
	data/avr-all-opcodes-10.asm data/avr-all-opcodes-11.asm
	data/avr-all-opcodes-12.asm data/avr-all-opcodes-13.asm
	data/avr-all-opcodes-14.asm data/avr-all-opcodes-15.asm
	data/avr-jmp-overflow.asm)
set(panopticon_test_verbatim_SOURCES data/empty.panop data/save.panop data/test.exe data/sosse)

include_directories(${GTEST_INCLUDE_DIRS})
link_directories(${panopticon_LIBRARY_DIRS} ${GTEST_LIBRARY_DIRS})

add_custom_target(verbatim-testfiles)
add_custom_target(avr-testfiles)

# copy verbatim test files
foreach(f ${panopticon_test_verbatim_SOURCES})
	add_custom_command(
		TARGET verbatim-testfiles
		COMMAND ${CMAKE_COMMAND} -E make_directory ${CMAKE_CURRENT_BINARY_DIR}
		COMMAND ${CMAKE_COMMAND} -E copy ${CMAKE_CURRENT_SOURCE_DIR}/${f} ${CMAKE_CURRENT_BINARY_DIR}/${f})
endforeach()

# assemble AVR test files
if(NOT AVRA STREQUAL "AVRA-NOTFOUND")
	message("-- Found AVRA: ${AVRA}")
	message("-- Building AVR tests")

	add_definitions("-DHAVE_TESTFILES_AVR")

	foreach(f ${panopticon_test_avr_SOURCES})
		add_custom_command(
			TARGET verbatim-testfiles
			COMMAND ${CMAKE_COMMAND} -E copy ${CMAKE_CURRENT_SOURCE_DIR}/${f} ${CMAKE_CURRENT_BINARY_DIR}/${f}
			COMMAND ${AVRA} ${CMAKE_CURRENT_BINARY_DIR}/${f} > /dev/null
			MAIN_DEPENDENCY "${f}"
			COMMENT "Assembling ${f}"
			WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR})
	endforeach()
endif()

add_executable(testsuite ${panopticon_test_HEADERS}
	${panopticon_test_SOURCES})
add_dependencies(testsuite verbatim-testfiles avr-testfiles)
add_definitions("-DTESTDATA_DIR=\"${CMAKE_CURRENT_BINARY_DIR}/data/\"")

target_link_libraries(testsuite gtest_main gtest
	${panopticon_STATIC_LIBRARIES})
panopticon_add_tests(testsuite "${CMAKE_CURRENT_BINARY_DIR}" "" "${panopticon_test_SOURCES}")

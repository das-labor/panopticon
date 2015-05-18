set(panopticon_test_verbatim_SOURCES empty.panop save.panop test.exe sosse)
set(panopticon_test_avr_SOURCES avr/all-01.asm
	avr/all-02.asm avr/all-03.asm
	avr/all-04.asm avr/all-05.asm
	avr/all-06.asm avr/all-07.asm
	avr/all-08.asm avr/all-09.asm
	avr/all-10.asm avr/all-11.asm
	avr/all-12.asm avr/all-13.asm
	avr/all-14.asm avr/all-15.asm
	avr/jmp-overflow.asm)
set(panopticon_test_ia32_SOURCES data/ia32.asm)
set(panopticon_test_amd64_SOURCES data/amd64.asm)

add_definitions("-DTESTDATA_DIR=\"${CMAKE_CURRENT_BINARY_DIR}/data/\"")

# copy verbatim test files
foreach(f ${panopticon_test_verbatim_SOURCES})
	add_custom_command(
		OUTPUT ${CMAKE_CURRENT_BINARY_DIR}/${f}
		MAIN_DEPENDENCY ${CMAKE_CURRENT_SOURCE_DIR}/${f}
		COMMAND ${CMAKE_COMMAND} -E make_directory ${CMAKE_CURRENT_BINARY_DIR}
		COMMAND ${CMAKE_COMMAND} -E copy ${CMAKE_CURRENT_SOURCE_DIR}/${f} ${CMAKE_CURRENT_BINARY_DIR}/${f}
		COMMENT "Copying ${f}")
	set(panopticon_test_verbatim_OBJECTS ${panopticon_test_verbatim_OBJECTS} ${CMAKE_CURRENT_BINARY_DIR}/${f})
endforeach()

# assemble AVR test files
if(NOT AVRA STREQUAL "AVRA-NOTFOUND")
	message("-- Found AVRA: ${AVRA}")
	message("-- Building AVR tests")

	add_definitions("-DHAVE_TESTFILES_AVR")

	foreach(f ${panopticon_test_avr_SOURCES})
		string(REGEX REPLACE ".asm$" ".obj" t ${f})

		add_custom_command(
			OUTPUT ${CMAKE_CURRENT_BINARY_DIR}/${t}
			MAIN_DEPENDENCY ${CMAKE_CURRENT_SOURCE_DIR}/${f}
			COMMAND ${CMAKE_COMMAND} -E copy ${CMAKE_CURRENT_SOURCE_DIR}/${f} ${CMAKE_CURRENT_BINARY_DIR}/${f}
			COMMAND ${AVRA} ${CMAKE_CURRENT_BINARY_DIR}/${f} > /dev/null
			COMMENT "Assembling AVR object ${CMAKE_CURRENT_BINARY_DIR}/${f}"
			WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR})
		set(panopticon_test_avr_OBJECTS ${panopticon_test_avr_OBJECTS} ${CMAKE_CURRENT_BINARY_DIR}/${t})
	endforeach()
endif()

# assemble intel test files
if(NOT NASM STREQUAL "NASM-NOTFOUND")
	message("-- Found NASM: ${NASM}")
	message("-- Building x86 tests")

	add_definitions("-DHAVE_TESTFILES_INTEL")

	foreach(f ${panopticon_test_ia32_SOURCES})
		add_custom_command(
			TARGET intel-testfiles
			COMMAND ${NASM} -f bin ${CMAKE_CURRENT_SOURCE_DIR}/${f} -o ${CMAKE_CURRENT_BINARY_DIR}/${f}.bin
			MAIN_DEPENDENCY "${f}"
			COMMENT "Assembling ${f}")
	endforeach()

	foreach(f ${panopticon_test_amd64_SOURCES})
		add_custom_command(
			TARGET intel-testfiles
			COMMAND ${NASM} -f bin ${CMAKE_CURRENT_SOURCE_DIR}/${f} -o ${CMAKE_CURRENT_BINARY_DIR}/${f}.bin
			MAIN_DEPENDENCY "${f}"
			COMMENT "Assembling ${f}")
	endforeach()
endif()

add_custom_target(testdata DEPENDS ${panopticon_test_verbatim_OBJECTS} ${panopticon_test_avr_OBJECTS})

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
set(panopticon_test_x86_SOURCES x86/ia32.asm x86/amd64.asm)

# copy verbatim test files
foreach(f ${panopticon_test_verbatim_SOURCES})
	add_custom_command(
		OUTPUT ${CMAKE_CURRENT_BINARY_DIR}/${f}
		MAIN_DEPENDENCY ${CMAKE_CURRENT_SOURCE_DIR}/${f}
		COMMAND ${CMAKE_COMMAND} -E make_directory ${CMAKE_CURRENT_BINARY_DIR}
		COMMAND ${CMAKE_COMMAND} -E copy ${CMAKE_CURRENT_SOURCE_DIR}/${f} ${CMAKE_CURRENT_BINARY_DIR}/${f}
		COMMENT "Generating object ${CMAKE_CURRENT_BINARY_DIR}/${f}")
	set(panopticon_test_verbatim_OBJECTS ${panopticon_test_verbatim_OBJECTS} ${CMAKE_CURRENT_BINARY_DIR}/${f})
endforeach()

# assemble AVR test files
if(NOT AVRA STREQUAL "AVRA-NOTFOUND")
	message("-- Found AVRA: ${AVRA}")
	message("-- Building AVR tests")

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

	foreach(f ${panopticon_test_x86_SOURCES})
		string(REGEX REPLACE ".asm$" ".bin" t ${f})
		get_filename_component(d ${t} DIRECTORY)

		add_custom_command(
			OUTPUT ${CMAKE_CURRENT_BINARY_DIR}/${t}
			MAIN_DEPENDENCY ${CMAKE_CURRENT_SOURCE_DIR}/${f}
			COMMAND ${CMAKE_COMMAND} -E make_directory ${d}
			COMMAND ${NASM} -f bin ${CMAKE_CURRENT_SOURCE_DIR}/${f} -o ${CMAKE_CURRENT_BINARY_DIR}/${t}
			COMMENT "Assembling x86 object ${CMAKE_CURRENT_BINARY_DIR}/${t}")
		set(panopticon_test_x86_OBJECTS ${panopticon_test_x86_OBJECTS} ${CMAKE_CURRENT_BINARY_DIR}/${t})
	endforeach()
endif()

add_custom_target(testdata DEPENDS ${panopticon_test_verbatim_OBJECTS} ${panopticon_test_avr_OBJECTS} ${panopticon_test_x86_OBJECTS})

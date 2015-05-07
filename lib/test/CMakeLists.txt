set(panopticon_test_HEADER architecture.hh ${panopticon_HEADERS})
set(panopticon_test_SOURCES layer.cc loc.cc region.cc testsuite.cc
	value.cc digraph.cc disassembler.cc procedure.cc
	architecture.cc store.cc marshal.cc tree.cc mnemonic.cc basic_block.cc
	program.cc dflow.cc interpreter.cc structure.cc database.cc session.cc
	amd64.cc)

include_directories(${GTEST_INCLUDE_DIRS})
link_directories(${panopticon_LIBRARY_DIRS} ${GTEST_LIBRARY_DIRS})

add_executable(testsuite ${panopticon_test_HEADERS}
	${panopticon_test_SOURCES})

if(NOT "${CMAKE_CXX_COMPILER_ID}" STREQUAL "MSVC")
	enable_language(ASM)
	set_property(SOURCE amd64.s PROPERTY LANGUAGE ASM)
	set_property(SOURCE ia32.s PROPERTY LANGUAGE ASM)

	add_executable(amd64-testelf amd64.s)
	set_target_properties(amd64-testelf PROPERTIES COMPILE_FLAGS "-nostdlib -m64")
	set_target_properties(amd64-testelf PROPERTIES LINK_FLAGS "-Ttext 0x0 -nostdlib -m64")

	add_executable(ia32-testelf ia32.s)
	set_target_properties(ia32-testelf PROPERTIES COMPILE_FLAGS "-nostdlib -m32")
	set_target_properties(ia32-testelf PROPERTIES LINK_FLAGS "-Ttext 0x0 -nostdlib -m32")


	add_custom_target(amd64-testraw COMMAND objcopy -Obinary amd64-testelf
		amd64-testraw DEPENDS amd64-testelf)
	add_custom_target(ia32-testraw COMMAND objcopy -Obinary ia32-testelf
		ia32-testraw DEPENDS ia32-testelf)
	add_dependencies(testsuite amd64-testraw ia32-testraw)
endif()

target_link_libraries(testsuite gtest_main gtest
	${panopticon_LIBRARIES})
panopticon_add_tests(testsuite "${CMAKE_CURRENT_BINARY_DIR}" "" "${panopticon_test_SOURCES}")

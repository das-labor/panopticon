# PANOPTICON_ADD_TESTS(executable workdir extra_args ARGN)
#    executable = The path to the test executable
#    workdir    = Working directory
#    extra_args = Pass a list of extra arguments to be passed to
#                 executable enclosed in quotes (or "" for none)
#    ARGN =       A list of source files to search for tests & test
#                 fixtures.
#
#  Example:
#     set(FooTestArgs --foo 1 --bar 2)
#     add_executable(FooTest FooUnitTest.cc)
#     GTEST_ADD_TESTS(FooTest "foo/test" "${FooTestArgs}" FooUnitTest.cc)
#CMake - Cross Platform Makefile Generator
#Copyright 2000-2014 Kitware, Inc.
#Copyright 2000-2011 Insight Software Consortium
#All rights reserved.
#
#Redistribution and use in source and binary forms, with or without
#modification, are permitted provided that the following conditions
#are met:
#
#* Redistributions of source code must retain the above copyright
#  notice, this list of conditions and the following disclaimer.
#
#* Redistributions in binary form must reproduce the above copyright
#  notice, this list of conditions and the following disclaimer in the
#  documentation and/or other materials provided with the distribution.
#
#* Neither the names of Kitware, Inc., the Insight Software Consortium,
#  nor the names of their contributors may be used to endorse or promote
#  products derived from this software without specific prior written
#  permission.
#
#THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
#"AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
#LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
#A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
#HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
#SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
#LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
#DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
#THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
#(INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
#OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
#
#------------------------------------------------------------------------------
#
#The above copyright and license notice applies to distributions of
#CMake in source and binary form. Some source files contain additional
#notices of original copyright by their contributors; see each source
#for details. Third-party software packages supplied with CMake under
#compatible licenses provide their own copyright notices documented in
#corresponding subdirectories.
#
#------------------------------------------------------------------------------
#
#CMake was initially developed by Kitware with the following sponsorship:
#
# * National Library of Medicine at the National Institutes of Health
#   as part of the Insight Segmentation and Registration Toolkit (ITK).
#
# * US National Labs (Los Alamos, Livermore, Sandia) ASC Parallel
#   Visualization Initiative.
#
# * National Alliance for Medical Image Computing (NAMIC) is funded by the
#   National Institutes of Health through the NIH Roadmap for Medical Research,
#   Grant U54 EB005149.
#
# * Kitware, Inc.
#
# Based on GTEST_ADD_TESTS by Daniel Blezek <blezek@gmail.com>

function(PANOPTICON_ADD_TESTS executable wd extra_args)
	if(NOT ARGN)
		message(FATAL_ERROR "Missing ARGN: Read the documentation for GTEST_ADD_TESTS")
	endif()

	foreach(source ${ARGN})
		file(READ "${source}" contents)
		string(REGEX MATCHALL "TEST_?F?\\(([A-Za-z_0-9 ,]+)\\)" found_tests ${contents})

		foreach(hit ${found_tests})
			string(REGEX REPLACE ".*\\( *([A-Za-z_0-9]+), *([A-Za-z_0-9]+) *\\).*" "\\1.\\2" test_name ${hit})
			add_test(NAME ${test_name} WORKING_DIRECTORY ${wd} COMMAND ${executable} --gtest_filter=${test_name} ${extra_args})
		endforeach()
	endforeach()
endfunction()

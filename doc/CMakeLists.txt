# From http://ericscottbarr.com/blog/2012/03/sphinx-and-cmake-beautiful-documentation-for-c-projects/
# configured documentation tools and intermediate build results
set(BINARY_BUILD_DIR "${CMAKE_CURRENT_BINARY_DIR}/_build")

# Sphinx cache with pickled ReST documents
set(SPHINX_CACHE_DIR "${CMAKE_CURRENT_BINARY_DIR}/_doctrees")

# HTML output directory
set(SPHINX_OUT_DIR "${CMAKE_CURRENT_BINARY_DIR}/")

#configure_file( "${CMAKE_CURRENT_SOURCE_DIR}/conf.py.in"
#	"${BINARY_BUILD_DIR}/conf.py" @ONLY)

add_custom_target(doc ALL "" DEPENDS doc_html doc_man)

add_custom_target(doc_html ALL ${SPHINX_EXECUTABLE} -q -b html
	-d "${SPHINX_CACHE_DIR}"
	"${CMAKE_CURRENT_SOURCE_DIR}" "${SPHINX_OUT_DIR}/html"
	COMMENT "Building HTML documentation")

add_custom_target(doc_man ALL ${SPHINX_EXECUTABLE} -q -b man
	-d "${SPHINX_CACHE_DIR}"
	"${CMAKE_CURRENT_SOURCE_DIR}" "${SPHINX_OUT_DIR}/man"
	COMMENT "Building manual page")

install(DIRECTORY "${SPHINX_OUT_DIR}/html" DESTINATION
	"/usr/share/doc/panopticon-${panopticon_VERSION}/")

install(FILES "${SPHINX_OUT_DIR}/man/qtpanopticon.1" DESTINATION
	"/usr/share/man/man1/")

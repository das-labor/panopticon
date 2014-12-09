# configured documentation tools and intermediate build results
set(BINARY_BUILD_DIR "${CMAKE_CURRENT_BINARY_DIR}/_build")

# Sphinx cache with pickled ReST documents
set(SPHINX_CACHE_DIR "${CMAKE_CURRENT_BINARY_DIR}/_doctrees")

# HTML output directory
set(SPHINX_HTML_DIR "${CMAKE_CURRENT_BINARY_DIR}/html")

#configure_file( "${CMAKE_CURRENT_SOURCE_DIR}/conf.py.in"
#	"${BINARY_BUILD_DIR}/conf.py" @ONLY)

add_custom_target(doc ALL ${SPHINX_EXECUTABLE} -q -b html
	-d "${SPHINX_CACHE_DIR}"
	"${CMAKE_CURRENT_SOURCE_DIR}" "${SPHINX_HTML_DIR}"
	COMMENT "Building HTML documentation with Sphinx")

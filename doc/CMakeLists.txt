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

set(panopticon_HTMLDOC feats.html arch.html disass.html index.html
	install.html intro.html others.html pil.html qtpanop.html refs.html
	regions.html usage.html _images/sugiyama.png searchindex.js genindex.html
	_static/down.png _static/up-pressed.png _static/basic.css _static/plus.png
	_static/file.png _static/comment-bright.png _static/comment.png
	_static/ajax-loader.gif _static/down-pressed.png _static/up.png
	_static/searchtools.js _static/comment-close.png _static/minus.png
	_static/pygments.css _static/websupport.js _static/autoload.js
	_static/ink-all.js _static/ink.css _static/site.css _static/favicon.ico)

foreach(file ${panopticon_HTMLDOC})
	get_filename_component(dirname "${file}" PATH)
	install(FILES "${SPHINX_OUT_DIR}/html/${file}" DESTINATION
		"share/doc/panopticon-${panopticon_VERSION}/html/${dirname}")
endforeach()

install(FILES "${SPHINX_OUT_DIR}/man/qtpanopticon.1" DESTINATION
	"share/man/man1/")

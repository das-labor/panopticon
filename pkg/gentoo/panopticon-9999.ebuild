# Copyright 1999-2013 Gentoo Foundation
# Distributed under the terms of the GNU General Public License v2
# $Header: $

EAPI=5

inherit git-2 cmake-utils pax-utils

DESCRIPTION="A disassembler with basic static analysis capabilities, SAT solver support and infer invariants for basic blocks."
HOMEPAGE="https://www.panopticon.re"
EGIT_REPO_URI="https://github.com/das-labor/panopticon.git"
EGIT_BRANCH="master"
SRC_URI=""
KEYWORDS="~amd64 ~x86"
LICENSE="GPL-3"
SLOT="0"
IUSE="clang debug doc +qt tests"

RDEPEND="dev-cpp/gtest
	dev-libs/boost
	app-arch/libarchive
	dev-db/kyotocabinet
	tests? ( dev-cpp/gtest )
	doc? ( app-doc/doxygen )
	qt? ( dev-qt/qtgui:5
		   dev-qt/qtwidgets:5
		   dev-qt/qtdeclarative:5
		   dev-qt/qtquickcontrols:5[widgets] )
	"
DEPEND="${RDEPEND}
		>=sys-devel/gcc-4.8.3
	clang? ( >=sys-devel/clang-3.4.1-r100
			sys-devel/llvm[clang] )"

RESTRICT="strip"

src_unpack() {
	git-2_src_unpack
  	cd "${S}"
}

src_configure() {
	if use clang; then
		CXX="clang++"
		CMAKE_CXX_COMPILER_ID="Clang"
	else
		CXX="g++"
		CMAKE_CXX_COMPILER_ID="GNU"
	fi

	if use debug; then
		CXXFLAGS="-O0 -g -ggdb"
	fi

	if use tests; then
		CMAKE_BUILD_TYPE="Debug"
	else
		CMAKE_BUILD_TYPE="Release"
	fi

	local mycmakeargs=(
	                $(cmake-utils_use_build qt)
	)

	cmake-utils_src_configure
}

src_compile() {
	cmake-utils_src_compile
}

src_install() {
	if use qt; then
		pax-mark -m "${ED}"usr/bin/"${PN}"
	fi

	cmake-utils_src_install
}

src_test() {
	if use tests; then
		cmake-utils_src_test
	fi
}

# Copyright 1999-2013 Gentoo Foundation
# Distributed under the terms of the GNU General Public License v2
# $Header: $

EAPI=5

inherit git-2 cmake-utils pax-utils

DESCRIPTION="A disassembler with basic static analysis capabilities and infer invariants for basic blocks."
HOMEPAGE="https://www.panopticon.re"
EGIT_REPO_URI="https://github.com/das-labor/panopticon.git"
EGIT_BRANCH="master"
SRC_URI=""
KEYWORDS="~amd64 ~x86"
LICENSE="GPL-3"
SLOT="0"
IUSE="debug doc +qt5 test"

RDEPEND="
	dev-libs/boost
	app-arch/libarchive
	dev-db/kyotocabinet
	qt5? ( dev-qt/qtgui:5
		   dev-qt/qtwidgets:5
		   dev-qt/qtdeclarative:5
		   dev-qt/qtquickcontrols:5[widgets]
		   dev-qt/qtconcurrent )
	"
DEPEND="${RDEPEND}
	test? ( dev-cpp/gtest )
	doc?    ( app-doc/doxygen )
	>=sys-devel/gcc-4.8.3"

RESTRICT="strip"

src_unpack() {
	git-2_src_unpack
	cd "${S}"
}

src_configure() {
	if use debug; then
		CXXFLAGS="-O0 -g -ggdb"
		CMAKE_BUILD_TYPE="Debug"
	else
		CMAKE_BUILD_TYPE="Release"
	fi

	local mycmakeargs=(
		$(cmake-utils_use_build qt5)
	)

	cmake-utils_src_configure
}

src_compile() {
	cmake-utils_src_compile
}

src_install() {
	if use qt5; then
		pax-mark -m "${ED}"usr/bin/"${PN}"
	fi

	cmake-utils_src_install
}

src_test() {
	if use test; then
		cmake-utils_src_test
	fi
}

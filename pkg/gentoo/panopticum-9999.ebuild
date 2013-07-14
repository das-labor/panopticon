# Copyright 1999-2013 Gentoo Foundation
# Distributed under the terms of the GNU General Public License v2
# $Header: $

EAPI=5
inherit git-2

DESCRIPTION="A disassembler with basic static analysis capabilities, SAT solver support and infer invariants for basic blocks."
HOMEPAGE="https://gitorious.org/panopticum/development"
EGIT_REPO_URI="git://gitorious.org/panopticum/development.git"
SRC_URI=""
KEYWORDS="~amd64 ~x86"
LICENSE="GPL-2"
SLOT="0"
IUSE="gcc doc llvm qt4"
REQUIRED_USE="^^ ( llvm gcc )"

RDEPEND="sci-mathematics/cvc4
	dev-util/cppunit
	media-libs/raptor
	dev-libs/redland
	dev-libs/boost
	media-gfx/graphviz
	doc? ( app-doc/doxygen[qt4] )
	qt4? ( dev-qt/qtgui:4 )
	"
DEPEND="${RDEPEND}
	gcc? ( >=sys-devel/gcc-4.8.0 )
	llvm? ( >=sys-devel/clang-3.0 )"

src_compile() {
	if use llvm ; then
		emake CC=clang CXX=clang++ cli || die "emake failed"
		if use qt4 ; then
			emake CC=clang CXX=clang++ qt || die "emake failed"
		fi
		if use doc ; then
			emake CC=clang CXX=clang++ doc || die "emake failed"
		fi
	else
		emake CC=gcc CXX=g++ cli || die "emake failed"
	fi
}

src_install() {
	emake DESTDIR="${D}" install || die "make install failed"
	dodoc TODO
}

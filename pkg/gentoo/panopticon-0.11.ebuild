# Copyright 1999-2013 Gentoo Foundation
# Distributed under the terms of the GNU General Public License v2
# $Header: $

EAPI=5

inherit git-2 pax-utils

DESCRIPTION="A libre cross platform disassembler"
HOMEPAGE="https://www.panopticon.re"
EGIT_REPO_URI="https://github.com/das-labor/panopticon.git"
EGIT_BRANCH="0.11.0"
SRC_URI=""
KEYWORDS="~amd64 ~x86"
LICENSE="GPL-3"
SLOT="0"
IUSE="debug test"

RDEPEND="
	sci-mathematics/glpk
	( dev-qt/qtgui:5
	  dev-qt/qtwidgets:5
	  dev-qt/qtdeclarative:5
	  dev-qt/qtquickcontrols:5[widgets])
	"
DEPEND="${RDEPEND}
	dev-lang/rust
	dev-util/cargo
	dev-util/cmake"

RESTRICT="strip"

src_unpack() {
	git-2_src_unpack
	cd "${S}"
}

src_configure() {
}

src_compile() {
	if debug then
		cargo build --debug --verbose
	else
		cargo build --release --verbose
	fi
}

src_install() {
	if use qt5; then
		pax-mark -m "${ED}"usr/bin/"${PN}"
	fi

	install -d -m 755 "$pkgdir/usr/bin"
    install -D -s -m 555 "$pkgname-$pkgver/target/release/qtpanopticon" "$pkgdir/usr/bin/qtpanopticon"
}

src_test() {
	if use test; then
		cargo test --verbose
	fi
}

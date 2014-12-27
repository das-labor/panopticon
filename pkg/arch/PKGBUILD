# Maintainer: Your Name <youremail@domain.com>
pkgname=panopticon
pkgver=0.9
pkgrel=1
pkgdesc="A libre cross platform disassembler"
arch=('x86_64' 'i686')
url="http://panopticon.re/"
license=('GPL3')
groups=('devel')
depends=(
    'kyotocabinet>=1.2.76'
    'libarchive>=3.1.2'
    'boost-libs>=1.53.0'
    'qt5-quickcontrols>=5.0')
makedepends=(
    'cmake>=2.8.9'
    'boost>=1.53.0'
    'git>=1'
    'python-sphinx>=1.2.2')
optdepends=(
    'python-sphinx: API documentation'
    'qt5-quickcontrols: Qt frontend')
provides=()
conflicts=()
replaces=()
backup=()
options=()
install=
changelog=
source=($pkgname-$pkgver::git+https://github.com/das-labor/panopticon.git#branch=release/0.9)
noextract=()
md5sums=('SKIP')

build() {
    mkdir -p "$pkgname-$pkgver/build"

    cd "$pkgname-$pkgver/build"
    cmake -DCMAKE_INSTALL_PREFIX=/usr ..
    make
}

package() {
    cd "$pkgname-$pkgver/build"

    make DESTDIR="$pkgdir/" install
}

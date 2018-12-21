[![Gitter](https://badges.gitter.im/das-labor/panopticon.svg)](https://gitter.im/das-labor/panopticon) [![Build Status](https://travis-ci.org/das-labor/panopticon.svg?branch=master)](https://travis-ci.org/das-labor/panopticon) [![Build status](https://ci.appveyor.com/api/projects/status/ht1wnf4qc0iocoar?svg=true)](https://ci.appveyor.com/project/flanfly/panopticon) [![Coverage Status](https://coveralls.io/repos/das-labor/panopticon/badge.svg?branch=master&service=github)](https://coveralls.io/github/das-labor/panopticon?branch=master)

![Panopticon](https://raw.githubusercontent.com/das-labor/panopticon/master/logo.png)

# Panopticon - A Libre Cross Platform Disassembler
Panopticon is a cross platform disassembler for reverse engineering written in
Rust. It can disassemble AMD64, x86, AVR and MOS 6502 instruction sets and open
ELF files. Panopticon comes with Qt GUI for browsing and annotating control
flow graphs,

## Install
If you simply want to use Panopticon follow the
[install instructions](https://panopticon.re/get) on the website.

## Building
Panopticon builds with Rust stable. The only dependencies aside from
a working Rust stable toolchain and Cargo you need is Qt 5.5 or higher.

**Ubuntu 15.10 and 16.04**
```bash
sudo apt install qt5-default qtdeclarative5-dev libqt5svg5-dev \
                 qml-module-qtquick-controls qml-module-qttest \
                 qml-module-qtquick2 qml-module-qtquick-layouts \
                 qml-module-qtgraphicaleffects qml-module-qtqml-models2 \
                 qml-module-qtquick-dialogs \
                 qtbase5-private-dev pkg-config \
                 git build-essential cmake \
                 qml-module-qt-labs-folderlistmodel \
                 qml-module-qt-labs-settings
```

**Fedora 22, 23 and 24**
```bash
sudo dnf install gcc-c++ cmake make qt5-qtdeclarative-devel qt5-qtquickcontrols \
                 qt5-qtgraphicaleffects qt5-qtsvg-devel \
                 adobe-source-sans-pro-fonts \
                 adobe-source-code-pro-fonts
```

**Gentoo**
```bash
layman -a rust

USE=widgets sudo -E emerge -av qtgraphicaleffects:5 qtsvg:5 qtquickcontrols:5 \
                               rust cargo cmake
```

After that clone the repository onto disk and use cargo to build
everything.

```bash
git clone https://github.com/das-labor/panopticon.git
cd panopticon
cargo build --all --release
```

**Windows**

Install the [Qt 5.4 SDK](http://download.qt.io/official_releases/online_installers/qt-unified-windows-x86-online.exe),
the [Rust toolchain](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe)
and [CMake](https://cmake.org/files/v3.6/cmake-3.6.1-win64-x64.msi).
Panopticon can be built using ``cargo build --all --release``.

**OS X**

Install [Homebrew](http://brew.sh/) and get Qt 5.5, CMake and the Rust toolchain.
Then, compile Panopticon using cargo.

```bash
brew install qt cmake rust
brew link qt --force
brew linkapps qt
export HOMEBREW_QT5_VERSION=$(brew list --versions qt | rev | cut -d ' ' -f1 | rev)
ln -s /usr/local/Cellar/qt/$HOMEBREW_QT5_VERSION/mkspecs /usr/local/mkspecs
ln -s /usr/local/Cellar/qt/$HOMEBREW_QT5_VERSION/plugins /usr/local/plugins
QTDIR64=/usr/local cargo build --all --release
```

## Running
After installation start the ``panopticon`` binary. If you build it from
source you can type:

```bash
cargo run --bin panopticon --release
```

For detailed usage information see the
[user documentaion](https://panopticon.re/usage).

## Contributing
Panopticon is licensed under GPLv3 and is Free Software. Hackers are always
welcome. Please check out [`CONTRIBUTING.md`](https://github.com/das-labor/panopticon/blob/master/CONTRIBUTING.md).

- [Issue Tracker](https://github.com/das-labor/panopticon/issues)
- [API Documentation](https://doc.panopticon.re/panopticon/index.html)

## Contact
- IRC: #panopticon on Freenode.
- Twitter: [```@panopticon_re```](https://twitter.com/@panopticon_re)

[![Build Status](https://travis-ci.org/das-labor/panopticon.svg?branch=master)](https://travis-ci.org/das-labor/panopticon) [![Build status](https://ci.appveyor.com/api/projects/status/ht1wnf4qc0iocoar?svg=true)](https://ci.appveyor.com/project/flanfly/panopticon) [![Coverage Status](https://coveralls.io/repos/das-labor/panopticon/badge.svg?branch=master&service=github)](https://coveralls.io/github/das-labor/panopticon?branch=master)

Intro
=====

Panopticon is a cross platform disassembler for reverse engineering
written in Rust. Panopticon has functions for disassembling, analysing
decompiling and patching binaries for various platforms and instruction
sets.

Panopticon comes with GUI for browsing control flow graphs, displaying
analysis results, controlling debugger instances and editing the on-disk
as well as in-memory representation of the program.

Building
========

Panopticon builds with Rust stable. The only dependencies aside from
a working Rust 1.10.0 toolchain and Cargo you need Qt 5.4 installed.

Linux
-----

Install Qt using your package manager.

Ubuntu 15.10 and 16.04:
```bash
sudo apt install qt5-default qtdeclarative5-dev \
                 qml-module-qtquick-controls qml-module-qttest \
                 qml-module-qtquick2 qml-module-qtquick-layouts \
                 qml-module-qtgraphicaleffects \
                 qtbase5-private-dev pkg-config \
                 git build-essential cmake
```

Fedora 22 and 23:
```bash
sudo dnf install qt5-qtdeclarative-devel qt5-qtquickcontrols \
                 qt5-qtgraphicaleffects
```

After that clone the repository onto disk and use cargo to build
everything.

```bash
git clone https://github.com/das-labor/panopticon.git
cd panopticon
cargo build --release
```

Gentoo:

```bash
layman -a rust
layman -f -o https://raw.github.com/das-labor/labor-overlay/master/labor-overlay -a labor-overlay

emerge -av panopticon
```

Windows
-------

Install the Qt 5.4 SDK and the Rust toolchain
Panopticon can be build using ``cargo build --release``.

Running
=======

```bash
cargo run --release
```

The current version only supports AVR and has no ELF or PE loader yet.
To test Panopticon you need relocated AVR code. Such a file is
prepared in ``tests/data/sosse``.

Contributing
============

Panopticon is licensed under GPLv3 and is Free Software. Hackers are
always welcome. See https://panopticon.re for our project documentation.
Panopticon uses Github for issue tracking: https://github.com/das-labor/panopticon/issues

Contact
=======

IRC: #panopticon on Freenode.
Twitter: ```@_cibo_```

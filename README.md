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
a working Rust toolchain and Cargo you need Qt 5.4 and GLPK installed.

Linux
-----

Install Qt using your package manager.

Ubuntu 13.10 and 14.04:
```bash
sudo apt-get install qt5-default qtdeclarative5-dev libqt5qml-quickcontrols \
                     qtdeclarative5-folderlistmodel-plugin qtdeclarative5-settings-plugin \
                     libglpk-dev
```

Fedora 20:
```bash
sudo yum install qt5-qtdeclarative-devel qt5-qtquickcontrols glpk-devel
```

After that clone the repository onto disk and use cargo to build
everything.

```bash
git clone https://github.com/das-labor/panopticon.git
cd panopticon
cargo build
```

Windows
-------

Install the Qt 5.4 SDK, GLPK for Windows and the Rust toolchain Panopticon can be build using ``cargo build``.

Running
=======

The current version only supports AVR and has no ELF or PE loader yet.
To test Panopticon you need relocated AVR code. Such a file is prepared in
``tests/data/sosse``.

Contributing
============

Panopticon is licensed under GPLv3 and is Free Software. Hackers are
always welcome. See https://panopticon.re for our project documentation.
Panopticon uses Github for issue tracking: https://github.com/das-labor/panopticon/issues

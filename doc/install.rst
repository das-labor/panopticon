Building
========

In order to compile Panopticon the following needs to be installed first:

- Qt 5.3
- CMake 2.8
- g++ 4.7 or Clang 3.4
- Boost 1.53
- Kyoto Cabinet 1.2.76
- libarchive 3.1.2

Linux
-----

First install the prerequisites using your package manager.

Ubuntu 13.10 and 14.04:

.. code-block:: bash

  sudo apt-get install g++ cmake git libboost-dev \
   libboost-filesystem-dev libboost-graph-dev \
   libkyotocabinet-dev libarchive-dev qt5-default \
   qtdeclarative5-dev libqt5qml-quickcontrols \
   qtdeclarative5-folderlistmodel-plugin \
   qtdeclarative5-settings-plugin

Fedora 20:

.. code-block:: bash

  sudo yum install gcc-c++ cmake git kyotocabinet-devel \
   libarchive-devel qt5-qtdeclarative-devel \
   qt5-qtquickcontrols boost-filesystem boost-graph \
   boost-static

After that clone the repository onto disk, create a build directory and
call cmake and the path to the source as argument. Compile the project
using GNU Make.

.. code-block:: bash

  git clone https://github.com/das-labor/panopticon.git
  mkdir panop-build
  cd panop-build
  cmake ../panopticon
  make -j4
  sudo make install

Windows
-------

After installing the prerequisites on Windows use the CMake GUI to
generate Visual Studio project files or Mingw Makefiles. Panopticon
can be compiled using VC++ 2013 or Mingw g++.



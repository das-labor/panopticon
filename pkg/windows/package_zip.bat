rem Variables
rem set QTDIR=C:\Qt\5.5\msvc2013_64
rem set RUSTDIR=C:\Program Files\Rust stable MSVC 1.10
rem set CMAKEDIR=C:\Program Files (x86)\CMake
rem set VSDIR=C:\Program Files (x86)\Microsoft Visual Studio 12.0
rem set P7ZDIR="C:\Program Files\7-Zip\7z.exe"
set VERSION="0.16"

rem Aux var setup
rem set PATH=%GLPKDIR%\w64;%RUSTDIR%\bin;%CMAKEDIR%\bin;%QTDIR%\bin;%PATH%
rem set CMAKE_PREFIX_PATH=%QTDIR%;%CMAKE_PREFIX_PATH%
rem set LIB=%GLPKDIR%\w64;%LIB%
rem call "%VSDIR%\VC\vcvarsall.bat" x64

rem Build
cargo build --all --release

rem Package
md out
copy ..\..\target\release\panopticon.exe out\panopticon.exe
xcopy /e /i /s /y ..\..\qml out\qml
TYPE ..\..\README.md | MORE /P > out\README.txt
TYPE ..\..\LICENSE | MORE /P > out\LICENSE.txt
TYPE ..\..\AUTHORS | MORE /P > out\AUTHORS.txt
TYPE ..\..\CHANGELOG | MORE /P > out\CHANGELOG.txt
%QTDIR%\bin\windeployqt.exe --release --qmldir out\qml out\panopticon.exe
7z a panopticon-%VERSION%.zip .\out\*

rmdir /s /q out

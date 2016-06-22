rem Variables
set QTDIR=C:\Qt\5.5\msvc2013_64
set GLPKDIR=C:\GLPK
set RUSTDIR=C:\Program Files\Rust stable MSVC 1.7
set CMAKEDIR=C:\Program Files (x86)\CMake
set VSDIR=C:\Program Files (x86)\Microsoft Visual Studio 12.0
set P7ZDIR="C:\Program Files\7-Zip\7z.exe"

rem Aux var setup
set PATH=%GLPKDIR%\w64;%RUSTDIR%\bin;%CMAKEDIR%\bin;%QTDIR%\bin;%PATH%
set CMAKE_PREFIX_PATH=%QTDIR%;%CMAKE_PREFIX_PATH%
set LIB=%GLPKDIR%\w64;%LIB%
call "%VSDIR%\VC\vcvarsall.bat" x64

rem Build
cargo build --release

rem Package
md out
copy ..\..\target\release\qtpanopticon.exe out\qtpanopticon.exe
copy %GLPKDIR%\w64\glpk_4_58.dll out\glpk_4_58.dll
xcopy /e /i /s /y ..\..\qml out\qml
%QTDIR%\bin\windeployqt.exe --release --qmldir out\qml out\qtpanopticon.exe
copy %QTDIR%\bin\Qt5QuickTest.dll out\Qt5QuickTest.dll
%P7ZDIR% a panopticon.zip .\out\*

rmdir /s /q out

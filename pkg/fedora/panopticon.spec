Name:          panopticon
Version:       master
Release:       1%{?dist}
Summary:       A libre cross-platform disassembler
URL:           http://panopticon.re
License:       GPLv3

Source0:       https://github.com/das-labor/panopticon/archive/%{version}.tar.gz

BuildRequires: gcc-c++
BuildRequires: cmake
BuildRequires: make
BuildRequires: qt5-qtdeclarative-devel
BuildRequires: qt5-qtquickcontrols
BuildRequires: qt5-qtgraphicaleffects
BuildRequires: qt5-qtsvg-devel
BuildRequires: rustc
BuildRequires: cargo

Requires:      adobe-source-sans-pro-fonts
Requires:      adobe-source-code-pro-fonts
Requires:      qt5-qtquickcontrols
Requires:      qt5-qtgraphicaleffects
Requires:      qt5-qtsvg

%description
Panopticon is a cross platform disassembler for reverse engineering written in
Rust. It can disassemble AMD64, x86, AVR and MOS 6502 instruction sets and open
ELF files. Panopticon comes with Qt GUI for browsing and annotating control
flow graphs.

%global debug_package %{nil}

%prep
%autosetup

%build
cargo build --all --release

%install
%{__install} -d -m755 %{buildroot}/usr/bin
%{__install} -D -s -m755 target/release/panopticon %{buildroot}/usr/bin/panopticon
%{__install} -d -m755 %{buildroot}/usr/share/panopticon/qml
cp -R qml/* %{buildroot}/usr/share/panopticon/qml
chown -R root:root %{buildroot}/usr/share/panopticon/qml
%{__install} -d -m755 %{buildroot}/usr/share/doc/panopticon
cp README.md %{buildroot}/usr/share/doc/panopticon/README.md

%files
%doc README.md
%{_bindir}/panopticon
%{_datarootdir}/panopticon/qml

%changelog
* Thu Oct 20 2016 seu <seu@panopticon.re> 0.16-1
- Remove dependency on GLPK

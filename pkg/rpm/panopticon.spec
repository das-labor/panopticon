Name: panopticon
Version: 0.10
Release: 1%{?dist}
License: GPLv3+
#Group: Applications/
BuildRequires: cmake >= 2.8.9, libarchive-devel >= 3.1.2, kyotocabinet-devel >= 1.2.76, qt5-qtdeclarative-devel,  boost-devel >= 1.53.0, python-sphinx
Requires: libarchive >= 3.1.2, kyotocabinet >= 1.2.76, qt5-qtdeclarative, qt5-qtquickcontrols, boost-filesystem >= 1.53.0, boost-graph >= 1.53.0, boost-system >= 1.53.0
Source: https://panopticon.re/files/panopticon_0.10.orig.tar.gz
URL: https://panopticon.re/
Summary: A libre cross-platform disassembler

%description
A libre cross-platform disassembler.

%changelog
* Thu Jan 22 2015 seu <seu@panopticon.re> - 0.10-1%{?dist}
- Initial package

%package devel
Group: Development/Libraries
Summary: Development files for panopticon
Requires: panopticon == 0.10


%description devel
Development files for panopticon.

%prep
%setup -q

%build
%cmake -DPC_INSTALL_DIR="/usr/share/pkgconfig" .
make

%install
rm -rf $RPM_BUILD_ROOT
make install DESTDIR=$RPM_BUILD_ROOT
ln -sf %{_mandir}/man1/qtpanopticon.1.gz %{buildroot}/%{_mandir}/man1/qtpanopticon-0.10.0.1.gz

%post -p /sbin/ldconfig

%postun -p /sbin/ldconfig

%files devel
%doc README.md
%{_includedir}/panopticon/tree.hh
%{_includedir}/panopticon/structure.hh
%{_includedir}/panopticon/amd64.hh
%{_includedir}/panopticon/region.hh
%{_includedir}/panopticon/hash.hh
%{_includedir}/panopticon/dflow.hh
%{_includedir}/panopticon/basic_block.hh
%{_includedir}/panopticon/database.hh
%{_includedir}/panopticon/architecture.hh
%{_includedir}/panopticon/decode.hh
%{_includedir}/panopticon/procedure.hh
%{_includedir}/panopticon/digraph.hh
%{_includedir}/panopticon/generic.hh
%{_includedir}/panopticon/mnemonic.hh
%{_includedir}/panopticon/loc.hh
%{_includedir}/panopticon/util.hh
%{_includedir}/panopticon/code_generator.hh
%{_includedir}/panopticon/disassembler.hh
%{_includedir}/panopticon/avr.hh
%{_includedir}/panopticon/program.hh
%{_includedir}/panopticon/ensure.hh
%{_includedir}/panopticon/interpreter.hh
%{_includedir}/panopticon/instr.hh
%{_includedir}/panopticon/value.hh
%{_includedir}/panopticon/marshal.hh
%{_libdir}/libpanopticon.so
%{_datadir}/pkgconfig/panopticon.pc

%files
%doc README.md
%{_mandir}/man1/qtpanopticon.1.gz
%{_mandir}/man1/qtpanopticon-0.10.0.1.gz
%{_libdir}/libpanopticon.so.0.10.0
%{_libdir}/libpanopticon.so.0
%{_bindir}/qtpanopticon-0.10.0
%{_bindir}/qtpanopticon
%doc %{_datadir}/doc/panopticon-0.10.0/html/feats.html
%doc %{_datadir}/doc/panopticon-0.10.0/html/arch.html
%doc %{_datadir}/doc/panopticon-0.10.0/html/disass.html
%doc %{_datadir}/doc/panopticon-0.10.0/html/index.html
%doc %{_datadir}/doc/panopticon-0.10.0/html/install.html
%doc %{_datadir}/doc/panopticon-0.10.0/html/intro.html
%doc %{_datadir}/doc/panopticon-0.10.0/html/others.html
%doc %{_datadir}/doc/panopticon-0.10.0/html/pil.html
%doc %{_datadir}/doc/panopticon-0.10.0/html/qtpanop.html
%doc %{_datadir}/doc/panopticon-0.10.0/html/refs.html
%doc %{_datadir}/doc/panopticon-0.10.0/html/regions.html
%doc %{_datadir}/doc/panopticon-0.10.0/html/usage.html
%doc %{_datadir}/doc/panopticon-0.10.0/html/_images/sugiyama.png
%doc %{_datadir}/doc/panopticon-0.10.0/html/searchindex.js
%doc %{_datadir}/doc/panopticon-0.10.0/html/genindex.html
%doc %{_datadir}/doc/panopticon-0.10.0/html/_static/down.png
%doc %{_datadir}/doc/panopticon-0.10.0/html/_static/up-pressed.png
%doc %{_datadir}/doc/panopticon-0.10.0/html/_static/basic.css
%doc %{_datadir}/doc/panopticon-0.10.0/html/_static/plus.png
%doc %{_datadir}/doc/panopticon-0.10.0/html/_static/file.png
%doc %{_datadir}/doc/panopticon-0.10.0/html/_static/comment-bright.png
%doc %{_datadir}/doc/panopticon-0.10.0/html/_static/comment.png
%doc %{_datadir}/doc/panopticon-0.10.0/html/_static/ajax-loader.gif
%doc %{_datadir}/doc/panopticon-0.10.0/html/_static/down-pressed.png
%doc %{_datadir}/doc/panopticon-0.10.0/html/_static/up.png
%doc %{_datadir}/doc/panopticon-0.10.0/html/_static/searchtools.js
%doc %{_datadir}/doc/panopticon-0.10.0/html/_static/comment-close.png
%doc %{_datadir}/doc/panopticon-0.10.0/html/_static/minus.png
%doc %{_datadir}/doc/panopticon-0.10.0/html/_static/pygments.css
%doc %{_datadir}/doc/panopticon-0.10.0/html/_static/websupport.js
%doc %{_datadir}/doc/panopticon-0.10.0/html/_static/autoload.js
%doc %{_datadir}/doc/panopticon-0.10.0/html/_static/favicon.ico
%doc %{_datadir}/doc/panopticon-0.10.0/html/_static/ink-all.js
%doc %{_datadir}/doc/panopticon-0.10.0/html/_static/ink.css
%doc %{_datadir}/doc/panopticon-0.10.0/html/_static/site.css

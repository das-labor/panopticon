#!/bin/bash
git clone https://github.com/flanfly/panopticon
cd panopticon
git checkout feature/debian-pkg
cd pkg/debian
dpkg-buildpackage
lintian ../*.deb
cp ../*.{dsc,deb} /out/

#!/bin/bash
sh /tmp/rustup.sh --disable-sudo --yes
git clone $PANOPTICON_URL
cd panopticon
git checkout $PANOPTICON_BRANCH
cd pkg/debian
dpkg-buildpackage
lintian ../*.deb
cp ../*.{dsc,deb} /out/

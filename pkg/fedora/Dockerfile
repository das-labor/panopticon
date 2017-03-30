FROM fedora:25

MAINTAINER sphinxc0re <sphinxc0re@panopticon.re>

RUN dnf install -y gcc-c++ cmake make \
                qt5-qtdeclarative-devel \
                qt5-qtquickcontrols \
                qt5-qtgraphicaleffects \
                qt5-qtsvg-devel \
                adobe-source-sans-pro-fonts \
                adobe-source-code-pro-fonts \
                fedora-packager \
                rustc \
                cargo

COPY panopticon.spec /panopticon.spec
COPY package.sh /package.sh

CMD /package.sh

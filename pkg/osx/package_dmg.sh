#!/bin/bash
cargo build --all --release
mkdir -p Panopticon.app/Contents/{MacOS,Resources}
cp Info.plist Panopticon.app/Contents
cp ../../target/release/panopticon Panopticon.app/Contents/MacOS/Panopticon
cp -R ../../qml Panopticon.app/Contents/Resources
$QTDIR64/bin/macdeployqt Panopticon.app -qmldir=Panopticon.app/Contents/Resources/qml/ -dmg

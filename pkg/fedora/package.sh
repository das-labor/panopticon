#!/bin/bash
spectool -g panopticon.spec
fedpkg --release f25 local
fedpkg --release f25 lint

cp /x86_64/panopticon*.rpm /out/panopticon_0.16_amd64.rpm

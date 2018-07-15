#!/bin/sh

GENET_VER=$(jq -r '.version | gsub("-";"~")' package.json)
mkdir -p out/.rpm
(cd out/.rpm && mkdir -p SOURCES BUILD RPMS SRPMS)
sed -e "s/{{GENET_VERSION}}/$GENET_VER/g" genet.rpm.spec > out/.rpm/genet.rpm.spec
cp -r debian/usr out/.rpm/BUILD
mkdir -p out/.rpm/BUILD/usr/share/icons/hicolor/256x256/apps
cp images/genet.png out/.rpm/BUILD/usr/share/icons/hicolor/256x256/apps
cp -r out/genet-linux-x64/. out/.rpm/BUILD/usr/share/genet
mv out/.rpm/BUILD/usr/share/genet/genet out/.rpm/BUILD/usr/share/genet/genet
chrpath -r /usr/share/genet out/.rpm/BUILD/usr/share/genet/genet
rpmbuild --define "_topdir $PWD/out/.rpm" -bb out/.rpm/genet.rpm.spec
mv out/.rpm/RPMS/*/*.rpm out/genet-linux-amd64.rpm

#!/bin/sh

set -e

rm -rf install
cargo install --locked --root install --path .

mkdir -p linux
cp install/bin/temple linux/temple
cp -r assets linux

cd linux
zip -r ../linux$VERSION.zip .

#!/bin/sh

set -e

rm -rf install
cargo install --locked --root install --target x86_64-pc-windows-gnu

mkdir -p windows
cp install/bin/temple.exe windows/temple.exe
cp -r temple/assets windows
cp /usr/lib/gcc/x86_64-w64-mingw32/8.3-posix/libstdc++-6.dll windows
cp /usr/lib/gcc/x86_64-w64-mingw32/8.3-posix/libgcc_s_seh-1.dll windows
cp /usr/x86_64-w64-mingw32/lib/libwinpthread-1.dll windows

cd windows
zip -r ../client/windows$VERSION.zip .

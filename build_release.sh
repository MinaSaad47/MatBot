#!/bin/sh

rm -rf build
mkdir build -p

# linux build
cargo build --release
mkdir build/MatBot-Linux
cp ./target/release/matbot build/MatBot-Linux
strip build/MatBot-Linux/matbot
cp ./settings-default.json build/MatBot-Linux/settings.json
pushd build/
7z a -t7z -m0=lzma2 -mx=9 -mfb=64 -md=192m -ms=on -mmt=12 MatBot-Linux.7z MatBot-Linux
popd


# windows build
cargo build --release --target x86_64-pc-windows-gnu
mkdir build/MatBot-Windows
cp ./target/x86_64-pc-windows-gnu/release/matbot MatBot-Windows
strip build/Windows-Linux/matbot
cp ./settings-default.json build/MatBot-Windows/settings.json
pushd build/
7z a -t7z -m0=lzma2 -mx=9 -mfb=64 -md=192m -ms=on -mmt=12 MatBot-Windows.7z MatBot-Windows
popd

#!/bin/bash
cargo build --release --target wasm32-unknown-unknown
cargo build --release --target x86_64-pc-windows-gnu
cargo build --release --target x86_64-apple-darwin

cp -r res ./target/wasm32-unknown-unknown/release/ &&
cp -r res ./target/x86_64-pc-windows-gnu/release/ &&
cp -r res ./target/x86_64-apple-darwin/release/

cd target || exit

cd wasm32-unknown-unknown/release/ || exit
zip -r ../../gravity-sim-web.zip gravity-sim.wasm res index.html
cd ../../

cd x86_64-pc-windows-gnu/release/ || exit
zip -r ../../gravity-sim-windows.zip gravity-sim.exe res
cd ../../

cd x86_64-apple-darwin/release/ || exit
zip -r ../../gravity-sim-mac.zip gravity-sim res
cd ../../
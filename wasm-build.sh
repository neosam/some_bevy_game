#!/usr/bin/env bash

cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --out-name bevy_game --out-dir wasm --target web target/wasm32-unknown-unknown/release/some_bevy_game.wasm
cp -r assets wasm/

pushd wasm
python3 -m http.server
pushd

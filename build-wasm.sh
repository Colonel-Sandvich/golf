#!/bin/bash

# Exit the script if any command fails
set -e

cargo build --profile wasm-release --target wasm32-unknown-unknown

wasm-bindgen --no-typescript --out-name golf --out-dir ./out --target web target/wasm32-unknown-unknown/wasm-release/golf.wasm

wasm-opt -Oz --output out/golf_bg.wasm out/golf_bg.wasm

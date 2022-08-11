@echo off
cargo build --release
wasm-pack build --target nodejs
pause
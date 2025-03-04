.PHONY: bserve build run serve

bserve: build serve

build:
	cargo build --release --target wasm32-unknown-unknown
	wasm-bindgen target/wasm32-unknown-unknown/release/rust-roguelike.wasm --out-dir wasm --no-modules --no-typescript

run:
	cargo run

serve:
	python -m http.server -d ./wasm

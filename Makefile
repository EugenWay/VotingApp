.PHONY: exp all prepare

exp:
	@ export CARGO_HOME="/home/gitpod/.cargo"

all: exp
	RUSTFLAGS="-C link-args=--import-memory" cargo +nightly build --release --target=wasm32-unknown-unknown
	wasm-proc --path ./target/wasm32-unknown-unknown/release/voting_app.wasm
	ls -la ./target/wasm32-unknown-unknown/release/voting_app*.wasm

prepare: exp
 	ustup toolchain add nightly
	rustup target add wasm32-unknown-unknown --toolchain nightly
	cargo install --git https://github.com/gear-tech/gear wasm-proc
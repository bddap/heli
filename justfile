web-build-release:
    cargo build --release --target wasm32-unknown-unknown
    cp target/wasm32-unknown-unknown/release/heli.wasm web/game.wasm

web-build-debug:
    cargo build --target wasm32-unknown-unknown
    cp target/wasm32-unknown-unknown/debug/heli.wasm web/game.wasm

serve:
    basic-http-server web

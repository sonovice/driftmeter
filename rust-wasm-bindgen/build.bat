cargo +nightly build --release --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/release/drift_meter.wasm --no-modules --browser --out-dir ./dist
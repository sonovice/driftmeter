Install ```cargo +nightly install wasm-bindgen-cli```

To build the example, use
```
cargo +nightly build --release --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/drift_meter.wasm --no-modules --browser --out-dir ./dist
```

To serve, make sure you have Python 3 and ```pip install ComplexHTTPServer```.

Serve dist folder using ```python -m ComplexHTTPServer```.
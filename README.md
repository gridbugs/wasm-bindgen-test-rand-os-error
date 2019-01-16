[![Travis-CI Status](https://travis-ci.org/stevebob/wasm-bindgen-test-rand-os-error.svg?branch=master)](https://travis-ci.org/stevebob/wasm-bindgen-test-rand-os-error)

Building this crate produces a wasm file which can be successfully `require`d
by a nodejs file. Running `wasm-pack test --node` on this crate produces the
following output:

```
  [1/5] Checking `rustc` version...
  [2/5] Adding WASM target...
  info: component 'rust-std' for target 'wasm32-unknown-unknown' is up to date
  [3/5] Compiling tests to WASM...
     Compiling foo v0.1.0 (/home/steve/src/wasm-bindgen-test-rand-os-error)
      Finished dev [unoptimized + debuginfo] target(s) in 0.24s
  wasm-bindgen 0.2.31
  [5/5] Running tests in node...
      Finished dev [unoptimized + debuginfo] target(s) in 0.03s
       Running target/wasm32-unknown-unknown/debug/deps/foo-5b4a5ece76605715.wasm
  LinkError: WebAssembly Instantiation: Import #25 module="./wasm-bindgen-test" function="__wbg_new_bc890df3e5e6a91d" error: function im
      at Object.<anonymous> (/home/steve/src/wasm-bindgen-test-rand-os-error/target/wasm32-unknown-unknown/wbg-tmp/wasm-bindgen-test_bg.
      at Module._compile (internal/modules/cjs/loader.js:721:30)
      at Object.Module._extensions..js (internal/modules/cjs/loader.js:732:10)
      at Module.load (internal/modules/cjs/loader.js:620:32)
      at tryModuleLoad (internal/modules/cjs/loader.js:560:12)
      at Function.Module._load (internal/modules/cjs/loader.js:552:3)
      at Module.require (internal/modules/cjs/loader.js:657:17)
      at require (internal/modules/cjs/helpers.js:22:18)
      at Object.<anonymous> (/home/steve/src/wasm-bindgen-test-rand-os-error/target/wasm32-unknown-unknown/wbg-tmp/wasm-bindgen-test.js:
      at Module._compile (internal/modules/cjs/loader.js:721:30)
  error: test failed, to rerun pass '--lib'
| Executing bindgen...
Error: Running Wasm tests with wasm-bindgen-test failed
Caused by: failed to execute `cargo test`: exited with exit code: 1
```

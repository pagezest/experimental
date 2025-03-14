To compile the rust source code

```
cargo run -- <tinywasm|wasmi>
```

For generating the wasm binary from assembly script
Edit the index file `wasm-generator/assembly/index.ts` and build the binary

```
cd wasm-generator
npm i
npm run build
```

You can find the debug.wasm file generated inside `wasm-generator/build/debug.wasm`.

To run benchmarks on `tinyWASM`

```
cargo run -- tinywasm
```

To run benchmarks on `wasmi`

```
cargo run -- wasmi
```

To compile the rust source code

```
cargo run
```

For generating the wasm binary from assembly script
Edit the index file `wasm-generator/assembly/index.ts` and build the binary

```
cd wasm-generator
npm i
npm run build
```

You can find the debug.wasm file generated inside `wasm-generator/build/debug.wasm`.

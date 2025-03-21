# Generating Wasm binary

For generating the wasm binary from assembly script
Edit the index file `wasm-generator/assembly/index.ts` and build the binary

```
cd wasm-generator
npm i
npm run build
```

You can find the debug.wasm file generated inside `wasm-generator/build/debug.wasm`.

# Quick Steps to run benchmarking

To compile the rust source code and run benchmarks based on
- Wasm VM to use.
- Number of times you'd want to initialize a wasm vm.

Run any of the following commands.
```
cargo run -- tinywasm 10
cargo run -- wasmi 10
cargo run -- wamr 10
```

# Wamr Setup

Please install 
## 1. CMAKE > 3.1

MacOS
```shell
brew install cmake
```

Linux
```shell
sudo pacman -S cmake
```

## 2. LLVM

MacOS
```shell
brew install llvm
export LLVM_DIR=$(brew --prefix llvm)/lib/cmake/llvm
```

Linux
```shell
sudo pacman -S llvm
export LLVM_DIR=/usr/bin/llvm
```

Keep your rust version > 1.82

```shell
rustup override set 1.82.0
```


### Run the Benchmark

Use the following command
```rust
cargo run -- wamr 10
```

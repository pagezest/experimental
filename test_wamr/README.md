For running WARM benchmarking

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
cargo run
```

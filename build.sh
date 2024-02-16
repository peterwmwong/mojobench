#!/bin/sh

set -e

system_profiler SPHardwareDataType -detailLevel mini
mojo --version
rustc --version -v

disasm () {
    xcrun llvm-objdump --disassemble "$1"> "disasm/$(basename $1).txt"
}

# Build
mojo build src/recursive-mojo.mojo -o ./recursive-mojo
cargo build --release

# Disassemble
mkdir -p disasm
disasm ./recursive-mojo
disasm target/release/recursive-rust

# Benchmark
hyperfine                                                                                 \
    -N          `# Suggested option when command takes less than 5 ms`                    \
    --warmup=16 `# Suggested option when first run is significantly slower than the rest` \
    ./recursive-mojo                                                                      \
    target/release/recursive-rust

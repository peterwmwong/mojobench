Comparing Rust and Mojo compiler's ability to optimize away store/loads.

Originally benchmarking the TCO/TCE performance claims in [Mojo vs. Rust: is Mojo 🔥 faster than Rust 🦀 ?](https://www.modular.com/blog/mojo-vs-rust-is-mojo-faster-than-rust) ([archive link](https://web.archive.org/web/20240215183318/https://www.modular.com/blog/mojo-vs-rust-is-mojo-faster-than-rust)) (see [`main` branch](https://github.com/peterwmwong/mojobench/tree/main)).

> Disclaimer: I'm not planning on maintaining/updating this repo. I'm personally not invested in either Rust or Mojo! This is just quick, slapped together comparison that works on my Apple M3.

## Setup

- [Mojo](https://developer.modular.com/download)
- [Rust Nightly](https://www.rust-lang.org/tools/install)
- [hyperfine](https://github.com/sharkdp/hyperfine?tab=readme-ov-file#with-cargo-linux-macos-windows)
- XCode (for generating disassembly)

## Benchmark

- [Mojo](./src/store-load-mojo.mojo)
- [Rust](./src/bin/store-load-rust.rs)

## Usage (terminal)

> ./build.sh

## Example output on my machine

```
Hardware:

    Hardware Overview:

      Model Name: MacBook Pro
      Model Identifier: Mac15,8
      Model Number: Z1AW001BXLL/A
      Chip: Apple M3 Max
      Total Number of Cores: 16 (12 performance and 4 efficiency)
      Memory: 64 GB
      System Firmware Version: 10151.100.756
      OS Loader Version: 10151.100.756

mojo 0.7.0 (af002202)
rustc 1.78.0-nightly (ee9c7c940 2024-02-14)
binary: rustc
commit-hash: ee9c7c940c07d8b67c9a6b2ec930db70dcd23a46
commit-date: 2024-02-14
host: aarch64-apple-darwin
release: 1.78.0-nightly
LLVM version: 18.1.0
    Finished `release` profile [optimized] target(s) in 0.00s
Benchmark 1: ./store-load-mojo
  Time (mean ± σ):     278.7 ms ±   2.7 ms    [User: 276.5 ms, System: 0.8 ms]
  Range (min … max):   272.2 ms … 282.1 ms    10 runs
 
  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet system without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.
 
Benchmark 2: target/release/store-load-rust
  Time (mean ± σ):       3.1 ms ±   0.1 ms    [User: 2.7 ms, System: 0.3 ms]
  Range (min … max):     3.0 ms …   4.1 ms    948 runs
 
  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet system without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.
 
Summary
  target/release/store-load-rust ran
   89.33 ± 2.91 times faster than ./store-load-mojo
```

## Example output disassembly for my machine

- [Mojo](./disasm/store-load-mojo.txt)
- [Rust](./disasm/store-load-rust.txt)

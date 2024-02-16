A quick look at the Mojo Tail Call Optimization and Elimination performance claims in [Mojo vs. Rust: is Mojo 🔥 faster than Rust 🦀 ?](https://www.modular.com/blog/mojo-vs-rust-is-mojo-faster-than-rust) ([archive link](https://web.archive.org/web/20240215183318/https://www.modular.com/blog/mojo-vs-rust-is-mojo-faster-than-rust))

> Note: Modular seems to have revised the post's benchmark, [originally](https://web.archive.org/web/20240212205752/https://www.modular.com/blog/mojo-vs-rust-is-mojo-faster-than-rust) a `factorial()` function.

> Disclaimer: I'm not planning on maintaining/updating this repo. I'm personally not invested in either Rust or Mojo! This is just quick, slapped together comparison that works on my Apple M3 with [Mojo](https://developer.modular.com/download), [Rust Nightly](https://www.rust-lang.org/tools/install), [hyperfine](https://github.com/sharkdp/hyperfine?tab=readme-ov-file#with-cargo-linux-macos-windows) and XCode (for disassembly) installed.

## Benchmark

- [Mojo](./src/recursive-mojo.mojo)
- [Rust](./src/bin/recursive-rust.rs)
- [Rust using Vec::with_capacity()](./src/bin/recursive-rust-vec_with_capacity.rs)
- [Rust min sized binary](./src/bin/recursive-rust-no_main.rs)
  - Just threw this in as some of my projects use `#![no_main]` to reduce the binary size and startup time
  - Reading the disassembly is waayyyy easier, I wish Mojo had something similar.
  - More info: [johnthagen/min-sized-rust](https://github.com/johnthagen/min-sized-rust?tab=readme-ov-file#remove-corefmt-with-no_main-and-careful-usage-of-libstd)

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
    Finished `release` profile [optimized] target(s) in 0.02s
Benchmark 1: ./recursive-mojo
  Time (mean ± σ):     780.8 µs ±  45.2 µs    [User: 334.2 µs, System: 272.6 µs]
  Range (min … max):   711.4 µs … 1086.6 µs    3818 runs
 
  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet system without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.
 
Benchmark 2: target/release/recursive-rust
  Time (mean ± σ):     752.3 µs ±  52.6 µs    [User: 303.0 µs, System: 272.8 µs]
  Range (min … max):   681.2 µs … 1588.8 µs    2978 runs
 
  Warning: The first benchmarking run for this command was significantly slower than the rest (1.0 ms). This could be caused by (filesystem) caches that were not filled until after the first run. You are already using the '--warmup' option which helps to fill these caches before the actual benchmark. You can either try to increase the warmup count further or re-run this benchmark on a quiet system in case it was a random outlier. Alternatively, consider using the '--prepare' option to clear the caches before each timing run.
 
Summary
  target/release/recursive-rust ran
    1.04 ± 0.09 times faster than ./recursive-mojo
```

## Example disassembly for my machine

- [Mojo](./disasm/recursive-mojo.txt)
- [Rust](./disasm/recursive-rust.txt)
- [Rust using Vec::with_capacity()](./disasm/recursive-rust-vec_with_capacity.txt)
- [Rust min sized binary](./disasm/recursive-rust-no_main.txt)

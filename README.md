A quick look at the Mojo Tail Call Optimization and Elimination performance claims in [Mojo vs. Rust: is Mojo 🔥 faster than Rust 🦀 ?](https://www.modular.com/blog/mojo-vs-rust-is-mojo-faster-than-rust) ([archive link](https://web.archive.org/web/20240215183318/https://www.modular.com/blog/mojo-vs-rust-is-mojo-faster-than-rust))

> Note: Modular seems to have revised the post's benchmark, [originally](https://web.archive.org/web/20240212205752/https://www.modular.com/blog/mojo-vs-rust-is-mojo-faster-than-rust) a `factorial()` function.

> Disclaimer: I'm not planning on maintaining/updating this repo. I'm personally not invested in either Rust or Mojo! This is just quick, slapped together comparison that works on my Apple M3 with [Mojo](https://developer.modular.com/download), [Rust Nightly](https://www.rust-lang.org/tools/install) and XCode (for disassembly) installed.

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
   Compiling mojobench v0.1.0 (/Users/pwong/projects/mojobench)
    Finished `release` profile [optimized] target(s) in 0.09s
Benchmark 1: ./recursive-mojo
  Time (mean ± σ):     811.7 µs ±  83.5 µs    [User: 350.2 µs, System: 281.1 µs]
  Range (min … max):   690.0 µs … 1468.6 µs    3179 runs
 
  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet system without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.
 
Benchmark 2: target/release/recursive-rust
  Time (mean ± σ):     755.7 µs ±  62.9 µs    [User: 311.5 µs, System: 269.3 µs]
  Range (min … max):   658.4 µs … 1474.8 µs    2836 runs
 
  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet system without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.
 
Benchmark 3: target/release/recursive-rust-vec_with_capacity
  Time (mean ± σ):     256.2 ms ±   1.2 ms    [User: 253.6 ms, System: 0.9 ms]
  Range (min … max):   254.9 ms … 258.4 ms    11 runs
 
Benchmark 4: target/release/recursive-rust-no_main
  Time (mean ± σ):     577.8 µs ±  72.6 µs    [User: 154.3 µs, System: 251.9 µs]
  Range (min … max):   484.8 µs … 1265.9 µs    5460 runs
 
  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet system without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.
 
Summary
  target/release/recursive-rust-no_main ran
    1.31 ± 0.20 times faster than target/release/recursive-rust
    1.40 ± 0.23 times faster than ./recursive-mojo
  443.47 ± 55.75 times faster than target/release/recursive-rust-vec_with_capacity
```

## Example disassembly for my machine

- [Mojo](./disasm/recursive-mojo.txt)
- [Rust](./disasm/recursive-rust.txt)
- [Rust using Vec::with_capacity()](./disasm/recursive-rust-vec_with_capacity.txt)
- [Rust min sized binary](./disasm/recursive-rust-no_main.txt)

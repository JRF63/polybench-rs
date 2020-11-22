## polybench-rs

Rust port of [PolyBench/C](http://polybench.sourceforge.net) - a collection of numerical computations from various domains.

This reimplementation was primarily motivated by benchmarking [LLVM Polly](https://polly.llvm.org/) as applied to Rust.

## Usage

Individual benchmarks can be run with:
   ```sh
   cargo run --bin <benchmark> --release
   ```

## Configuration

The benchmarks are implemented as generic functions that accept the problem size as const generics. The data type to the benchmarks can be modified in [src/config.rs](https://github.com/JRF63/polybench-rs/blob/master/src/config.rs).

LLVM flags can be set through the `RUSTFLAGS` environment variable:
   ```sh
   set RUSTFLAGS=-Cllvm-args=--polly -Cllvm-args=--polly-vectorizer=stripmine
   ```

## Results

![](.github/images/all.png?raw=true)

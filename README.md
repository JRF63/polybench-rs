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

The Polly optimization shows improvement on the `correlation`, `covariance`, `gemm`, `gemver`, `trmm`, `2mm`, `3mm`, `gramschmidt` and `mvt`. `trisolv` started off being improved but had worse performance on increased input sizes.

Performance on `gesummv`, `atax`, `bicg`, `durbin`, `deriche`, `fdtd_2d` and `jacobi_2d` decreased. `gesummv`, `atax`, `bicg` and `jacobi_2d` are particularly bad, being about twice as slow with or without vectorization. `ludcmp` and `jacobi_1d` also have worse performance but only on vectorized builds. `floyd_warshall` is initially slowed but recovers on increased input sizes.

Benchmarks that show no significant (less than 5%) change includes `symm`, `syr2k`, `syrk`, `doitgen`, `cholesky`, `lu`, `nussinov`, `adi`, `heat_3d` and `seidel_2d`.

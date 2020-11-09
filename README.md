## polybench-rs

Rust port of [PolyBench/C](http://polybench.sourceforge.net) - a collection of numerical computations from various domains.

This reimplementation was primarily motivated by benchmarking [LLVM Polly](https://polly.llvm.org/) as applied to Rust.

## Usage

Individual benchmarks can be run by:
   ```sh
   cargo run --bin <benchmark> --release
   ```

Or run all the benchmarks sequentially: 
   ```sh
   cargo run --bin all --release
   ```

## Configuration

The input sizes to the benchmarks can be modified in [src/config.rs](https://github.com/JRF63/polybench-rs/blob/master/src/config.rs).

LLVM flags can be set through the `RUSTFLAGS` environment variable:
   ```sh
   set RUSTFLAGS=-Cllvm-args=--polly -Cllvm-args=--polly-vectorizer=stripmine
   ```

## Results

The results here are the minimum elapsed time of each benchmark tested on a 3.60 GHz Ryzen 1600X. Polly was applied on top of commit 8df58ae03a8 of the Rust compiler. The speedups/slowdowns are observed to be depedent on the input size - the slowdowns are reduced with increasing problem size while the speedups are decreased with decreasing problem size. The values here are from the default config.

| Benchmark      | No polly (s) | polly (s)  | speedup  | polly +<br />stripmine<br />vectorizer (s) | speedup  |
| :------------- | -----------: | ---------: | -------: | -----------------------------------------: | -------: |
| correlation    |    5.8437839 |  0.5987963 |  875.92% |                                  0.3184663 | 1734.98% |
| covariance     |    5.8045154 |  0.5722860 |  914.27% |                                  0.2969109 | 1854.97% |
| gemm           |    1.6411026 |  0.7337929 |  123.65% |                                  0.3542405 |  363.27% |
| gemver         |    0.0211482 |  0.0123314 |   71.50% |                                  0.0121561 |   73.97% |
| gesummv        |    0.0014259 |  0.0028342 |  -49.69% |                                  0.0022809 |  -37.49% |
| symm           |    2.0924019 |  2.1540793 |   -2.86% |                                  1.9206123 |    8.94% |
| syr2k          |    2.2735184 |  2.3731959 |   -4.20% |                                  2.2201281 |    2.40% |
| syrk           |    0.5595778 |  0.5646177 |   -0.89% |                                  0.5604074 |   -0.15% |
| trmm           |    1.8121414 |  0.5821095 |  211.31% |                                  0.5834571 |  210.59% |
| 2mm            |    1.3551517 |  0.8449635 |   60.38% |                                  0.4122511 |  228.72% |
| 3mm            |    2.5542321 |  1.2912484 |   97.81% |                                  0.5514134 |  363.22% |
| atax           |    0.0039606 |  0.0078308 |  -49.42% |                                  0.0057150 |  -30.70% |
| bicg           |    0.0031234 |  0.0052898 |  -40.95% |                                  0.0053397 |  -41.51% |
| doitgen        |    0.3455372 |  0.3647281 |   -5.26% |                                  0.3644316 |   -5.18% |
| mvt            |    0.0192662 |  0.0074722 |  157.84% |                                  0.0057340 |  236.00% |
| cholesky       |    1.2176717 |  1.2410114 |   -1.88% |                                  1.2320304 |   -1.17% |
| durbin         |    0.0022422 |  0.0024166 |   -7.22% |                                  0.0023948 |   -6.37% |
| gramschmidt    |    9.4215762 |  1.2369117 |  661.70% |                                  1.0364080 |  809.06% |
| lu             |    3.7693336 |  3.8977189 |   -3.29% |                                  3.6831233 |    2.34% |
| ludcmp         |    3.6132317 |  3.7723242 |   -4.22% |                                  3.6403986 |   -0.75% |
| trisolv        |    0.0018153 |  0.0018312 |   -0.87% |                                  0.0018164 |   -0.06% |
| deriche        |    0.2688601 |  0.3066753 |  -12.33% |                                  0.2991336 |  -10.12% |
| floyd_warshall |   15.3093169 | 15.5693250 |   -1.67% |                                 15.3744842 |   -0.42% |
| nussinov       |    5.2919465 |  5.4717441 |   -3.29% |                                  5.2572920 |    0.66% |
| adi            |    9.8254784 |  9.2741692 |    5.94% |                                  9.1387150 |    7.51% |
| fdtd_2d        |    1.9114797 |  2.3060303 |  -17.11% |                                  2.3435431 |  -18.44% |
| heat_3d        |    2.8167419 |  2.8460667 |   -1.03% |                                  2.7965778 |    0.72% |
| jacobi_1d      |    0.0005189 |  0.0005258 |   -1.31% |                                  0.0010164 |  -48.95% |
| jacobi_2d      |    1.6958271 |  3.6769243 |  -53.88% |                                  3.2459165 |  -47.76% |
| seidel_2d      |   16.3429896 | 16.3723473 |   -0.18% |                                 15.9205988 |    2.65% |

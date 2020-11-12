#![feature(min_const_generics)]

use polybench_rs::linear_algebra::blas::trmm::bench;

fn bench_and_print<const M: usize, const N: usize>() {
    let dims = format!("{:?}", (M, N));
    let elapsed = bench::<M, N>().as_secs_f64();
    println!("{:<14} | {:<30} | {:.7} s", "trmm", dims, elapsed);
}

fn main() {
    bench_and_print::<250, 300>();
    bench_and_print::<500, 600>();
    bench_and_print::<1000, 1200>();
}

#![feature(min_const_generics)]

use polybench_rs::linear_algebra::blas::gemm::bench;

fn bench_and_print<const NI: usize, const NJ: usize, const NK: usize>() {
    let dims = format!("{:?}", (NI, NJ, NK));
    let elapsed = bench::<NI, NJ, NK>().as_secs_f64();
    println!("{:<14} | {:<30} | {:.7} s", "gemm", dims, elapsed);
}

fn main() {
    bench_and_print::<250, 275, 300>();
    bench_and_print::<500, 550, 600>();
    bench_and_print::<1000, 1100, 1200>();
}

#![feature(min_const_generics)]

use polybench_rs::linear_algebra::kernels::atax::bench;

fn bench_and_print<const M: usize, const N: usize>() {
    let dims = format!("{:?}", (M, N));
    let elapsed = bench::<M, N>().as_secs_f64();
    println!("{:<14} | {:<30} | {:.7} s", "atax", dims, elapsed);
}

fn main() {
    bench_and_print::<475, 525>();
    bench_and_print::<950, 1050>();
    bench_and_print::<1900, 2100>();
}

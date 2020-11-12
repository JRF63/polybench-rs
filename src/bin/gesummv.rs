#![feature(min_const_generics)]

use polybench_rs::linear_algebra::blas::gesummv::bench;

fn bench_and_print<const N: usize>() {
    let dims = format!("{:?}", (N));
    let elapsed = bench::<N>().as_secs_f64();
    println!("{:<14} | {:<30} | {:.7} s", "gesummv", dims, elapsed);
}

fn main() {
    bench_and_print::<5000>();
    bench_and_print::<10000>();
    bench_and_print::<20000>();
}

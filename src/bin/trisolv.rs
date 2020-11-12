#![feature(min_const_generics)]

use polybench_rs::linear_algebra::solvers::trisolv::bench;

fn bench_and_print<const N: usize>() {
    let dims = format!("{:?}", (N));
    let elapsed = bench::<N>().as_secs_f64();
    println!("{:<14} | {:<30} | {:.7} s", "trisolv", dims, elapsed);
}

fn main() {
    bench_and_print::<500>();
    bench_and_print::<1000>();
    bench_and_print::<2000>();
}

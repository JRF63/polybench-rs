#![feature(min_const_generics)]

use polybench_rs::linear_algebra::kernels::_3mm::bench;

fn bench_and_print<
    const A: usize,
    const B: usize,
    const C: usize,
    const D: usize,
    const E: usize,
>() {
    let dims = format!("{:?}", (A, B, C, D, E));
    let elapsed = bench::<A, B, C, D, E>().as_secs_f64();
    println!("{:<14} | {:<30} | {:.7} s", "3mm", dims, elapsed);
}

fn main() {
    bench_and_print::<200, 225, 250, 275, 300>();
    bench_and_print::<400, 450, 500, 550, 600>();
    bench_and_print::<800, 900, 1000, 1100, 1200>();
}

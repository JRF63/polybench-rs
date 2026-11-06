#![feature(min_const_generics)]

use polybench_rs::stencils::jacobi_2d::bench;

fn bench_and_print<const TSTEPS: usize, const N: usize>() {
    let dims = format!("{:?}", (TSTEPS, N));
    let elapsed = bench::<N, TSTEPS>().as_secs_f64();
    println!("{:<14} | {:<30} | {:.7} s", "jacobi_2d", dims, elapsed);
}

fn main() {
    bench_and_print::<125, 325>();
    bench_and_print::<250, 650>();
    bench_and_print::<500, 1300>();
}

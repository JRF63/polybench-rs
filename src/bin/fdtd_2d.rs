#![feature(min_const_generics)]

use polybench_rs::stencils::fdtd_2d::bench;

fn bench_and_print<const TMAX: usize, const NX: usize, const NY: usize>() {
    let dims = format!("{:?}", (TMAX, NX, NY));
    let elapsed = bench::<NX, NY, TMAX>().as_secs_f64();
    println!("{:<14} | {:<30} | {:.7} s", "fdtd_2d", dims, elapsed);
}

fn main() {
    bench_and_print::<125, 250, 300>();
    bench_and_print::<250, 500, 600>();
    bench_and_print::<500, 1000, 1200>();
}

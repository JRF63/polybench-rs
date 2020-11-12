#![feature(min_const_generics)]

use polybench_rs::stencils::fdtd_2d::bench;

fn bench_and_print<const NX: usize, const NY: usize, const TMAX: usize>() {
    let dims = format!("{:?}", (NX, NY, TMAX));
    let elapsed = bench::<NX, NY, TMAX>().as_secs_f64();
    println!("{:<14} | {:<30} | {:.7} s", "fdtd_2d", dims, elapsed);
}

fn main() {
    bench_and_print::<250, 300, 125>();
    bench_and_print::<500, 600, 250>();
    bench_and_print::<1000, 1200, 500>();
}

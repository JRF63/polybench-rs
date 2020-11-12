#![feature(min_const_generics)]

use polybench_rs::stencils::heat_3d::bench;

fn bench_and_print<const N: usize, const TSTEPS: usize>() {
    let dims = format!("{:?}", (N, TSTEPS));
    let elapsed = bench::<N, TSTEPS>().as_secs_f64();
    println!("{:<14} | {:<30} | {:.7} s", "heat_3d", dims, elapsed);
}

fn main() {
    bench_and_print::<30, 125>();
    bench_and_print::<60, 250>();
    bench_and_print::<120, 500>();
}

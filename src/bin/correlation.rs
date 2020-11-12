#![feature(min_const_generics)]

use polybench_rs::datamining::correlation::bench;

fn bench_and_print<const M: usize, const N: usize>() {
    let dims = format!("{:?}", (M, N));
    let elapsed = bench::<M, N>().as_secs_f64();
    println!("{:<14} | {:<30} | {:.7} s", "correlation", dims, elapsed);
}

fn main() {
    bench_and_print::<300, 350>();
    bench_and_print::<600, 700>();
    bench_and_print::<1200, 1400>();
}

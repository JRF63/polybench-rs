#![feature(min_const_generics)]

use polybench_rs::linear_algebra::kernels::doitgen::bench;

fn bench_and_print<const NP: usize, const NQ: usize, const NR: usize>() {
    let dims = format!("{:?}", (NP, NQ, NR));
    let elapsed = bench::<NP, NQ, NR>().as_secs_f64();
    println!("{:<14} | {:<30} | {:.7} s", "doitgen", dims, elapsed);
}

fn main() {
    bench_and_print::<35, 37, 40>();
    bench_and_print::<70, 75, 80>();
    bench_and_print::<140, 150, 160>();
}

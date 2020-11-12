#![feature(min_const_generics)]

use polybench_rs::medley::deriche::bench;

fn bench_and_print<const H: usize, const W: usize>() {
    let dims = format!("{:?}", (H, W));
    let elapsed = bench::<H, W>().as_secs_f64();
    println!("{:<14} | {:<30} | {:.7} s", "deriche", dims, elapsed);
}

fn main() {
    bench_and_print::<1024, 540>();
    bench_and_print::<2048, 1080>();
    bench_and_print::<4096, 2160>();
}

use polybench_rs::linear_algebra::kernels::bicg::bench;

fn main() {
    println!("{:<14}: {:>15}", "bicg", bench().as_nanos());
}

use polybench_rs::linear_algebra::kernels::atax::bench;

fn main() {
    println!("{:<14}: {:>15}", "atax", bench().as_nanos());
}

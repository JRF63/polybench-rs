use polybench_rs::linear_algebra::kernels::mvt::bench;

fn main() {
    println!("{:<14}: {:>15}", "mvt", bench().as_nanos());
}

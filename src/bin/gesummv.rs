use polybench_rs::linear_algebra::blas::gesummv::bench;

fn main() {
    println!("{:<14}: {:>15}", "gesummv", bench().as_nanos());
}

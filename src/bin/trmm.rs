use polybench_rs::linear_algebra::blas::trmm::bench;

fn main() {
    println!("{:<14}: {:>15}", "trmm", bench().as_nanos());
}

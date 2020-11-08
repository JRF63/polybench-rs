use polybench_rs::linear_algebra::blas::symm::bench;

fn main() {
    println!("{:<14}: {:>15}", "symm", bench().as_nanos());
}

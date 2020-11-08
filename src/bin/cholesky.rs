use polybench_rs::linear_algebra::solvers::cholesky::bench;

fn main() {
    println!("{:<14}: {:>15}", "cholesky", bench().as_nanos());
}

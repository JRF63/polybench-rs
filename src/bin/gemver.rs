use polybench_rs::linear_algebra::blas::gemver::bench;

fn main() {
    println!("{:<14}: {:>15}", "gemver", bench().as_nanos());
}

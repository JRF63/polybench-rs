use polybench_rs::linear_algebra::blas::syrk::bench;

fn main() {
    println!("{:<14}: {:>15}", "syrk", bench().as_nanos());
}

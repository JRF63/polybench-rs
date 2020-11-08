use polybench_rs::linear_algebra::blas::syr2k::bench;

fn main() {
    println!("{:<14}: {:>15}", "syr2k", bench().as_nanos());
}

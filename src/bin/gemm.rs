use polybench_rs::linear_algebra::blas::gemm::bench;

fn main() {
    println!("{:<14}: {:>15}", "gemm", bench().as_nanos());
}

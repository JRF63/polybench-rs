use polybench_rs::linear_algebra::kernels::doitgen::bench;

fn main() {
    println!("{:<14}: {:>15}", "doitgen", bench().as_nanos());
}

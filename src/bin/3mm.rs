use polybench_rs::linear_algebra::kernels::_3mm::bench;

fn main() {
    println!("{:<14}: {:>15}", "3mm", bench().as_nanos());
}

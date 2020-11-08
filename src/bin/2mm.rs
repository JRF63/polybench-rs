use polybench_rs::linear_algebra::kernels::_2mm::bench;

fn main() {
    println!("{:<14}: {:>15}", "2mm", bench().as_nanos());
}

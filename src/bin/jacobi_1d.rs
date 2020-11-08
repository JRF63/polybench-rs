use polybench_rs::stencils::jacobi_1d::bench;

fn main() {
    println!("{:<14}: {:>15}", "jacobi_1d", bench().as_nanos());
}

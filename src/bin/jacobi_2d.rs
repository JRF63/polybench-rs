use polybench_rs::stencils::jacobi_2d::bench;

fn main() {
    println!("{:<14}: {:>15}", "jacobi_2d", bench().as_nanos());
}

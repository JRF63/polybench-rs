use polybench_rs::stencils::fdtd_2d::bench;

fn main() {
    println!("{:<14}: {:>15}", "fdtd_2d", bench().as_nanos());
}

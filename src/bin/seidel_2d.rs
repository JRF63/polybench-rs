use polybench_rs::stencils::seidel_2d::bench;

fn main() {
    println!("{:<14}: {:>15}", "seidel_2d", bench().as_nanos());
}

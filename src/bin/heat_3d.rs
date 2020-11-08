use polybench_rs::stencils::heat_3d::bench;

fn main() {
    println!("{:<14}: {:>15}", "heat_3d", bench().as_nanos());
}

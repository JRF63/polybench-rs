use polybench_rs::stencils::adi::bench;

fn main() {
    println!("{:<14}: {:>15}", "adi", bench().as_nanos());
}

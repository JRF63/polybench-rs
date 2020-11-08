use polybench_rs::linear_algebra::solvers::trisolv::bench;

fn main() {
    println!("{:<14}: {:>15}", "trisolv", bench().as_nanos());
}

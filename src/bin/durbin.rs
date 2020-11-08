use polybench_rs::linear_algebra::solvers::durbin::bench;

fn main() {
    println!("{:<14}: {:>15}", "durbin", bench().as_nanos());
}

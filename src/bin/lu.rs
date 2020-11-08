use polybench_rs::linear_algebra::solvers::lu::bench;

fn main() {
    println!("{:<14}: {:>15}", "lu", bench().as_nanos());
}

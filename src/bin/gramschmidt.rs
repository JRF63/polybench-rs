use polybench_rs::linear_algebra::solvers::gramschmidt::bench;

fn main() {
    println!("{:<14}: {:>15}", "gramschmidt", bench().as_nanos());
}

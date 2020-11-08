use polybench_rs::linear_algebra::solvers::ludcmp::bench;

fn main() {
    println!("{:<14}: {:>15}", "ludcmp", bench().as_nanos());
}

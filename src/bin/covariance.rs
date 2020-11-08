use polybench_rs::datamining::covariance::bench;

fn main() {
    println!("{:<14}: {:>15}", "covariance", bench().as_nanos());
}

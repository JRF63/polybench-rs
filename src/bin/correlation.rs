use polybench_rs::datamining::correlation::bench;

fn main() {
    println!("{:<14}: {:>15}", "correlation", bench().as_nanos());
}

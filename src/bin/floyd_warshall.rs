use polybench_rs::medley::floyd_warshall::bench;

fn main() {
    println!("{:<14}: {:>15}", "floyd_warshall", bench().as_nanos());
}

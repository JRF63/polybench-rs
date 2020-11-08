use polybench_rs::medley::nussinov::bench;

fn main() {
    println!("{:<14}: {:>15}", "nussinov", bench().as_nanos());
}

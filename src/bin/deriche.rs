use polybench_rs::medley::deriche::bench;

fn main() {
    println!("{:<14}: {:>15}", "deriche", bench().as_nanos());
}

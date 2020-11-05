pub use bencher::black_box;
use std::time::Duration;

const LLC_CACHE_SIZE: usize = 32 * 1024 * 1024; // 32 MiB

pub fn flush_llc_cache() {
    let num_elems = (LLC_CACHE_SIZE - 1) / std::mem::size_of::<usize>() + 1;
    let mut buf: Vec<usize> = Vec::with_capacity(num_elems);
    buf.resize(num_elems, Default::default());
    let sum: usize = buf.iter().sum();
    black_box(sum);
}

pub fn max_duration() -> Duration {
    Duration::new(u64::MAX, 1000000000 - 1)
}
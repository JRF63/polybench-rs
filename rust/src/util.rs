use std::time::Duration;

const LLC_CACHE_SIZE: usize = 32 * 1024 * 1024; // 32 MiB

// Lifted from bencher crate:
// https://docs.rs/bencher/0.1.5/src/bencher/lib.rs.html#590-596
pub fn black_box<T>(dummy: T) -> T {
    unsafe {
        let ret = std::ptr::read_volatile(&dummy);
        std::mem::forget(dummy);
        ret
    }
}

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

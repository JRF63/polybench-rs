use std::fmt;
use std::time::{Duration, Instant};

pub fn consume<T: fmt::Display>(dummy: T) -> T {
    #[cfg(feature = "print-result")]
    println!("{}", &dummy);

    unsafe {
        // Taken from bencher crate:
        // https://docs.rs/bencher/0.1.5/src/bencher/lib.rs.html#590-596
        let ret = std::ptr::read_volatile(&dummy);
        std::mem::forget(dummy);
        ret
    }
}

fn flush_llc_cache() {
    const LLC_CACHE_SIZE: usize = 32 * 1024 * 1024; // 32 MiB
    const NUM_ELEMS: usize = (LLC_CACHE_SIZE - 1) / std::mem::size_of::<usize>() + 1;

    let mut buf: Vec<usize> = Vec::with_capacity(NUM_ELEMS);
    buf.resize(NUM_ELEMS, Default::default());
    let sum: usize = buf.iter().sum();
    consume(sum);
}

#[inline(always)]
pub fn time_function<F: FnOnce()>(f: F) -> Duration {
    flush_llc_cache();
    let now = Instant::now();
    f();
    now.elapsed()
}

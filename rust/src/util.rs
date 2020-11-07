use crate::ndarray::AllocUninit;
use crate::ndarray::Array2D;
use std::time::Duration;

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
    const LLC_CACHE_SIZE: usize = 32 * 1024 * 1024; // 32 MiB
    const NUM_ELEMS: usize = (LLC_CACHE_SIZE - 1) / std::mem::size_of::<usize>() + 1;
    
    let mut buf: Vec<usize> = Vec::with_capacity(NUM_ELEMS);
    buf.resize(NUM_ELEMS, Default::default());
    let sum: usize = buf.iter().sum();
    black_box(sum);
}

pub fn max_duration() -> Duration {
    Duration::new(u64::MAX, 1000000000 - 1)
}

pub fn into_positive_semi_definite<const N: usize>(a: &mut Array2D<N, N>) {
    let mut b = Array2D::<N, N>::uninit();
    let n = N;
    for r in 0..n {
        for s in 0..n {
            b[(r, s)] = 0.0;
        }
    }
    for t in 0..n {
        for r in 0..n {
            for s in 0..n {
                b[(r, s)] += a[(r, t)] * a[(s, t)];
            }
        }
    }
    for r in 0..n {
        for s in 0..n {
            a[(r, s)] = b[(r, s)];
        }
    }
}
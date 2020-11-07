use crate::ndarray::AllocUninit;
use crate::ndarray::Array2D;
use crate::util;
use std::time::{Duration, Instant};

const M: usize = 1000;
const N: usize = 1200;

unsafe fn init_array(
    m: usize,
    n: usize,
    a: &mut Array2D<M, N>,
    r: &mut Array2D<N, N>,
    q: &mut Array2D<M, N>,
) {
    for i in 0..m {
        for j in 0..n {
            a[(i, j)] = ((((i * j) % m) as f64 / m as f64) * 100.0) + 10.0;
            q[(i, j)] = 0.0;
        }
    }
    for i in 0..n {
        for j in 0..n {
            r[(i, j)] = 0.0;
        }
    }
}

unsafe fn kernel_gramschmidt(
    m: usize,
    n: usize,
    a: &mut Array2D<M, N>,
    r: &mut Array2D<N, N>,
    q: &mut Array2D<M, N>,
) {
    for k in 0..n {
        let mut nrm = 0.0;
        for i in 0..m {
            nrm += a[(i, k)] * a[(i, k)];
        }
        r[(k, k)] = nrm.sqrt();
        for i in 0..m {
            q[(i, k)] = a[(i, k)] / r[(k, k)];
        }
        for j in (k + 1)..n {
            r[(k, j)] = 0.0;
            for i in 0..m {
                r[(k, j)] += q[(i, k)] * a[(i, j)];
            }
            for i in 0..m {
                a[(i, j)] = a[(i, j)] - q[(i, k)] * r[(k, j)];
            }
        }
    }
}

pub fn bench(num_runs: usize) -> Duration {
    let m = M;
    let n = N;

    let mut a = Array2D::uninit();
    let mut r = Array2D::uninit();
    let mut q = Array2D::uninit();

    let mut min_dur = util::max_duration();
    for _ in 0..num_runs {
        unsafe {
            init_array(m, n, &mut a, &mut r, &mut q);

            util::flush_llc_cache();

            let now = Instant::now();
            kernel_gramschmidt(m, n, &mut a, &mut r, &mut q);
            let elapsed = now.elapsed();

            util::black_box(&a);
            util::black_box(&r);
            util::black_box(&q);

            if elapsed < min_dur {
                min_dur = elapsed;
            }
        }
    }
    min_dur
}

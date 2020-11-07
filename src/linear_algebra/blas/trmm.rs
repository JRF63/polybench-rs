use crate::ndarray::AllocUninit;
use crate::ndarray::Array2D;
use crate::util;
use std::time::{Duration, Instant};

const M: usize = 1000;
const N: usize = 1200;

unsafe fn init_array(
    m: usize,
    n: usize,
    alpha: &mut f64,
    a: &mut Array2D<M, M>,
    b: &mut Array2D<M, N>,
) {
    *alpha = 1.5;
    for i in 0..m {
        for j in 0..i {
            a[(i, j)] = ((i + j) % m) as f64 / m as f64;
        }
        a[(i, i)] = 1.0;
        for j in 0..n {
            b[(i, j)] = (((n + i) - j) % n) as f64 / n as f64;
        }
    }
}

unsafe fn kernel_trmm(m: usize, n: usize, alpha: f64, a: &Array2D<M, M>, b: &mut Array2D<M, N>) {
    for i in 0..m {
        for j in 0..n {
            for k in (i + 1)..m {
                b[(i, j)] += a[(k, i)] * b[(k, j)];
            }
            b[(i, j)] = alpha * b[(i, j)];
        }
    }
}

pub fn bench(num_runs: usize) -> Duration {
    let m = M;
    let n = N;

    let mut alpha = 0.0;
    let mut a = Array2D::uninit();
    let mut b = Array2D::uninit();

    let mut min_dur = util::max_duration();
    for _ in 0..num_runs {
        unsafe {
            init_array(m, n, &mut alpha, &mut a, &mut b);

            util::flush_llc_cache();

            let now = Instant::now();
            kernel_trmm(m, n, alpha, &a, &mut b);
            let elapsed = now.elapsed();

            util::black_box(&b);

            if elapsed < min_dur {
                min_dur = elapsed;
            }
        }
    }
    min_dur
}

use crate::ndarray::AllocUninit;
use crate::ndarray::{Array1D, Array2D};
use crate::util;
use std::time::{Duration, Instant};

const M: usize = 1900;
const N: usize = 2100;

unsafe fn init_array(
    m: usize,
    n: usize,
    a: &mut Array2D<N, M>,
    r: &mut Array1D<N>,
    p: &mut Array1D<M>,
) {
    for i in 0..m {
        p[i] = (i % m) as f64 / m as f64;
    }
    for i in 0..n {
        r[i] = (i % n) as f64 / n as f64;
        for j in 0..m {
            a[(i, j)] = (i * (j + 1) % n) as f64 / n as f64;
        }
    }
}

unsafe fn kernel_bicg(
    m: usize,
    n: usize,
    a: &Array2D<N, M>,
    s: &mut Array1D<M>,
    q: &mut Array1D<N>,
    p: &Array1D<M>,
    r: &Array1D<N>,
) {
    for i in 0..m {
        s[i] = 0.0;
    }
    for i in 0..n {
        q[i] = 0.0;
        for j in 0..m {
            s[j] = s[j] + r[i] * a[(i, j)];
            q[i] = q[i] + a[(i, j)] * p[j];
        }
    }
}

pub fn bench(num_runs: usize) -> Duration {
    let n = N;
    let m = M;

    let mut a = Array2D::uninit();
    let mut s = Array1D::uninit();
    let mut q = Array1D::uninit();
    let mut p = Array1D::uninit();
    let mut r = Array1D::uninit();

    let mut min_dur = util::max_duration();
    for _ in 0..num_runs {
        unsafe {
            init_array(m, n, &mut a, &mut r, &mut p);

            util::flush_llc_cache();

            let now = Instant::now();
            kernel_bicg(m, n, &a, &mut s, &mut q, &p, &r);
            let elapsed = now.elapsed();

            util::black_box(&s);
            util::black_box(&q);

            if elapsed < min_dur {
                min_dur = elapsed;
            }
        }
    }
    min_dur
}

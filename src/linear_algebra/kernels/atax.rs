use crate::ndarray::AllocUninit;
use crate::ndarray::{Array1D, Array2D};
use crate::util;
use std::time::{Duration, Instant};

const M: usize = 1900;
const N: usize = 2100;

unsafe fn init_array(m: usize, n: usize, a: &mut Array2D<M, N>, x: &mut Array1D<N>) {
    let float_n = n as f64;
    for i in 0..n {
        x[i] = 1.0 + (i as f64 / float_n);
    }
    for i in 0..m {
        for j in 0..n {
            a[(i, j)] = ((i + j) % n) as f64 / (5 * m) as f64;
        }
    }
}

unsafe fn kernel_atax(
    m: usize,
    n: usize,
    a: &Array2D<M, N>,
    x: &Array1D<N>,
    y: &mut Array1D<N>,
    tmp: &mut Array1D<M>,
) {
    for i in 0..n {
        y[i] = 0.0;
    }
    for i in 0..m {
        tmp[i] = 0.0;
        for j in 0..n {
            tmp[i] = tmp[i] + a[(i, j)] * x[j];
        }
        for j in 0..n {
            y[j] = y[j] + a[(i, j)] * tmp[i];
        }
    }
}

pub fn bench(num_runs: usize) -> Duration {
    let m = M;
    let n = N;

    let mut a = Array2D::uninit();
    let mut x = Array1D::uninit();
    let mut y = Array1D::uninit();
    let mut tmp = Array1D::uninit();

    let mut min_dur = util::max_duration();
    for _ in 0..num_runs {
        unsafe {
            init_array(m, n, &mut a, &mut x);

            util::flush_llc_cache();

            let now = Instant::now();
            kernel_atax(m, n, &a, &x, &mut y, &mut tmp);
            let elapsed = now.elapsed();

            util::black_box(&y);

            if elapsed < min_dur {
                min_dur = elapsed;
            }
        }
    }
    min_dur
}

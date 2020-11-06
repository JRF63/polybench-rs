use crate::ndarray::AllocUninit;
use crate::ndarray::{Array1D, Array2D};
use crate::util;
use std::time::{Duration, Instant};

const N: usize = 1300;

unsafe fn init_array(
    n: usize,
    alpha: &mut f64,
    beta: &mut f64,
    a: &mut Array2D<N, N>,
    b: &mut Array2D<N, N>,
    x: &mut Array1D<N>,
) {
    *alpha = 1.5;
    *beta = 1.2;
    for i in 0..n {
        x[i] = (i % n) as f64 / n as f64;
        for j in 0..n {
            a[(i, j)] = ((i * j + 1) % n) as f64 / n as f64;
            b[(i, j)] = ((i * j + 2) % n) as f64 / n as f64;
        }
    }
}

unsafe fn kernel_gesummv(
    n: usize,
    alpha: f64,
    beta: f64,
    a: &Array2D<N, N>,
    b: &Array2D<N, N>,
    tmp: &mut Array1D<N>,
    x: &Array1D<N>,
    y: &mut Array1D<N>,
) {
    for i in 0..n {
        tmp[i] = 0.0;
        y[i] = 0.0;
        for j in 0..n {
            tmp[i] = a[(i, j)] * x[j] + tmp[i];
            y[i] = b[(i, j)] * x[j] + y[i];
        }
        y[i] = alpha * tmp[i] + beta * y[i];
    }
}

pub fn bench(num_runs: usize) -> Duration {
    let n = N;

    let mut alpha = 0.0;
    let mut beta = 0.0;
    let mut a = Array2D::uninit();
    let mut b = Array2D::uninit();
    let mut tmp = Array1D::uninit();
    let mut x = Array1D::uninit();
    let mut y = Array1D::uninit();

    let mut min_dur = util::max_duration();
    for _ in 0..num_runs {
        unsafe {
            init_array(n, &mut alpha, &mut beta, &mut a, &mut b, &mut x);

            util::flush_llc_cache();

            let now = Instant::now();
            kernel_gesummv(n, alpha, beta, &a, &b, &mut tmp, &x, &mut y);
            let elapsed = now.elapsed();

            util::black_box(&y);

            if elapsed < min_dur {
                min_dur = elapsed;
            }
        }
    }
    min_dur
}

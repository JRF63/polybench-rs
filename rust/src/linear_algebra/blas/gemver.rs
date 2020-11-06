use crate::ndarray::AllocUninit;
use crate::ndarray::{Array1D, Array2D};
use crate::util;
use std::time::{Duration, Instant};

const N: usize = 2000;

unsafe fn init_array(
    n: usize,
    alpha: &mut f64,
    beta: &mut f64,
    a: &mut Array2D<N, N>,
    u1: &mut Array1D<N>,
    v1: &mut Array1D<N>,
    u2: &mut Array1D<N>,
    v2: &mut Array1D<N>,
    w: &mut Array1D<N>,
    x: &mut Array1D<N>,
    y: &mut Array1D<N>,
    z: &mut Array1D<N>,
) {
    *alpha = 1.5;
    *beta = 1.2;

    let float_n = n as f64;

    for i in 0..n {
        u1[i] = i as f64;
        u2[i] = ((i + 1) as f64 / float_n) / 2.0;
        v1[i] = ((i + 1) as f64 / float_n) / 4.0;
        v2[i] = ((i + 1) as f64 / float_n) / 6.0;
        y[i] = ((i + 1) as f64 / float_n) / 8.0;
        z[i] = ((i + 1) as f64 / float_n) / 9.0;
        x[i] = 0.0;
        w[i] = 0.0;
        for j in 0..n {
            a[(i, j)] = (i * j % n) as f64 / n as f64;
        }
    }
}

unsafe fn kernel_gemver(
    n: usize,
    alpha: f64,
    beta: f64,
    a: &mut Array2D<N, N>,
    u1: &Array1D<N>,
    v1: &Array1D<N>,
    u2: &Array1D<N>,
    v2: &Array1D<N>,
    w: &mut Array1D<N>,
    x: &mut Array1D<N>,
    y: &Array1D<N>,
    z: &Array1D<N>,
) {
    for i in 0..n {
        for j in 0..n {
            a[(i, j)] = a[(i, j)] + u1[i] * v1[j] + u2[i] * v2[j];
        }
    }

    for i in 0..n {
        for j in 0..n {
            x[i] = x[i] + beta * a[(j, i)] * y[j];
        }
    }

    for i in 0..n {
        x[i] = x[i] + z[i];
    }

    for i in 0..n {
        for j in 0..n {
            w[i] = w[i] + alpha * a[(i, j)] * x[j];
        }
    }
}

pub fn bench(num_runs: usize) -> Duration {
    let n = N;

    let mut alpha = 0.0;
    let mut beta = 0.0;
    let mut a = Array2D::uninit();
    let mut u1 = Array1D::uninit();
    let mut v1 = Array1D::uninit();
    let mut u2 = Array1D::uninit();
    let mut v2 = Array1D::uninit();
    let mut w = Array1D::uninit();
    let mut x = Array1D::uninit();
    let mut y = Array1D::uninit();
    let mut z = Array1D::uninit();

    let mut min_dur = util::max_duration();
    for _ in 0..num_runs {
        unsafe {
            init_array(
                n, &mut alpha, &mut beta, &mut a, &mut u1, &mut v1, &mut u2, &mut v2, &mut w,
                &mut x, &mut y, &mut z,
            );

            util::flush_llc_cache();

            let now = Instant::now();
            kernel_gemver(
                n, alpha, beta, &mut a, &u1, &v1, &u2, &v2, &mut w, &mut x, &y, &z,
            );
            let elapsed = now.elapsed();

            util::black_box(&w);

            if elapsed < min_dur {
                min_dur = elapsed;
            }
        }
    }
    min_dur
}

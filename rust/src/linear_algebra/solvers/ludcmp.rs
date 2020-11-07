use crate::ndarray::AllocUninit;
use crate::ndarray::{Array1D, Array2D};
use crate::util;
use std::time::{Duration, Instant};

const N: usize = 2000;

unsafe fn init_array(
    n: usize,
    a: &mut Array2D<N, N>,
    b: &mut Array1D<N>,
    x: &mut Array1D<N>,
    y: &mut Array1D<N>,
) {
    let float_n = n as f64;

    for i in 0..n {
        x[i] = 0.0;
        y[i] = 0.0;
        b[i] = (i + 1) as f64 / float_n / 2.0 + 4.0;
    }

    for i in 0..n {
        for j in 0..=i {
            a[(i, j)] = (-(j as isize) % n as isize) as f64 / n as f64 + 1.0;
        }
        for j in (i + 1)..n {
            a[(i, j)] = 0.0;
        }
        a[(i, i)] = 1.0;
    }

    util::into_positive_semi_definite(a);
}

unsafe fn kernel_ludcmp(
    n: usize,
    a: &mut Array2D<N, N>,
    b: &Array1D<N>,
    x: &mut Array1D<N>,
    y: &mut Array1D<N>,
) {
    let mut w;
    for i in 0..n {
        for j in 0..i {
            w = a[(i, j)];
            for k in 0..j {
                w -= a[(i, k)] * a[(k, j)];
            }
            a[(i, j)] = w / a[(j, j)];
        }
        for j in i..n {
            w = a[(i, j)];
            for k in 0..i {
                w -= a[(i, k)] * a[(k, j)];
            }
            a[(i, j)] = w;
        }
    }

    for i in 0..n {
        w = b[i];
        for j in 0..i {
            w -= a[(i, j)] * y[j];
        }
        y[i] = w;
    }

    for i in (0..n).rev() {
        w = y[i];
        for j in (i + 1)..n {
            w -= a[(i, j)] * x[j];
        }
        x[i] = w / a[(i, i)];
    }
}

pub fn bench(num_runs: usize) -> Duration {
    let n = N;

    let mut a = Array2D::uninit();
    let mut b = Array1D::uninit();
    let mut x = Array1D::uninit();
    let mut y = Array1D::uninit();

    let mut min_dur = util::max_duration();
    for _ in 0..num_runs {
        unsafe {
            init_array(n, &mut a, &mut b, &mut x, &mut y);

            util::flush_llc_cache();

            let now = Instant::now();
            kernel_ludcmp(n, &mut a, &b, &mut x, &mut y);
            let elapsed = now.elapsed();

            util::black_box(&x);

            if elapsed < min_dur {
                min_dur = elapsed;
            }
        }
    }
    min_dur
}

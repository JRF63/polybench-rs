use crate::ndarray::AllocUninit;
use crate::ndarray::{Array1D, Array2D};
use crate::util;
use std::time::{Duration, Instant};

const N: usize = 2000;

unsafe fn init_array(
    n: usize,
    x1: &mut Array1D<N>,
    x2: &mut Array1D<N>,
    y_1: &mut Array1D<N>,
    y_2: &mut Array1D<N>,
    a: &mut Array2D<N, N>,
) {
    for i in 0..n {
        x1[i] = (i % n) as f64 / n as f64;
        x2[i] = ((i + 1) % n) as f64 / n as f64;
        y_1[i] = ((i + 3) % n) as f64 / n as f64;
        y_2[i] = ((i + 4) % n) as f64 / n as f64;
        for j in 0..n {
            a[(i, j)] = (i * j % n) as f64 / n as f64;
        }
    }
}

unsafe fn kernel_mvt(
    n: usize,
    x1: &mut Array1D<N>,
    x2: &mut Array1D<N>,
    y_1: &Array1D<N>,
    y_2: &Array1D<N>,
    a: &Array2D<N, N>,
) {
    for i in 0..n {
        for j in 0..n {
            x1[i] = x1[i] + a[(i, j)] * y_1[j];
        }
    }
    for i in 0..n {
        for j in 0..n {
            x2[i] = x2[i] + a[(j, i)] * y_2[j];
        }
    }
}

pub fn bench(num_runs: usize) -> Duration {
    let n = N;

    let mut a = Array2D::uninit();
    let mut x1 = Array1D::uninit();
    let mut x2 = Array1D::uninit();
    let mut y_1 = Array1D::uninit();
    let mut y_2 = Array1D::uninit();

    let mut min_dur = util::max_duration();
    for _ in 0..num_runs {
        unsafe {
            init_array(n, &mut x1, &mut x2, &mut y_1, &mut y_2, &mut a);

            util::flush_llc_cache();

            let now = Instant::now();
            kernel_mvt(n, &mut x1, &mut x2, &y_1, &y_2, &a);
            let elapsed = now.elapsed();

            util::black_box(&x1);
            util::black_box(&x2);

            if elapsed < min_dur {
                min_dur = elapsed;
            }
        }
    }
    min_dur
}

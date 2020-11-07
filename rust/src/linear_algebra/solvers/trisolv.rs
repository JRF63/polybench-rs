use crate::ndarray::AllocUninit;
use crate::ndarray::{Array1D, Array2D};
use crate::util;
use std::time::{Duration, Instant};

const N: usize = 2000;

unsafe fn init_array(n: usize, l: &mut Array2D<N, N>, x: &mut Array1D<N>, b: &mut Array1D<N>) {
    for i in 0..n {
        x[i] = -999.0;
        b[i] = i as f64;
        for j in 0..=i {
            l[(i, j)] = (i + n - j + 1) as f64 * 2.0 / n as f64;
        }
    }
}

unsafe fn kernel_trisolv(n: usize, l: &Array2D<N, N>, x: &mut Array1D<N>, b: &Array1D<N>) {
    for i in 0..n {
        x[i] = b[i];
        for j in 0..i {
            x[i] -= l[(i, j)] * x[j];
        }
        x[i] = x[i] / l[(i, i)];
    }
}

pub fn bench(num_runs: usize) -> Duration {
    let n = N;

    let mut l = Array2D::uninit();
    let mut x = Array1D::uninit();
    let mut b = Array1D::uninit();

    let mut min_dur = util::max_duration();
    for _ in 0..num_runs {
        unsafe {
            init_array(n, &mut l, &mut x, &mut b);

            util::flush_llc_cache();

            let now = Instant::now();
            kernel_trisolv(n, &l, &mut x, &b);
            let elapsed = now.elapsed();

            util::black_box(&x);

            if elapsed < min_dur {
                min_dur = elapsed;
            }
        }
    }
    min_dur
}

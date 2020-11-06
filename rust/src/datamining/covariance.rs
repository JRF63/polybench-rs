use crate::ndarray::AllocUninit;
use crate::ndarray::{Array1D, Array2D};
use crate::util;
use std::time::{Duration, Instant};

const M: usize = 1200;
const N: usize = 1400;

unsafe fn init_array(m: usize, n: usize, float_n: &mut f64, data: &mut Array2D<N, M>) {
    *float_n = n as f64;
    for i in 0..n {
        for j in 0..m {
            data[(i, j)] = (i*j) as f64 / M as f64;
        }
    }
}

unsafe fn kernel_covariance(
    m: usize,
    n: usize,
    float_n: f64,
    data: &mut Array2D<N, M>,
    cov: &mut Array2D<M, M>,
    mean: &mut Array1D<M>,
) {
    for j in 0..m {
        mean[j] = 0.0;
        for i in 0..n {
            mean[j] += data[(i, j)];
        }
        mean[j] /= float_n;
    }

    for i in 0..n {
        for j in 0..m {
            data[(i, j)] -= mean[j];
        }
    }

    for i in 0..m {
        for j in i..m {
            cov[(i, j)] = 0.0;
            for k in 0..n {
                cov[(i, j)] += data[(k, i)] * data[(k, j)];
            }
            cov[(i, j)] /= float_n - 1.0;
            cov[(j, i)] = cov[(i, j)];
        }
    }
}

pub fn bench(num_runs: usize) -> Duration {
    let n = N;
    let m = M;

    let mut float_n = 0.0;
    let mut data = Array2D::uninit();
    let mut cov = Array2D::uninit();
    let mut mean = Array1D::uninit();

    let mut min_dur = util::max_duration();
    for _ in 0..num_runs {
        unsafe {
            init_array(m, n, &mut float_n, &mut data);

            util::flush_llc_cache();

            let now = Instant::now();
            kernel_covariance(m, n, float_n, &mut data, &mut cov, &mut mean);
            let elapsed = now.elapsed();

            util::black_box(&cov);

            if elapsed < min_dur {
                min_dur = elapsed;
            }
        }
    }
    min_dur
}

use crate::ndarray::AllocUninit;
use crate::ndarray::Array2D;
use crate::util;
use std::time::{Duration, Instant};

const M: usize = 1000;
const N: usize = 1200;

unsafe fn init_array(
    n: usize,
    m: usize,
    alpha: &mut f64,
    beta: &mut f64,
    c: &mut Array2D<N, N>,
    a: &mut Array2D<N, M>,
) {
    *alpha = 1.5;
    *beta = 1.2;
    for i in 0..n {
        for j in 0..m {
            a[(i, j)] = ((i * j + 1) % n) as f64 / n as f64;
        }
    }

    for i in 0..n {
        for j in 0..n {
            c[(i, j)] = ((i * j + 2) % m) as f64 / m as f64;
        }
    }
}

unsafe fn kernel_syrk(
    n: usize,
    m: usize,
    alpha: f64,
    beta: f64,
    c: &mut Array2D<N, N>,
    a: &Array2D<N, M>,
) {
    for i in 0..n {
        for j in 0..=i {
            c[(i, j)] *= beta;
        }
        for k in 0..m {
            for j in 0..=i {
                c[(i, j)] += alpha * a[(i, k)] * a[(j, k)];
            }
        }
    }
}

pub fn bench(num_runs: usize) -> Duration {
    let n = N;
    let m = M;
    
    let mut alpha = 0.0;
    let mut beta = 0.0;
    let mut c = Array2D::uninit();
    let mut a = Array2D::uninit();

    let mut min_dur = util::max_duration();
    for _ in 0..num_runs {
        unsafe {
            init_array(n, m, &mut alpha, &mut beta, &mut c, &mut a);

            util::flush_llc_cache();

            let now = Instant::now();
            kernel_syrk(n, m, alpha, beta, &mut c, &a);
            let elapsed = now.elapsed();

            util::black_box(&c);

            if elapsed < min_dur {
                min_dur = elapsed;
            }
        }
    }
    min_dur
}

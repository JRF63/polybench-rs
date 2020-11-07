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
    beta: &mut f64,
    c: &mut Array2D<M, N>,
    a: &mut Array2D<M, M>,
    b: &mut Array2D<M, N>,
) {
    *alpha = 1.5;
    *beta = 1.2;
    for i in 0..m {
        for j in 0..n {
            c[(i, j)] = ((i + j) % 100) as f64 / m as f64;
            b[(i, j)] = ((n + i - j) % 100) as f64 / m as f64;
        }
    }

    for i in 0..m {
        for j in 0..=i {
            a[(i, j)] = ((i + j) % 100) as f64 / m as f64;
        }
        for j in (i + 1)..m {
            a[(i, j)] = -999 as f64;
        }
    }
}

unsafe fn kernel_symm(
    m: usize,
    n: usize,
    alpha: f64,
    beta: f64,
    c: &mut Array2D<M, N>,
    a: &Array2D<M, M>,
    b: &Array2D<M, N>,
) {
    for i in 0..m {
        for j in 0..n {
            let mut temp2 = 0.0;
            for k in 0..i {
                c[(k, j)] += alpha * b[(i, j)] * a[(i, k)];
                temp2 += b[(k, j)] * a[(i, k)];
            }
            c[(i, j)] = beta * c[(i, j)] + alpha * b[(i, j)] * a[(i, i)] + alpha * temp2;
        }
    }
}

pub fn bench(num_runs: usize) -> Duration {
    let m = M;
    let n = N;

    let mut alpha = 0.0;
    let mut beta = 0.0;
    let mut c = Array2D::uninit();
    let mut a = Array2D::uninit();
    let mut b = Array2D::uninit();

    let mut min_dur = util::max_duration();
    for _ in 0..num_runs {
        unsafe {
            init_array(m, n, &mut alpha, &mut beta, &mut c, &mut a, &mut b);

            util::flush_llc_cache();

            let now = Instant::now();
            kernel_symm(m, n, alpha, beta, &mut c, &a, &b);
            let elapsed = now.elapsed();

            util::black_box(&c);

            if elapsed < min_dur {
                min_dur = elapsed;
            }
        }
    }
    min_dur
}

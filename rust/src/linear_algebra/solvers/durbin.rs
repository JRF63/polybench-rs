use crate::ndarray::AllocUninit;
use crate::ndarray::Array1D;
use crate::util;
use std::time::{Duration, Instant};

const N: usize = 2000;

unsafe fn init_array(n: usize, r: &mut Array1D<N>) {
    for i in 0..n {
        r[i] = (n + 1 - i) as f64;
    }
}

unsafe fn kernel_durbin(n: usize, r: &Array1D<N>, y: &mut Array1D<N>) {
    let mut z: [f64; N] = std::mem::MaybeUninit::uninit().assume_init();

    y[0] = -r[0];
    let mut beta = 1.0;
    let mut alpha = -r[0];
    for k in 1..n {
        beta = (1.0 - alpha * alpha) * beta;
        let mut sum = 0.0;
        for i in 0..k {
            sum += r[k - i - 1] * y[i];
        }
        alpha = -(r[k] + sum) / beta;

        for i in 0..k {
            z[i] = y[i] + alpha * y[k - i - 1];
        }
        for i in 0..k {
            y[i] = z[i];
        }
        y[k] = alpha;
    }
}

pub fn bench(num_runs: usize) -> Duration {
    let n = N;

    let mut r = Array1D::uninit();
    let mut y = Array1D::uninit();

    let mut min_dur = util::max_duration();
    for _ in 0..num_runs {
        unsafe {
            init_array(n, &mut r);

            util::flush_llc_cache();

            let now = Instant::now();
            kernel_durbin(n, &r, &mut y);
            let elapsed = now.elapsed();

            util::black_box(&y);

            if elapsed < min_dur {
                min_dur = elapsed;
            }
        }
    }
    min_dur
}

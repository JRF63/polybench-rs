use crate::ndarray::AllocUninit;
use crate::ndarray::Array2D;
use crate::util;
use std::time::{Duration, Instant};

const N: usize = 2000;

unsafe fn init_array(n: usize, a: &mut Array2D<N, N>) {
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

unsafe fn kernel_lu(n: usize, a: &mut Array2D<N, N>) {
    for i in 0..n {
        for j in 0..i {
            for k in 0..j {
                a[(i, j)] -= a[(i, k)] * a[(k, j)];
            }
            a[(i, j)] /= a[(j, j)];
        }
        for j in i..n {
            for k in 0..i {
                a[(i, j)] -= a[(i, k)] * a[(k, j)];
            }
        }
    }
}

pub fn bench(num_runs: usize) -> Duration {
    let n = N;

    let mut a = Array2D::uninit();

    let mut min_dur = util::max_duration();
    for _ in 0..num_runs {
        unsafe {
            init_array(n, &mut a);

            util::flush_llc_cache();

            let now = Instant::now();
            kernel_lu(n, &mut a);
            let elapsed = now.elapsed();

            util::black_box(&a);

            if elapsed < min_dur {
                min_dur = elapsed;
            }
        }
    }
    min_dur
}

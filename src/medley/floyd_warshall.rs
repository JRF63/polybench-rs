use crate::ndarray::AllocUninit;
use crate::ndarray::Array2D;
use crate::util;
use std::time::{Duration, Instant};

const N: usize = 2800;

unsafe fn init_array(n: usize, path: &mut Array2D<N, N>) {
    for i in 0..n {
        for j in 0..n {
            path[(i, j)] = (i * j % 7 + 1) as f64;
            if (i + j) % 13 == 0 || (i + j) % 7 == 0 || (i + j) % 11 == 0 {
                path[(i, j)] = 999 as f64;
            }
        }
    }
}

unsafe fn kernel_floyd_warshall(n: usize, path: &mut Array2D<N, N>) {
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                path[(i, j)] = if path[(i, j)] < path[(i, k)] + path[(k, j)] {
                    path[(i, j)]
                } else {
                    path[(i, k)] + path[(k, j)]
                };
            }
        }
    }
}

pub fn bench(num_runs: usize) -> Duration {
    let n = N;

    let mut path = Array2D::uninit();

    let mut min_dur = util::max_duration();
    for _ in 0..num_runs {
        unsafe {
            init_array(n, &mut path);

            util::flush_llc_cache();

            let now = Instant::now();
            kernel_floyd_warshall(n, &mut path);
            let elapsed = now.elapsed();

            util::black_box(&path);

            if elapsed < min_dur {
                min_dur = elapsed;
            }
        }
    }
    min_dur
}

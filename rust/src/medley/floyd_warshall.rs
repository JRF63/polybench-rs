use crate::ndarray::AllocUninit;
use crate::ndarray::Array2D;
use crate::util;
use std::time::{Duration, Instant};

const N: usize = 1024;

unsafe fn init_array(n: usize, path: &mut Array2D<N, N>) {
    for i in 0..n {
        for j in 0..n {
            *path.0.get_unchecked_mut(i).0.get_unchecked_mut(j) =
                ((i + 1) * (j + 1)) as f64 / n as f64;
        }
    }
}

unsafe fn kernel_floyd_warshall(n: usize, path: &mut Array2D<N, N>) {
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                *path.0.get_unchecked_mut(i).0.get_unchecked_mut(j) =
                    if *path.0.get_unchecked(i).0.get_unchecked(j)
                        < *path.0.get_unchecked(i).0.get_unchecked(k)
                            + *path.0.get_unchecked(k).0.get_unchecked(j)
                    {
                        *path.0.get_unchecked(i).0.get_unchecked(j)
                    } else {
                        *path.0.get_unchecked(i).0.get_unchecked(k)
                            + *path.0.get_unchecked(k).0.get_unchecked(j)
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

use crate::ndarray::AllocUninit;
use crate::ndarray::{Array1D, Array2D};
use crate::util;
use crate::NUM_SAMPLES;
use std::time::{Duration, Instant};

const M: usize = 1000;
const N: usize = 1000;

unsafe fn init_array(m: usize, n: usize, float_n: &mut f64, data: &mut Array2D<M, N>) {
    *float_n = 1.2;
    for i in 0..m {
        for j in 0..n {
            *data.0.get_unchecked_mut(i).0.get_unchecked_mut(j) = (i * j) as f64 / M as f64;
        }
    }
}

unsafe fn kernel_correlation(
    m: usize,
    n: usize,
    float_n: f64,
    data: &mut Array2D<M, N>,
    symmat: &mut Array2D<M, M>,
    mean: &mut Array1D<M>,
    stddev: &mut Array1D<M>,
) {
    let eps = 0.1;

    for j in 0..m {
        *mean.0.get_unchecked_mut(j) = 0.0;
        for i in 0..n {
            *mean.0.get_unchecked_mut(j) += *data.0.get_unchecked(i).0.get_unchecked(j);
        }
        *mean.0.get_unchecked_mut(j) /= float_n;
    }

    for j in 0..m {
        *stddev.0.get_unchecked_mut(j) = 0.0;
        for i in 0..n {
            *stddev.0.get_unchecked_mut(j) += (*data.0.get_unchecked(i).0.get_unchecked(j)
                - *mean.0.get_unchecked(j))
                * (*data.0.get_unchecked(i).0.get_unchecked(j) - *mean.0.get_unchecked(j));
            *stddev.0.get_unchecked_mut(j) /= float_n;
            *stddev.0.get_unchecked_mut(j) = stddev.0.get_unchecked_mut(j).sqrt();
            *stddev.0.get_unchecked_mut(j) = if *stddev.0.get_unchecked_mut(j) <= eps {
                1.0
            } else {
                *stddev.0.get_unchecked_mut(j)
            };
        }
    }

    for i in 0..n {
        for j in 0..m {
            *data.0.get_unchecked_mut(i).0.get_unchecked_mut(j) -= *mean.0.get_unchecked(j);
            *data.0.get_unchecked_mut(i).0.get_unchecked_mut(j) /=
                float_n.sqrt() * (*stddev.0.get_unchecked(j));
        }
    }

    for j1 in 0..(m - 1) {
        *symmat.0.get_unchecked_mut(j1).0.get_unchecked_mut(j1) = 1.0;
        for j2 in (j1 + 1)..m {
            *symmat.0.get_unchecked_mut(j1).0.get_unchecked_mut(j2) = 0.0;
            for i in 0..n {
                *symmat.0.get_unchecked_mut(j1).0.get_unchecked_mut(j2) +=
                    (*data.0.get_unchecked(i).0.get_unchecked(j1))
                        * (*data.0.get_unchecked(i).0.get_unchecked(j2));
                *symmat.0.get_unchecked_mut(j2).0.get_unchecked_mut(j1) =
                    *symmat.0.get_unchecked_mut(j1).0.get_unchecked_mut(j2);
            }
        }
    }
    *symmat.0.get_unchecked_mut(m - 1).0.get_unchecked_mut(m - 1) = 1.0
}

pub fn bench() -> Duration {
    let n = N;
    let m = M;

    let mut float_n = 0.0;
    let mut data = Array2D::uninit();
    let mut symmat = Array2D::uninit();
    let mut mean = Array1D::uninit();
    let mut stddev = Array1D::uninit();

    let mut min_dur = util::max_duration();
    for _ in 0..NUM_SAMPLES {
        unsafe {
            init_array(m, n, &mut float_n, &mut data);

            util::flush_llc_cache();

            let now = Instant::now();
            kernel_correlation(
                m,
                n,
                float_n,
                &mut data,
                &mut symmat,
                &mut mean,
                &mut stddev,
            );
            let elapsed = now.elapsed();

            util::black_box(&data);
            util::black_box(&symmat);
            util::black_box(&mean);
            util::black_box(&stddev);

            if elapsed < min_dur {
                min_dur = elapsed;
            }
        }
    }
    min_dur
}

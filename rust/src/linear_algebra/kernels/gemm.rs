use crate::ndarray::Array2D;
use crate::ndarray::AllocUninit;
use crate::NUM_SAMPLES;
use crate::util;
use std::time::{Instant, Duration};

const NI: usize = 1024;
const NJ: usize = 1024;
const NK: usize = 1024;

unsafe fn init_array(
    ni: usize,
    nj: usize,
    nk: usize,
    alpha: &mut f64,
    beta: &mut f64,
    c: &mut Array2D<NI, NJ>,
    a: &mut Array2D<NI, NK>,
    b: &mut Array2D<NK, NJ>,
) {
    *alpha = 32412.0;
    *beta = 2123.0;
    for i in 0..ni {
        for j in 0..nj {
            *c.0.get_unchecked_mut(i).0.get_unchecked_mut(j) = (i * j) as f64 / ni as f64;
        }
    }
    for i in 0..ni {
        for j in 0..nk {
            *a.0.get_unchecked_mut(i).0.get_unchecked_mut(j) = (i * j) as f64 / ni as f64;
        }
    }
    for i in 0..nk {
        for j in 0..nj {
            *b.0.get_unchecked_mut(i).0.get_unchecked_mut(j) = (i * j) as f64 / ni as f64;
        }
    }
}

unsafe fn kernel_gemm(
    ni: usize,
    nj: usize,
    nk: usize,
    alpha: f64,
    beta: f64,
    c: &mut Array2D<NI, NJ>,
    a: &Array2D<NI, NK>,
    b: &Array2D<NK, NJ>,
) {
    for i in 0..ni {
        for j in 0..nj {
            *c.0.get_unchecked_mut(i).0.get_unchecked_mut(j) *= beta;
            for k in 0..nk {
                *c.0.get_unchecked_mut(i).0.get_unchecked_mut(j) += alpha
                    * (*a.0.get_unchecked(i).0.get_unchecked(k))
                    * (*b.0.get_unchecked(k).0.get_unchecked(j));
            }
        }
    }
}

pub fn bench() -> Duration {
    let ni = NI;
    let nj = NJ;
    let nk = NK;

    let mut alpha = 0.0;
    let mut beta = 0.0;
    let mut c = Array2D::uninit();
    let mut a = Array2D::uninit();
    let mut b = Array2D::uninit();

    let mut min_dur = util::max_duration();
    for _ in 0..NUM_SAMPLES {
        unsafe {
            init_array(
                ni, nj, nk, &mut alpha, &mut beta, &mut c, &mut a, &mut b,
            );

            util::flush_llc_cache();

            let now = Instant::now();
            kernel_gemm(ni, nj, nk, alpha, beta, &mut c, &a, &b);
            let elapsed = now.elapsed();

            util::black_box(&c);

            if elapsed < min_dur {
                min_dur = elapsed;
            }
        }
    }
    min_dur
}
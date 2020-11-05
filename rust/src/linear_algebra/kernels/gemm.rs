use crate::ndarray::Array2D;
use crate::ndarray::AllocUninit;
use crate::{NI, NJ, NK, NUM_SAMPLES};
use crate::util;
use std::time::{Instant, Duration};

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

unsafe fn kernel_gemm2(
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
            c.0[i].0[j] *= beta;
            for k in 0..nk {
                c.0[i].0[j] += alpha * a.0[i].0[k] * b.0[k].0[j];
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
    let mut c: Box<Array2D<NI, NJ>> = Array2D::uninit();
    let mut a: Box<Array2D<NI, NK>> = Array2D::uninit();
    let mut b: Box<Array2D<NK, NJ>> = Array2D::uninit();

    let mut min_dur = util::max_duration();
    for _ in 0..NUM_SAMPLES {
        unsafe {
            init_array(
                ni, nj, nk, &mut alpha, &mut beta, &mut c, &mut a, &mut b,
            );

            util::flush_llc_cache();

            let now = Instant::now();
            kernel_gemm2(ni, nj, nk, alpha, beta, &mut c, &a, &b);
            let elapsed = now.elapsed();

            util::black_box(&c);

            if elapsed < min_dur {
                min_dur = elapsed;
            }
        }
    }
    min_dur
}
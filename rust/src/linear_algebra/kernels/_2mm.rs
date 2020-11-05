use crate::ndarray::AllocUninit;
use crate::ndarray::Array2D;
use crate::util;
use crate::{NI, NJ, NK, NL, NUM_SAMPLES};
use std::time::{Duration, Instant};

unsafe fn init_array(
    ni: usize,
    nj: usize,
    nk: usize,
    nl: usize,
    alpha: &mut f64,
    beta: &mut f64,
    a: &mut Array2D<NI, NL>,
    b: &mut Array2D<NK, NJ>,
    c: &mut Array2D<NL, NJ>,
    d: &mut Array2D<NI, NL>,
) {
    *alpha = 32412.0;
    *beta = 2123.0;
    for i in 0..ni {
        for j in 0..nk {
            *a.0.get_unchecked_mut(i).0.get_unchecked_mut(j) = (i * j) as f64 / ni as f64;
        }
    }
    for i in 0..nk {
        for j in 0..nj {
            *b.0.get_unchecked_mut(i).0.get_unchecked_mut(j) = (i * (j + 1)) as f64 / ni as f64;
        }
    }
    for i in 0..nl {
        for j in 0..nj {
            *c.0.get_unchecked_mut(i).0.get_unchecked_mut(j) = (i * (j + 3)) as f64 / ni as f64;
        }
    }
    for i in 0..ni {
        for j in 0..nl {
            *d.0.get_unchecked_mut(i).0.get_unchecked_mut(j) = (i * (j + 2)) as f64 / ni as f64;
        }
    }
}

unsafe fn kernel_2mm(
    ni: usize,
    nj: usize,
    nk: usize,
    nl: usize,
    alpha: f64,
    beta: f64,
    tmp: &mut Array2D<NI, NJ>,
    a: &Array2D<NI, NK>,
    b: &Array2D<NK, NJ>,
    c: &Array2D<NL, NJ>,
    d: &mut Array2D<NI, NL>,
) {
    for i in 0..ni {
        for j in 0..nj {
            *tmp.0.get_unchecked_mut(i).0.get_unchecked_mut(j) = 0.0;
            for k in 0..nk {
                *tmp.0.get_unchecked_mut(i).0.get_unchecked_mut(j) += alpha
                    * (*a.0.get_unchecked(i).0.get_unchecked(k))
                    * (*b.0.get_unchecked(k).0.get_unchecked(j))
            }
        }
    }
    for i in 0..ni {
        for j in 0..nl {
            *d.0.get_unchecked_mut(i).0.get_unchecked_mut(j) *= beta;
            for k in 0..nj {
                *d.0.get_unchecked_mut(i).0.get_unchecked_mut(j) +=
                    (*tmp.0.get_unchecked(i).0.get_unchecked(k))
                        * (*c.0.get_unchecked(k).0.get_unchecked(j))
            }
        }
    }
}

pub fn bench() -> Duration {
    let ni = NI;
    let nj = NJ;
    let nk = NK;
    let nl = NL;

    let mut alpha = 0.0;
    let mut beta = 0.0;
    let mut tmp = Array2D::uninit();
    let mut a = Array2D::uninit();
    let mut b = Array2D::uninit();
    let mut c = Array2D::uninit();
    let mut d = Array2D::uninit();

    let mut min_dur = util::max_duration();
    for _ in 0..NUM_SAMPLES {
        unsafe {
            init_array(
                ni, nj, nk, nl, &mut alpha, &mut beta, &mut a, &mut b, &mut c, &mut d,
            );

            util::flush_llc_cache();

            let now = Instant::now();
            kernel_2mm(ni, nj, nk, nl, alpha, beta, &mut tmp, &a, &b, &c, &mut d);
            let elapsed = now.elapsed();

            util::black_box(&d);

            if elapsed < min_dur {
                min_dur = elapsed;
            }
        }
    }
    min_dur
}

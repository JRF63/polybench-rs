use crate::ndarray::AllocUninit;
use crate::ndarray::Array2D;
use crate::util;
use std::time::{Duration, Instant};

const NI: usize = 800;
const NJ: usize = 900;
const NK: usize = 1100;
const NL: usize = 1200;

unsafe fn init_array(
    ni: usize,
    nj: usize,
    nk: usize,
    nl: usize,
    alpha: &mut f64,
    beta: &mut f64,
    a: &mut Array2D<NI, NK>,
    b: &mut Array2D<NK, NJ>,
    c: &mut Array2D<NJ, NL>,
    d: &mut Array2D<NI, NL>,
) {
    *alpha = 1.5;
    *beta = 1.2;
    for i in 0..ni {
        for j in 0..nk {
            a[(i, j)] = ((i * j + 1) % ni) as f64 / ni as f64;
        }
    }
    for i in 0..nk {
        for j in 0..nj {
            b[(i, j)] = (i * (j + 1) % nj) as f64 / nj as f64;
        }
    }
    for i in 0..nj {
        for j in 0..nl {
            c[(i, j)] = ((i * (j + 3) + 1) % nl) as f64 / nl as f64;
        }
    }
    for i in 0..ni {
        for j in 0..nl {
            d[(i, j)] = (i * (j + 2) % nk) as f64 / nk as f64;
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
    c: &Array2D<NJ, NL>,
    d: &mut Array2D<NI, NL>,
) {
    for i in 0..ni {
        for j in 0..nj {
            tmp[(i, j)] = 0.0;
            for k in 0..nk {
                tmp[(i, j)] += alpha * a[(i, k)] * b[(k, j)];
            }
        }
    }
    for i in 0..ni {
        for j in 0..nl {
            d[(i, j)] *= beta;
            for k in 0..nj {
                d[(i, j)] += tmp[(i, k)] * c[(k, j)];
            }
        }
    }
}

pub fn bench(num_runs: usize) -> Duration {
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
    for _ in 0..num_runs {
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

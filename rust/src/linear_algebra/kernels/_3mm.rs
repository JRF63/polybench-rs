use crate::ndarray::AllocUninit;
use crate::ndarray::Array2D;
use crate::util;
use std::time::{Duration, Instant};

const NI: usize = 800;
const NJ: usize = 900;
const NK: usize = 1000;
const NL: usize = 1100;
const NM: usize = 1200;

unsafe fn init_array(
    ni: usize,
    nj: usize,
    nk: usize,
    nl: usize,
    nm: usize,
    a: &mut Array2D<NI, NK>,
    b: &mut Array2D<NK, NJ>,
    c: &mut Array2D<NJ, NM>,
    d: &mut Array2D<NM, NL>,
) {
    for i in 0..ni {
        for j in 0..nk {
            a[(i, j)] = ((i * j + 1) % ni) as f64 / (5 * ni) as f64;
        }
    }
    for i in 0..nk {
        for j in 0..nj {
            b[(i, j)] = ((i * (j + 1) + 2) % nj) as f64 / (5 * nj) as f64;
        }
    }
    for i in 0..nj {
        for j in 0..nm {
            c[(i, j)] = (i * (j + 3) % nl) as f64 / (5 * nl) as f64;
        }
    }
    for i in 0..nm {
        for j in 0..nl {
            d[(i, j)] = ((i * (j + 2) + 2) % nk) as f64 / (5 * nk) as f64;
        }
    }
}

unsafe fn kernel_3mm(
    ni: usize,
    nj: usize,
    nk: usize,
    nl: usize,
    nm: usize,
    e: &mut Array2D<NI, NJ>,
    a: &Array2D<NI, NK>,
    b: &Array2D<NK, NJ>,
    f: &mut Array2D<NJ, NL>,
    c: &Array2D<NJ, NM>,
    d: &Array2D<NM, NL>,
    g: &mut Array2D<NI, NL>,
) {
    for i in 0..ni {
        for j in 0..nj {
            e[(i, j)] = 0.0;
            for k in 0..nk {
                e[(i, j)] += a[(i, k)] * b[(k, j)];
            }
        }
    }
    for i in 0..nj {
        for j in 0..nl {
            f[(i, j)] = 0.0;
            for k in 0..nm {
                f[(i, j)] += c[(i, k)] * d[(k, j)];
            }
        }
    }
    for i in 0..ni {
        for j in 0..nl {
            g[(i, j)] = 0.0;
            for k in 0..nj {
                g[(i, j)] += e[(i, k)] * f[(k, j)];
            }
        }
    }
}

pub fn bench(num_runs: usize) -> Duration {
    let ni = NI;
    let nj = NJ;
    let nk = NK;
    let nl = NL;
    let nm = NM;

    let mut e = Array2D::uninit();
    let mut a = Array2D::uninit();
    let mut b = Array2D::uninit();
    let mut f = Array2D::uninit();
    let mut c = Array2D::uninit();
    let mut d = Array2D::uninit();
    let mut g = Array2D::uninit();

    let mut min_dur = util::max_duration();
    for _ in 0..num_runs {
        unsafe {
            init_array(ni, nj, nk, nl, nm, &mut a, &mut b, &mut c, &mut d);

            util::flush_llc_cache();

            let now = Instant::now();
            kernel_3mm(ni, nj, nk, nl, nm, &mut e, &a, &b, &mut f, &c, &d, &mut g);
            let elapsed = now.elapsed();

            util::black_box(&g);

            if elapsed < min_dur {
                min_dur = elapsed;
            }
        }
    }
    min_dur
}

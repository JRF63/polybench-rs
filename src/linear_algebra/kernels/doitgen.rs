use crate::ndarray::AllocUninit;
use crate::ndarray::{Array1D, Array2D, Array3D};
use crate::util;
use std::time::{Duration, Instant};

const NP: usize = 160;
const NQ: usize = 140;
const NR: usize = 150;

unsafe fn init_array(
    nr: usize,
    nq: usize,
    np: usize,
    a: &mut Array3D<NR, NQ, NP>,
    c4: &mut Array2D<NP, NP>,
) {
    for i in 0..nr {
        for j in 0..nq {
            for k in 0..np {
                a[(i, j, k)] = ((i * j + k) % np) as f64 / np as f64;
            }
        }
    }
    for i in 0..np {
        for j in 0..np {
            c4[(i, j)] = (i * j % np) as f64 / np as f64;
        }
    }
}

unsafe fn kernel_doitgen(
    nr: usize,
    nq: usize,
    np: usize,
    a: &mut Array3D<NR, NQ, NP>,
    c4: &Array2D<NP, NP>,
    sum: &mut Array1D<NP>,
) {
    for r in 0..nr {
        for q in 0..nq {
            for p in 0..np {
                sum[p] = 0.0;
                for s in 0..np {
                    sum[p] += a[(r, q, s)] * c4[(s, p)];
                }
            }
            for p in 0..np {
                a[(r, q, p)] = sum[p];
            }
        }
    }
}

pub fn bench(num_runs: usize) -> Duration {
    let nr = NR;
    let nq = NQ;
    let np = NP;

    let mut a = Array3D::uninit();
    let mut sum = Array1D::uninit();
    let mut c4 = Array2D::uninit();

    let mut min_dur = util::max_duration();
    for _ in 0..num_runs {
        unsafe {
            init_array(nr, nq, np, &mut a, &mut c4);

            util::flush_llc_cache();

            let now = Instant::now();
            kernel_doitgen(nr, nq, np, &mut a, &c4, &mut sum);
            let elapsed = now.elapsed();

            util::black_box(&a);

            if elapsed < min_dur {
                min_dur = elapsed;
            }
        }
    }
    min_dur
}

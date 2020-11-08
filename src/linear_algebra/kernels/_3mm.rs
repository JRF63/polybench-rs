#![allow(non_snake_case)]

use crate::config::linear_algebra::kernels::_3mm::{DataType, NI, NJ, NK, NL, NM};
use crate::ndarray::{Array2D, ArrayAlloc};
use crate::util;
use std::time::Duration;

unsafe fn init_array(
    ni: usize,
    nj: usize,
    nk: usize,
    nl: usize,
    nm: usize,
    A: &mut Array2D<DataType, NI, NK>,
    B: &mut Array2D<DataType, NK, NJ>,
    C: &mut Array2D<DataType, NJ, NM>,
    D: &mut Array2D<DataType, NM, NL>,
) {
    for i in 0..ni {
        for j in 0..nk {
            A[i][j] = ((i * j + 1) % ni) as DataType / (5 * ni) as DataType;
        }
    }
    for i in 0..nk {
        for j in 0..nj {
            B[i][j] = ((i * (j + 1) + 2) % nj) as DataType / (5 * nj) as DataType;
        }
    }
    for i in 0..nj {
        for j in 0..nm {
            C[i][j] = (i * (j + 3) % nl) as DataType / (5 * nl) as DataType;
        }
    }
    for i in 0..nm {
        for j in 0..nl {
            D[i][j] = ((i * (j + 2) + 2) % nk) as DataType / (5 * nk) as DataType;
        }
    }
}

unsafe fn kernel_3mm(
    ni: usize,
    nj: usize,
    nk: usize,
    nl: usize,
    nm: usize,
    E: &mut Array2D<DataType, NI, NJ>,
    A: &Array2D<DataType, NI, NK>,
    B: &Array2D<DataType, NK, NJ>,
    F: &mut Array2D<DataType, NJ, NL>,
    C: &Array2D<DataType, NJ, NM>,
    D: &Array2D<DataType, NM, NL>,
    G: &mut Array2D<DataType, NI, NL>,
) {
    for i in 0..ni {
        for j in 0..nj {
            E[i][j] = 0.0;
            for k in 0..nk {
                E[i][j] += A[i][k] * B[k][j];
            }
        }
    }
    for i in 0..nj {
        for j in 0..nl {
            F[i][j] = 0.0;
            for k in 0..nm {
                F[i][j] += C[i][k] * D[k][j];
            }
        }
    }
    for i in 0..ni {
        for j in 0..nl {
            G[i][j] = 0.0;
            for k in 0..nj {
                G[i][j] += E[i][k] * F[k][j];
            }
        }
    }
}

pub fn bench() -> Duration {
    let ni = NI;
    let nj = NJ;
    let nk = NK;
    let nl = NL;
    let nm = NM;

    let mut E = Array2D::uninit();
    let mut A = Array2D::uninit();
    let mut B = Array2D::uninit();
    let mut F = Array2D::uninit();
    let mut C = Array2D::uninit();
    let mut D = Array2D::uninit();
    let mut G = Array2D::uninit();

    unsafe {
        init_array(ni, nj, nk, nl, nm, &mut A, &mut B, &mut C, &mut D);
        let elapsed = util::time_function(|| {
            kernel_3mm(ni, nj, nk, nl, nm, &mut E, &A, &B, &mut F, &C, &D, &mut G)
        });
        util::black_box(&G);
        elapsed
    }
}

#[test]
fn check() {
    bench();
}

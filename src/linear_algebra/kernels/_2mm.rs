#![allow(non_snake_case)]

use crate::config::linear_algebra::kernels::_2mm::{DataType, NI, NJ, NK, NL};
use crate::ndarray2::{Array2D, ArrayAlloc};
use crate::util;
use std::time::Duration;

unsafe fn init_array(
    ni: usize,
    nj: usize,
    nk: usize,
    nl: usize,
    alpha: &mut DataType,
    beta: &mut DataType,
    A: &mut Array2D<DataType, NI, NK>,
    B: &mut Array2D<DataType, NK, NJ>,
    C: &mut Array2D<DataType, NJ, NL>,
    D: &mut Array2D<DataType, NI, NL>,
) {
    *alpha = 1.5;
    *beta = 1.2;
    for i in 0..ni {
        for j in 0..nk {
            A[i][j] = ((i * j + 1) % ni) as DataType / ni as DataType;
        }
    }
    for i in 0..nk {
        for j in 0..nj {
            B[i][j] = (i * (j + 1) % nj) as DataType / nj as DataType;
        }
    }
    for i in 0..nj {
        for j in 0..nl {
            C[i][j] = ((i * (j + 3) + 1) % nl) as DataType / nl as DataType;
        }
    }
    for i in 0..ni {
        for j in 0..nl {
            D[i][j] = (i * (j + 2) % nk) as DataType / nk as DataType;
        }
    }
}

unsafe fn kernel_2mm(
    ni: usize,
    nj: usize,
    nk: usize,
    nl: usize,
    alpha: DataType,
    beta: DataType,
    tmp: &mut Array2D<DataType, NI, NJ>,
    A: &Array2D<DataType, NI, NK>,
    B: &Array2D<DataType, NK, NJ>,
    C: &Array2D<DataType, NJ, NL>,
    D: &mut Array2D<DataType, NI, NL>,
) {
    for i in 0..ni {
        for j in 0..nj {
            tmp[i][j] = 0.0;
            for k in 0..nk {
                tmp[i][j] += alpha * A[i][k] * B[k][j];
            }
        }
    }
    for i in 0..ni {
        for j in 0..nl {
            D[i][j] *= beta;
            for k in 0..nj {
                D[i][j] += tmp[i][k] * C[k][j];
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
    let mut A = Array2D::uninit();
    let mut B = Array2D::uninit();
    let mut C = Array2D::uninit();
    let mut D = Array2D::uninit();

    unsafe {
        init_array(
            ni, nj, nk, nl, &mut alpha, &mut beta, &mut A, &mut B, &mut C, &mut D,
        );
        let elapsed = util::time_function(|| {
            kernel_2mm(ni, nj, nk, nl, alpha, beta, &mut tmp, &A, &B, &C, &mut D)
        });
        util::black_box(&D);
        elapsed
    }
}

#[test]
fn check() {
    bench();
}

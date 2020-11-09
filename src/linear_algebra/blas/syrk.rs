#![allow(non_snake_case)]

use crate::config::linear_algebra::blas::syrk::{DataType, M, N};
use crate::ndarray::{Array2D, ArrayAlloc};
use crate::util;
use std::time::Duration;

unsafe fn init_array(
    m: usize,
    n: usize,
    alpha: &mut DataType,
    beta: &mut DataType,
    C: &mut Array2D<DataType, M, M>,
    A: &mut Array2D<DataType, M, N>,
) {
    *alpha = 1.5;
    *beta = 1.2;
    for i in 0..m {
        for j in 0..n {
            A[i][j] = ((i * j + 1) % m) as DataType / m as DataType;
        }
    }

    for i in 0..m {
        for j in 0..m {
            C[i][j] = ((i * j + 2) % n) as DataType / n as DataType;
        }
    }
}

unsafe fn kernel_syrk(
    m: usize,
    n: usize,
    alpha: DataType,
    beta: DataType,
    C: &mut Array2D<DataType, M, M>,
    A: &Array2D<DataType, M, N>,
) {
    for i in 0..m {
        for j in 0..=i {
            C[i][j] *= beta;
        }
        for k in 0..n {
            for j in 0..=i {
                C[i][j] += alpha * A[i][k] * A[j][k];
            }
        }
    }
}

pub fn bench() -> Duration {
    let m = M;
    let n = N;

    let mut alpha = 0.0;
    let mut beta = 0.0;
    let mut C = Array2D::uninit();
    let mut A = Array2D::uninit();

    unsafe {
        init_array(m, n, &mut alpha, &mut beta, &mut C, &mut A);
        let elapsed = util::time_function(|| kernel_syrk(m, n, alpha, beta, &mut C, &A));
        util::consume(C);
        elapsed
    }
}

#[test]
fn check() {
    bench();
}

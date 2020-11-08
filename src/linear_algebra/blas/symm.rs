#![allow(non_snake_case)]

use crate::config::linear_algebra::blas::symm::{DataType, M, N};
use crate::ndarray::{Array2D, ArrayAlloc};
use crate::util;
use std::time::Duration;

unsafe fn init_array(
    m: usize,
    n: usize,
    alpha: &mut DataType,
    beta: &mut DataType,
    C: &mut Array2D<DataType, M, N>,
    A: &mut Array2D<DataType, M, M>,
    B: &mut Array2D<DataType, M, N>,
) {
    *alpha = 1.5;
    *beta = 1.2;
    for i in 0..m {
        for j in 0..n {
            C[i][j] = ((i + j) % 100) as DataType / m as DataType;
            B[i][j] = ((n + i - j) % 100) as DataType / m as DataType;
        }
    }

    for i in 0..m {
        for j in 0..=i {
            A[i][j] = ((i + j) % 100) as DataType / m as DataType;
        }
        for j in (i + 1)..m {
            A[i][j] = -999 as DataType;
        }
    }
}

unsafe fn kernel_symm(
    m: usize,
    n: usize,
    alpha: DataType,
    beta: DataType,
    C: &mut Array2D<DataType, M, N>,
    A: &Array2D<DataType, M, M>,
    B: &Array2D<DataType, M, N>,
) {
    for i in 0..m {
        for j in 0..n {
            let mut temp2 = 0.0;
            for k in 0..i {
                C[k][j] += alpha * B[i][j] * A[i][k];
                temp2 += B[k][j] * A[i][k];
            }
            C[i][j] = beta * C[i][j] + alpha * B[i][j] * A[i][i] + alpha * temp2;
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
    let mut B = Array2D::uninit();

    unsafe {
        init_array(m, n, &mut alpha, &mut beta, &mut C, &mut A, &mut B);
        let elapsed = util::time_function(|| kernel_symm(m, n, alpha, beta, &mut C, &A, &B));
        util::consume(C);
        elapsed
    }
}

#[test]
fn check() {
    bench();
}

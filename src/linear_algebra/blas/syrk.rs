#![allow(non_snake_case)]

use crate::config::linear_algebra::blas::syrk::{DataType, M, N};
use crate::ndarray2::{Array2D, ArrayAlloc};
use crate::util;
use std::time::Duration;

unsafe fn init_array(
    n: usize,
    m: usize,
    alpha: &mut DataType,
    beta: &mut DataType,
    C: &mut Array2D<DataType, N, N>,
    A: &mut Array2D<DataType, N, M>,
) {
    *alpha = 1.5;
    *beta = 1.2;
    for i in 0..n {
        for j in 0..m {
            A[i][j] = ((i * j + 1) % n) as DataType / n as DataType;
        }
    }

    for i in 0..n {
        for j in 0..n {
            C[i][j] = ((i * j + 2) % m) as DataType / m as DataType;
        }
    }
}

unsafe fn kernel_syrk(
    n: usize,
    m: usize,
    alpha: DataType,
    beta: DataType,
    C: &mut Array2D<DataType, N, N>,
    A: &Array2D<DataType, N, M>,
) {
    for i in 0..n {
        for j in 0..=i {
            C[i][j] *= beta;
        }
        for k in 0..m {
            for j in 0..=i {
                C[i][j] += alpha * A[i][k] * A[j][k];
            }
        }
    }
}

pub fn bench() -> Duration {
    let n = N;
    let m = M;
    
    let mut alpha = 0.0;
    let mut beta = 0.0;
    let mut C = Array2D::uninit();
    let mut A = Array2D::uninit();

    unsafe {
        init_array(n, m, &mut alpha, &mut beta, &mut C, &mut A);
        let elapsed = util::time_function(|| kernel_syrk(n, m, alpha, beta, &mut C, &A));
        util::black_box(&C);
        elapsed
    }
}

#[test]
fn check() {
    bench();
}

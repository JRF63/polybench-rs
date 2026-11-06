#![allow(non_snake_case)]

use crate::config::linear_algebra::blas::syrk::DataType;
use crate::ndarray::{Array2D, ArrayAlloc};
use crate::util;
use std::time::Duration;

unsafe fn init_array<const M: usize, const N: usize>(
    m: usize,
    n: usize,
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

unsafe fn kernel_syrk<const M: usize, const N: usize>(
    m: usize,
    n: usize,
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

pub fn bench<const M: usize, const N: usize>() -> Duration {
    let n = N;
    let m = M;

    let mut alpha = 0.0;
    let mut beta = 0.0;
    let mut C = Array2D::<DataType, N, N>::uninit();
    let mut A = Array2D::<DataType, N, M>::uninit();

    unsafe {
        init_array(m, n, &mut alpha, &mut beta, &mut C, &mut A);
        let elapsed = util::time_function(|| kernel_syrk(m, n, alpha, beta, &mut C, &A));
        util::consume(C);
        elapsed
    }
}

#[test]
fn check() {
    bench::<10, 12>();
}

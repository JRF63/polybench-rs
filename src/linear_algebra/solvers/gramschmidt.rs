#![allow(non_snake_case)]

use crate::config::linear_algebra::solvers::gramschmidt::DataType;
use crate::ndarray::{Array2D, ArrayAlloc};
use crate::util;
use std::time::Duration;

unsafe fn init_array<const M: usize, const N: usize>(
    m: usize,
    n: usize,
    A: &mut Array2D<DataType, M, N>,
    R: &mut Array2D<DataType, N, N>,
    Q: &mut Array2D<DataType, M, N>,
) {
    for i in 0..m {
        for j in 0..n {
            A[i][j] = ((((i * j) % m) as DataType / m as DataType) * 100.0) + 10.0;
            Q[i][j] = 0.0;
        }
    }
    for i in 0..n {
        for j in 0..n {
            R[i][j] = 0.0;
        }
    }
}

unsafe fn kernel_gramschmidt<const M: usize, const N: usize>(
    m: usize,
    n: usize,
    A: &mut Array2D<DataType, M, N>,
    R: &mut Array2D<DataType, N, N>,
    Q: &mut Array2D<DataType, M, N>,
) {
    for k in 0..n {
        let mut nrm = 0.0;
        for i in 0..m {
            nrm += A[i][k] * A[i][k];
        }
        R[k][k] = nrm.sqrt();
        for i in 0..m {
            Q[i][k] = A[i][k] / R[k][k];
        }
        for j in (k + 1)..n {
            R[k][j] = 0.0;
            for i in 0..m {
                R[k][j] += Q[i][k] * A[i][j];
            }
            for i in 0..m {
                A[i][j] = A[i][j] - Q[i][k] * R[k][j];
            }
        }
    }
}

pub fn bench<const M: usize, const N: usize>() -> Duration {
    let m = M;
    let n = N;

    let mut A = Array2D::<DataType, M, N>::uninit();
    let mut R = Array2D::<DataType, N, N>::uninit();
    let mut Q = Array2D::<DataType, M, N>::uninit();

    unsafe {
        init_array(m, n, &mut A, &mut R, &mut Q);
        let elapsed = util::time_function(|| kernel_gramschmidt(m, n, &mut A, &mut R, &mut Q));
        util::consume(A);
        util::consume(R);
        util::consume(Q);
        elapsed
    }
}

#[test]
fn check() {
    bench::<10, 12>();
}

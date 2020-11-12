#![allow(non_snake_case)]

use crate::config::stencils::jacobi_2d::DataType;
use crate::ndarray::{Array2D, ArrayAlloc};
use crate::util;
use std::time::Duration;

unsafe fn init_array<const N: usize, const TSTEPS: usize>(
    n: usize,
    A: &mut Array2D<DataType, N, N>,
    B: &mut Array2D<DataType, N, N>,
) {
    for i in 0..n {
        for j in 0..n {
            A[i][j] = (i * (j + 2) + 2) as DataType / n as DataType;
            B[i][j] = (i * (j + 3) + 3) as DataType / n as DataType;
        }
    }
}

unsafe fn kernel_jacobi_2d<const N: usize, const TSTEPS: usize>(
    tsteps: usize,
    n: usize,
    A: &mut Array2D<DataType, N, N>,
    B: &mut Array2D<DataType, N, N>,
) {
    for _ in 0..tsteps {
        for i in 1..(n - 1) {
            for j in 1..(n - 1) {
                B[i][j] = 0.2 * (A[i][j] + A[i][j - 1] + A[i][1 + j] + A[1 + i][j] + A[i - 1][j]);
            }
        }
        for i in 1..(n - 1) {
            for j in 1..(n - 1) {
                A[i][j] = 0.2 * (B[i][j] + B[i][j - 1] + B[i][1 + j] + B[1 + i][j] + B[i - 1][j]);
            }
        }
    }
}

pub fn bench<const N: usize, const TSTEPS: usize>() -> Duration {
    let n = N;
    let tsteps = TSTEPS;

    let mut A = Array2D::<DataType, N, N>::uninit();
    let mut B = Array2D::<DataType, N, N>::uninit();

    unsafe {
        init_array::<N, TSTEPS>(n, &mut A, &mut B);
        let elapsed =
            util::time_function(|| kernel_jacobi_2d::<N, TSTEPS>(tsteps, n, &mut A, &mut B));
        util::consume(A);
        elapsed
    }
}

#[test]
fn check() {
    bench::<13, 5>();
}

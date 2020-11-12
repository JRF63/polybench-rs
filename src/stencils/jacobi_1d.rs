#![allow(non_snake_case)]

use crate::config::stencils::jacobi_1d::DataType;
use crate::ndarray::{Array1D, ArrayAlloc};
use crate::util;
use std::time::Duration;

unsafe fn init_array<const N: usize, const TSTEPS: usize>(
    n: usize,
    A: &mut Array1D<DataType, N>,
    B: &mut Array1D<DataType, N>,
) {
    for i in 0..n {
        A[i] = (i + 2) as DataType / n as DataType;
        B[i] = (i + 3) as DataType / n as DataType;
    }
}

unsafe fn kernel_jacobi_1d<const N: usize, const TSTEPS: usize>(
    tsteps: usize,
    n: usize,
    A: &mut Array1D<DataType, N>,
    B: &mut Array1D<DataType, N>,
) {
    for _ in 0..tsteps {
        for i in 1..(n - 1) {
            B[i] = 0.33333 * (A[i - 1] + A[i] + A[i + 1]);
        }
        for i in 1..(n - 1) {
            A[i] = 0.33333 * (B[i - 1] + B[i] + B[i + 1]);
        }
    }
}

pub fn bench<const N: usize, const TSTEPS: usize>() -> Duration {
    let n = N;
    let tsteps = TSTEPS;

    let mut A = Array1D::<DataType, N>::uninit();
    let mut B = Array1D::<DataType, N>::uninit();

    unsafe {
        init_array::<N, TSTEPS>(n, &mut A, &mut B);
        let elapsed =
            util::time_function(|| kernel_jacobi_1d::<N, TSTEPS>(tsteps, n, &mut A, &mut B));
        util::consume(A);
        elapsed
    }
}

#[test]
fn check() {
    bench::<20, 5>();
}

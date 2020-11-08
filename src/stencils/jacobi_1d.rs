#![allow(non_snake_case)]

use crate::config::stencils::jacobi_1d::{DataType, N, TSTEPS};
use crate::ndarray2::{Array1D, ArrayAlloc};
use crate::util;
use std::time::Duration;

unsafe fn init_array(n: usize, A: &mut Array1D<DataType, N>, B: &mut Array1D<DataType, N>) {
    for i in 0..n {
        A[i] = (i + 2) as DataType / n as DataType;
        B[i] = (i + 3) as DataType / n as DataType;
    }
}

unsafe fn kernel_jacobi_1d(
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

pub fn bench() -> Duration {
    let n = N;
    let tsteps = TSTEPS;

    let mut A = Array1D::uninit();
    let mut B = Array1D::uninit();

    unsafe {
        init_array(n, &mut A, &mut B);
        let elapsed = util::time_function(|| kernel_jacobi_1d(tsteps, n, &mut A, &mut B));
        util::black_box(&A);
        elapsed
    }
}

#[test]
fn check() {
    bench();
}

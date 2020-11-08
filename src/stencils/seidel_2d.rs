#![allow(non_snake_case)]

use crate::config::stencils::seidel_2d::{DataType, N, TSTEPS};
use crate::ndarray2::{Array2D, ArrayAlloc};
use crate::util;
use std::time::Duration;

unsafe fn init_array(n: usize, A: &mut Array2D<DataType, N, N>) {
    for i in 0..n {
        for j in 0..n {
            A[i][j] = (i * (j + 2) + 2) as DataType / n as DataType;
        }
    }
}

unsafe fn kernel_seidel_2d(tsteps: usize, n: usize, A: &mut Array2D<DataType, N, N>) {
    for _ in 0..tsteps {
        for i in 1..(n - 1) {
            for j in 1..(n - 1) {
                A[i][j] = (A[i - 1][j - 1]
                    + A[i - 1][j]
                    + A[i - 1][j + 1]
                    + A[i][j - 1]
                    + A[i][j]
                    + A[i][j + 1]
                    + A[i + 1][j - 1]
                    + A[i + 1][j]
                    + A[i + 1][j + 1])
                    / 9.0;
            }
        }
    }
}

pub fn bench() -> Duration {
    let n = N;
    let tsteps = TSTEPS;

    let mut A = Array2D::uninit();

    unsafe {
        init_array(n, &mut A);
        let elapsed = util::time_function(|| kernel_seidel_2d(tsteps, n, &mut A));
        util::black_box(&A);
        elapsed
    }
}

#[test]
fn check() {
    bench();
}

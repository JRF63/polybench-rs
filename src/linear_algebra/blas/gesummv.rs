#![allow(non_snake_case)]

use crate::config::linear_algebra::blas::gesummv::{DataType, N};
use crate::ndarray::{Array1D, Array2D, ArrayAlloc};
use crate::util;
use std::time::Duration;

unsafe fn init_array(
    n: usize,
    alpha: &mut DataType,
    beta: &mut DataType,
    A: &mut Array2D<DataType, N, N>,
    B: &mut Array2D<DataType, N, N>,
    x: &mut Array1D<DataType, N>,
) {
    *alpha = 1.5;
    *beta = 1.2;
    for i in 0..n {
        x[i] = (i % n) as DataType / n as DataType;
        for j in 0..n {
            A[i][j] = ((i * j + 1) % n) as DataType / n as DataType;
            B[i][j] = ((i * j + 2) % n) as DataType / n as DataType;
        }
    }
}

unsafe fn kernel_gesummv(
    n: usize,
    alpha: DataType,
    beta: DataType,
    A: &Array2D<DataType, N, N>,
    B: &Array2D<DataType, N, N>,
    tmp: &mut Array1D<DataType, N>,
    x: &Array1D<DataType, N>,
    y: &mut Array1D<DataType, N>,
) {
    for i in 0..n {
        tmp[i] = 0.0;
        y[i] = 0.0;
        for j in 0..n {
            tmp[i] = A[i][j] * x[j] + tmp[i];
            y[i] = B[i][j] * x[j] + y[i];
        }
        y[i] = alpha * tmp[i] + beta * y[i];
    }
}

pub fn bench() -> Duration {
    let n = N;

    let mut alpha = 0.0;
    let mut beta = 0.0;
    let mut A = Array2D::uninit();
    let mut B = Array2D::uninit();
    let mut tmp = Array1D::uninit();
    let mut x = Array1D::uninit();
    let mut y = Array1D::uninit();

    unsafe {
        init_array(n, &mut alpha, &mut beta, &mut A, &mut B, &mut x);
        let elapsed =
            util::time_function(|| kernel_gesummv(n, alpha, beta, &A, &B, &mut tmp, &x, &mut y));
        util::black_box(&y);
        elapsed
    }
}

#[test]
fn check() {
    bench();
}

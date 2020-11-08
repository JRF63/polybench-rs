#![allow(non_snake_case)]

use crate::config::linear_algebra::solvers::trisolv::{DataType, N};
use crate::ndarray::{Array1D, Array2D, ArrayAlloc};
use crate::util;
use std::time::Duration;

unsafe fn init_array(
    n: usize,
    L: &mut Array2D<DataType, N, N>,
    x: &mut Array1D<DataType, N>,
    b: &mut Array1D<DataType, N>,
) {
    for i in 0..n {
        x[i] = -999.0;
        b[i] = i as DataType;
        for j in 0..=i {
            L[i][j] = (i + n - j + 1) as DataType * 2.0 / n as DataType;
        }
    }
}

unsafe fn kernel_trisolv(
    n: usize,
    L: &Array2D<DataType, N, N>,
    x: &mut Array1D<DataType, N>,
    b: &Array1D<DataType, N>,
) {
    for i in 0..n {
        x[i] = b[i];
        for j in 0..i {
            x[i] -= L[i][j] * x[j];
        }
        x[i] = x[i] / L[i][i];
    }
}

pub fn bench() -> Duration {
    let n = N;

    let mut L = Array2D::uninit();
    let mut x = Array1D::uninit();
    let mut b = Array1D::uninit();

    unsafe {
        init_array(n, &mut L, &mut x, &mut b);
        let elapsed = util::time_function(|| kernel_trisolv(n, &L, &mut x, &b));
        util::black_box(&x);
        elapsed
    }
}

#[test]
fn check() {
    bench();
}

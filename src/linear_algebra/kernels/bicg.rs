#![allow(non_snake_case)]

use crate::config::linear_algebra::kernels::bicg::{DataType, M, N};
use crate::ndarray::{Array1D, Array2D, ArrayAlloc};
use crate::util;
use std::time::Duration;

unsafe fn init_array(
    m: usize,
    n: usize,
    A: &mut Array2D<DataType, M, N>,
    r: &mut Array1D<DataType, M>,
    p: &mut Array1D<DataType, N>,
) {
    for i in 0..n {
        p[i] = (i % n) as DataType / n as DataType;
    }
    for i in 0..m {
        r[i] = (i % m) as DataType / m as DataType;
        for j in 0..n {
            A[i][j] = (i * (j + 1) % m) as DataType / m as DataType;
        }
    }
}

unsafe fn kernel_bicg(
    m: usize,
    n: usize,
    A: &Array2D<DataType, M, N>,
    s: &mut Array1D<DataType, N>,
    q: &mut Array1D<DataType, M>,
    p: &Array1D<DataType, N>,
    r: &Array1D<DataType, M>,
) {
    for i in 0..n {
        s[i] = 0.0;
    }
    for i in 0..m {
        q[i] = 0.0;
        for j in 0..n {
            s[j] = s[j] + r[i] * A[i][j];
            q[i] = q[i] + A[i][j] * p[j];
        }
    }
}

pub fn bench() -> Duration {
    let m = M;
    let n = N;

    let mut A = Array2D::uninit();
    let mut s = Array1D::uninit();
    let mut q = Array1D::uninit();
    let mut p = Array1D::uninit();
    let mut r = Array1D::uninit();

    unsafe {
        init_array(m, n, &mut A, &mut r, &mut p);
        let elapsed = util::time_function(|| kernel_bicg(m, n, &A, &mut s, &mut q, &p, &r));
        util::consume(s);
        util::consume(q);
        elapsed
    }
}

#[test]
fn check() {
    bench();
}

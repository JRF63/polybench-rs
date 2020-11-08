#![allow(non_snake_case)]

use crate::config::linear_algebra::kernels::bicg::{DataType, M, N};
use crate::ndarray2::{Array1D, Array2D, ArrayAlloc};
use crate::util;
use std::time::Duration;

unsafe fn init_array(
    m: usize,
    n: usize,
    A: &mut Array2D<DataType, N, M>,
    r: &mut Array1D<DataType, N>,
    p: &mut Array1D<DataType, M>,
) {
    for i in 0..m {
        p[i] = (i % m) as DataType / m as DataType;
    }
    for i in 0..n {
        r[i] = (i % n) as DataType / n as DataType;
        for j in 0..m {
            A[i][j] = (i * (j + 1) % n) as DataType / n as DataType;
        }
    }
}

unsafe fn kernel_bicg(
    m: usize,
    n: usize,
    A: &Array2D<DataType, N, M>,
    s: &mut Array1D<DataType, M>,
    q: &mut Array1D<DataType, N>,
    p: &Array1D<DataType, M>,
    r: &Array1D<DataType, N>,
) {
    for i in 0..m {
        s[i] = 0.0;
    }
    for i in 0..n {
        q[i] = 0.0;
        for j in 0..m {
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
        util::black_box(&s);
        util::black_box(&q);
        elapsed
    }
}

#[test]
fn check() {
    bench();
}

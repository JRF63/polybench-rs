#![allow(non_snake_case)]

use crate::config::linear_algebra::solvers::lu::{DataType, N};
use crate::ndarray::{Array2D, ArrayAlloc};
use crate::util;
use std::time::Duration;

unsafe fn init_array(n: usize, A: &mut Array2D<DataType, N, N>) {
    for i in 0..n {
        for j in 0..=i {
            A[i][j] = (-(j as isize) % n as isize) as DataType / n as DataType + 1.0;
        }
        for j in (i + 1)..n {
            A[i][j] = 0.0;
        }
        A[i][i] = 1.0;
    }

    A.make_positive_semi_definite();
}

unsafe fn kernel_lu(n: usize, A: &mut Array2D<DataType, N, N>) {
    for i in 0..n {
        for j in 0..i {
            for k in 0..j {
                A[i][j] -= A[i][k] * A[k][j];
            }
            A[i][j] /= A[j][j];
        }
        for j in i..n {
            for k in 0..i {
                A[i][j] -= A[i][k] * A[k][j];
            }
        }
    }
}

pub fn bench() -> Duration {
    let n = N;

    let mut A = Array2D::uninit();

    unsafe {
        init_array(n, &mut A);
        let elapsed = util::time_function(|| kernel_lu(n, &mut A));
        util::black_box(&A);
        elapsed
    }
}

#[test]
fn check() {
    bench();
}

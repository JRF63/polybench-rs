#![allow(non_snake_case)]

use crate::config::linear_algebra::solvers::cholesky::{DataType, N};
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

unsafe fn kernel_cholesky(n: usize, A: &mut Array2D<DataType, N, N>) {
    for i in 0..n {
        for j in 0..i {
            for k in 0..j {
                A[i][j] -= A[i][k] * A[j][k];
            }
            A[i][j] /= A[j][j];
        }
        for k in 0..i {
            A[i][i] -= A[i][k] * A[i][k];
        }
        A[i][i] = A[i][i].sqrt();
    }
}

pub fn bench() -> Duration {
    let n = N;

    let mut A = Array2D::uninit();

    unsafe {
        init_array(n, &mut A);
        let elapsed = util::time_function(|| kernel_cholesky(n, &mut A));
        util::black_box(&A);
        elapsed
    }
}

#[test]
fn check() {
    bench();
}

use crate::config::datamining::covariance::{DataType, M, N};
use crate::ndarray::{Array1D, Array2D, ArrayAlloc};
use crate::util;
use std::time::Duration;

unsafe fn init_array(
    m: usize,
    n: usize,
    float_n: &mut DataType,
    data: &mut Array2D<DataType, M, N>,
) {
    *float_n = n as DataType;
    for i in 0..m {
        for j in 0..n {
            data[i][j] = (i * j) as DataType / N as DataType;
        }
    }
}

unsafe fn kernel_covariance(
    m: usize,
    n: usize,
    float_n: DataType,
    data: &mut Array2D<DataType, M, N>,
    cov: &mut Array2D<DataType, N, N>,
    mean: &mut Array1D<DataType, N>,
) {
    for j in 0..n {
        mean[j] = 0.0;
        for i in 0..m {
            mean[j] += data[i][j];
        }
        mean[j] /= float_n;
    }

    for i in 0..m {
        for j in 0..n {
            data[i][j] -= mean[j];
        }
    }

    for i in 0..n {
        for j in i..n {
            cov[i][j] = 0.0;
            for k in 0..m {
                cov[i][j] += data[k][i] * data[k][j];
            }
            cov[i][j] /= float_n - 1.0;
            cov[j][i] = cov[i][j];
        }
    }
}

pub fn bench() -> Duration {
    let m = M;
    let n = N;

    let mut float_n = 0.0;
    let mut data = Array2D::uninit();
    let mut cov = Array2D::uninit();
    let mut mean = Array1D::uninit();

    unsafe {
        init_array(m, n, &mut float_n, &mut data);
        let elapsed = util::time_function(|| {
            kernel_covariance(m, n, float_n, &mut data, &mut cov, &mut mean)
        });
        util::consume(cov);
        elapsed
    }
}

#[test]
fn check() {
    bench();
}

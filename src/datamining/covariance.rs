use crate::config::datamining::correlation::{DataType, M, N};
use crate::ndarray2::{Array1D, Array2D, ArrayAlloc};
use crate::util;
use std::time::Duration;

unsafe fn init_array(
    m: usize,
    n: usize,
    float_n: &mut DataType,
    data: &mut Array2D<DataType, N, M>,
) {
    *float_n = n as DataType;
    for i in 0..n {
        for j in 0..m {
            data[i][j] = (i * j) as DataType / M as DataType;
        }
    }
}

unsafe fn kernel_covariance(
    m: usize,
    n: usize,
    float_n: DataType,
    data: &mut Array2D<DataType, N, M>,
    cov: &mut Array2D<DataType, M, M>,
    mean: &mut Array1D<DataType, M>,
) {
    for j in 0..m {
        mean[j] = 0.0;
        for i in 0..n {
            mean[j] += data[i][j];
        }
        mean[j] /= float_n;
    }

    for i in 0..n {
        for j in 0..m {
            data[i][j] -= mean[j];
        }
    }

    for i in 0..m {
        for j in i..m {
            cov[i][j] = 0.0;
            for k in 0..n {
                cov[i][j] += data[k][i] * data[k][j];
            }
            cov[i][j] /= float_n - 1.0;
            cov[j][i] = cov[i][j];
        }
    }
}

pub fn bench() -> Duration {
    let n = N;
    let m = M;

    let mut float_n = 0.0;
    let mut data = Array2D::uninit();
    let mut cov = Array2D::uninit();
    let mut mean = Array1D::uninit();

    unsafe {
        init_array(m, n, &mut float_n, &mut data);
        let elapsed = util::time_function(|| {
            kernel_covariance(m, n, float_n, &mut data, &mut cov, &mut mean)
        });
        util::black_box(&cov);
        elapsed
    }
}

#[test]
fn check() {
    bench();
}

use crate::config::datamining::correlation::{DataType, M, N};
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
            data[i][j] = (i * j) as DataType / (N + i) as DataType;
        }
    }
}

unsafe fn kernel_correlation(
    m: usize,
    n: usize,
    float_n: DataType,
    data: &mut Array2D<DataType, M, N>,
    corr: &mut Array2D<DataType, N, N>,
    mean: &mut Array1D<DataType, N>,
    stddev: &mut Array1D<DataType, N>,
) {
    let eps = 0.1;

    for j in 0..n {
        mean[j] = 0.0;
        for i in 0..m {
            mean[j] += data[i][j];
        }
        mean[j] /= float_n;
    }

    for j in 0..n {
        stddev[j] = 0.0;
        for i in 0..m {
            stddev[j] += (data[i][j] - mean[j]) * (data[i][j] - mean[j]);
            stddev[j] /= float_n;
            stddev[j] = stddev[j].sqrt();
            stddev[j] = if stddev[j] <= eps { 1.0 } else { stddev[j] };
        }
    }

    for i in 0..m {
        for j in 0..n {
            data[i][j] -= mean[j];
            data[i][j] /= float_n.sqrt() * stddev[j];
        }
    }

    for i in 0..(n - 1) {
        corr[i][i] = 1.0;
        for j in (i + 1)..n {
            corr[i][j] = 0.0;
            for k in 0..m {
                corr[i][j] += data[k][i] * data[k][j];
            }
            corr[j][i] = corr[i][j];
        }
    }
    corr[n - 1][n - 1] = 1.0;
}

pub fn bench() -> Duration {
    let m = M;
    let n = N;

    let mut float_n = 0.0;
    let mut data = Array2D::uninit();
    let mut corr = Array2D::uninit();
    let mut mean = Array1D::uninit();
    let mut stddev = Array1D::uninit();

    unsafe {
        init_array(m, n, &mut float_n, &mut data);
        let elapsed = util::time_function(|| {
            kernel_correlation(m, n, float_n, &mut data, &mut corr, &mut mean, &mut stddev)
        });
        util::consume(corr);
        elapsed
    }
}

#[test]
fn check() {
    bench();
}

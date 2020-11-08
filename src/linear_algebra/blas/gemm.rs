use crate::config::linear_algebra::blas::gemm::{DataType, NI, NJ, NK};
use crate::ndarray2::{Array2D, ArrayAlloc};
use crate::util;
use std::time::Duration;

unsafe fn init_array(
    ni: usize,
    nj: usize,
    nk: usize,
    alpha: &mut DataType,
    beta: &mut DataType,
    c: &mut Array2D<DataType, NI, NJ>,
    a: &mut Array2D<DataType, NI, NK>,
    b: &mut Array2D<DataType, NK, NJ>,
) {
    *alpha = 1.5;
    *beta = 1.2;
    for i in 0..ni {
        for j in 0..nj {
            c[i][j] = ((i * j + 1) % ni) as DataType / ni as DataType;
        }
    }
    for i in 0..ni {
        for j in 0..nk {
            a[i][j] = (i * (j + 1) % nk) as DataType / nk as DataType;
        }
    }
    for i in 0..nk {
        for j in 0..nj {
            b[i][j] = (i * (j + 2) % nj) as DataType / nj as DataType;
        }
    }
}

unsafe fn kernel_gemm(
    ni: usize,
    nj: usize,
    nk: usize,
    alpha: DataType,
    beta: DataType,
    c: &mut Array2D<DataType, NI, NJ>,
    a: &Array2D<DataType, NI, NK>,
    b: &Array2D<DataType, NK, NJ>,
) {
    for i in 0..ni {
        for j in 0..nj {
            c[i][j] *= beta;
            for k in 0..nk {
                c[i][j] += alpha * a[i][k] * b[k][j];
            }
        }
    }
}

pub fn bench() -> Duration {
    let ni = NI;
    let nj = NJ;
    let nk = NK;

    let mut alpha = 0.0;
    let mut beta = 0.0;
    let mut c = Array2D::uninit();
    let mut a = Array2D::uninit();
    let mut b = Array2D::uninit();

    unsafe {
        init_array(ni, nj, nk, &mut alpha, &mut beta, &mut c, &mut a, &mut b);
        let elapsed = util::time_function(|| kernel_gemm(ni, nj, nk, alpha, beta, &mut c, &a, &b));
        util::black_box(&c);
        elapsed
    }
}

#[test]
fn check() {
    bench();
}

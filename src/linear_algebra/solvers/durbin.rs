use crate::config::linear_algebra::solvers::durbin::{DataType, N};
use crate::ndarray::{Array1D, ArrayAlloc};
use crate::util;
use std::time::Duration;

unsafe fn init_array(n: usize, r: &mut Array1D<DataType, N>) {
    for i in 0..n {
        r[i] = (n + 1 - i) as DataType;
    }
}

unsafe fn kernel_durbin(n: usize, r: &Array1D<DataType, N>, y: &mut Array1D<DataType, N>) {
    let mut z: [DataType; N] = std::mem::MaybeUninit::uninit().assume_init();

    y[0] = -r[0];
    let mut beta = 1.0;
    let mut alpha = -r[0];
    for k in 1..n {
        beta = (1.0 - alpha * alpha) * beta;
        let mut sum = 0.0;
        for i in 0..k {
            sum += r[k - i - 1] * y[i];
        }
        alpha = -(r[k] + sum) / beta;

        for i in 0..k {
            z[i] = y[i] + alpha * y[k - i - 1];
        }
        for i in 0..k {
            y[i] = z[i];
        }
        y[k] = alpha;
    }
}

pub fn bench() -> Duration {
    let n = N;

    let mut r = Array1D::uninit();
    let mut y = Array1D::uninit();

    unsafe {
        init_array(n, &mut r);
        let elapsed = util::time_function(|| kernel_durbin(n, &r, &mut y));
        util::consume(y);
        elapsed
    }
}

#[test]
fn check() {
    bench();
}

use crate::config::medley::floyd_warshall::{DataType, N};
use crate::ndarray::{Array2D, ArrayAlloc};
use crate::util;
use std::time::Duration;

unsafe fn init_array(n: usize, path: &mut Array2D<DataType, N, N>) {
    for i in 0..n {
        for j in 0..n {
            path[i][j] = (i * j % 7 + 1) as DataType;
            if (i + j) % 13 == 0 || (i + j) % 7 == 0 || (i + j) % 11 == 0 {
                path[i][j] = 999 as DataType;
            }
        }
    }
}

unsafe fn kernel_floyd_warshall(n: usize, path: &mut Array2D<DataType, N, N>) {
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                path[i][j] = if path[i][j] < path[i][k] + path[k][j] {
                    path[i][j]
                } else {
                    path[i][k] + path[k][j]
                };
            }
        }
    }
}

pub fn bench() -> Duration {
    let n = N;

    let mut path = Array2D::uninit();

    unsafe {
        init_array(n, &mut path);
        let elapsed = util::time_function(|| kernel_floyd_warshall(n, &mut path));
        util::consume(path);
        elapsed
    }
}

#[test]
fn check() {
    bench();
}

use crate::config::medley::nussinov::{DataType, N};
use crate::ndarray2::{Array1D, Array2D, ArrayAlloc};
use crate::util;
use std::time::Duration;

type Base = i8;

unsafe fn init_array(n: usize, seq: &mut Array1D<Base, N>, table: &mut Array2D<DataType, N, N>) {
    for i in 0..n {
        seq[i] = ((i + 1) % 4) as Base;
    }

    for i in 0..n {
        for j in 0..n {
            table[i][j] = 0 as DataType;
        }
    }
}

unsafe fn kernel_nussinov(n: usize, seq: &Array1D<Base, N>, table: &mut Array2D<DataType, N, N>) {
    let match_base = |b1, b2| {
        if b1 + b2 == 3 {
            1
        } else {
            0
        }
    };
    let max_score = |s1, s2| {
        if s1 >= s2 {
            s1
        } else {
            s2
        }
    };

    for i in (0..n).rev() {
        for j in (i as isize + 1)..(n as isize) {
            if j - 1 >= 0 {
                let j = j as usize;
                table[i][j] = max_score(table[i][j], table[i][j - 1]);
            }
            if i + 1 < n {
                let j = j as usize;
                table[i][j] = max_score(table[i][j], table[i + 1][j]);
            }

            if j - 1 >= 0 && i + 1 < n {
                let j = j as usize;
                if i < j - 1 {
                    table[i][j] = max_score(
                        table[i][j],
                        table[i + 1][j - 1] + match_base(seq[i], seq[j]),
                    );
                } else {
                    table[i][j] = max_score(table[i][j], table[i + 1][j - 1]);
                }
            }

            let j = j as usize;
            for k in (i + 1)..j {
                table[i][j] = max_score(table[i][j], table[i][k] + table[k + 1][j]);
            }
        }
    }
}

pub fn bench() -> Duration {
    let n = N;

    let mut seq = Array1D::uninit();
    let mut table = Array2D::uninit();

    unsafe {
        init_array(n, &mut seq, &mut table);
        let elapsed = util::time_function(|| kernel_nussinov(n, &seq, &mut table));
        util::black_box(&table);
        elapsed
    }
}

#[test]
fn check() {
    bench();
}

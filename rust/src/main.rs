#![allow(dead_code)]
#![feature(min_const_generics)]

mod datamining;
mod linear_algebra;
mod medley;
mod ndarray;
mod util;

use std::time::Duration;

macro_rules! bench {
    ($module:path) => {
        (stringify!($module), $module)
    };
}

fn main() {
    const NUM_RUNS: usize = 1;
    
    let benches: [(&str, fn(usize) -> Duration); 5]  = [
        bench!(datamining::correlation::bench),
        bench!(datamining::covariance::bench),
        bench!(linear_algebra::blas::gemm::bench),
        bench!(linear_algebra::blas::gemver::bench),
        bench!(linear_algebra::blas::gesummv::bench),
    ];

    let (s, f) = benches[4];
    println!("{}: {:?}", s, f(NUM_RUNS));

    // let elapsed = linear_algebra::blas::gemm::bench();
    // println!("linear_algebra::blas::gemm: {:?}", elapsed);

    // let elapsed = medley::floyd_warshall::bench();
    // println!("medley::floyd_warshall: {:?}", elapsed);
}

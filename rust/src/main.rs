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
    let benches: [(&str, fn(usize) -> Duration); 15]  = [
        bench!(datamining::correlation::bench),
        bench!(datamining::covariance::bench),
        bench!(linear_algebra::blas::gemm::bench),
        bench!(linear_algebra::blas::gemver::bench),
        bench!(linear_algebra::blas::gesummv::bench),
        bench!(linear_algebra::blas::symm::bench),
        bench!(linear_algebra::blas::syr2k::bench),
        bench!(linear_algebra::blas::syrk::bench),
        bench!(linear_algebra::blas::trmm::bench),
        bench!(linear_algebra::kernels::_2mm::bench),
        bench!(linear_algebra::kernels::_3mm::bench),
        bench!(linear_algebra::kernels::atax::bench),
        bench!(linear_algebra::kernels::bicg::bench),
        bench!(linear_algebra::kernels::doitgen::bench),
        bench!(linear_algebra::kernels::mvt::bench),
    ];

    let (s, f) = benches.last().unwrap();
    println!("{}: {:?}", s, f(1));

    // let elapsed = linear_algebra::blas::gemm::bench();
    // println!("linear_algebra::blas::gemm: {:?}", elapsed);

    // let elapsed = medley::floyd_warshall::bench();
    // println!("medley::floyd_warshall: {:?}", elapsed);
}

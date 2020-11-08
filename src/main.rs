#![allow(dead_code)]
#![feature(min_const_generics)]

pub mod datamining;
pub mod linear_algebra;
pub mod medley;
pub mod stencils;

pub mod config;
pub mod ndarray;
pub mod ndarray2;
pub mod util;

// use std::time::Duration;

// macro_rules! bench {
//     ($module:path) => {
//         (stringify!($module), $module)
//     };
// }

// static BENCHMARKS: [(&str, fn(usize) -> Duration); 23] = [
//     bench!(datamining::correlation::bench),
//     bench!(datamining::covariance::bench),
//     bench!(linear_algebra::blas::gemm::bench),
//     bench!(linear_algebra::blas::gemver::bench),
//     bench!(linear_algebra::blas::gesummv::bench),
//     bench!(linear_algebra::blas::symm::bench),
//     bench!(linear_algebra::blas::syr2k::bench),
//     bench!(linear_algebra::blas::syrk::bench),
//     bench!(linear_algebra::blas::trmm::bench),
//     bench!(linear_algebra::kernels::_2mm::bench),
//     bench!(linear_algebra::kernels::_3mm::bench),
//     bench!(linear_algebra::kernels::atax::bench),
//     bench!(linear_algebra::kernels::bicg::bench),
//     bench!(linear_algebra::kernels::doitgen::bench),
//     bench!(linear_algebra::kernels::mvt::bench),
//     bench!(linear_algebra::solvers::cholesky::bench),
//     bench!(linear_algebra::solvers::durbin::bench),
//     bench!(linear_algebra::solvers::gramschmidt::bench),
//     bench!(linear_algebra::solvers::lu::bench),
//     bench!(linear_algebra::solvers::ludcmp::bench),
//     bench!(linear_algebra::solvers::trisolv::bench),
//     bench!(medley::deriche::bench),
//     bench!(medley::floyd_warshall::bench),
//     // bench!(medley::nussinov::bench),
// ];

fn main() {
    // let (s, f) = BENCHMARKS.last().unwrap();
    // println!("{}: {:?}", s, f(1));
}

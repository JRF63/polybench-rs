use polybench_rs::{datamining, linear_algebra, medley, stencils};
use std::time::Duration;

fn main() {
    let benchmarks: &[(&str, fn() -> Duration)] = &[
        ("correlation", datamining::correlation::bench),
        ("covariance", datamining::covariance::bench),
        ("gemm", linear_algebra::blas::gemm::bench),
        ("gemver", linear_algebra::blas::gemver::bench),
        ("gesummv", linear_algebra::blas::gesummv::bench),
        ("symm", linear_algebra::blas::symm::bench),
        ("syr2k", linear_algebra::blas::syr2k::bench),
        ("syrk", linear_algebra::blas::syrk::bench),
        ("trmm", linear_algebra::blas::trmm::bench),
        ("2mm", linear_algebra::kernels::_2mm::bench),
        ("3mm", linear_algebra::kernels::_3mm::bench),
        ("atax", linear_algebra::kernels::atax::bench),
        ("bicg", linear_algebra::kernels::bicg::bench),
        ("doitgen", linear_algebra::kernels::doitgen::bench),
        ("mvt", linear_algebra::kernels::mvt::bench),
        ("cholesky", linear_algebra::solvers::cholesky::bench),
        ("durbin", linear_algebra::solvers::durbin::bench),
        ("gramschmidt", linear_algebra::solvers::gramschmidt::bench),
        ("lu", linear_algebra::solvers::lu::bench),
        ("ludcmp", linear_algebra::solvers::ludcmp::bench),
        ("trisolv", linear_algebra::solvers::trisolv::bench),
        ("deriche", medley::deriche::bench),
        ("floyd_warshall", medley::floyd_warshall::bench),
        ("nussinov", medley::nussinov::bench),
        ("adi", stencils::adi::bench),
        ("fdtd_2d", stencils::fdtd_2d::bench),
        ("heat_3d", stencils::heat_3d::bench),
        ("jacobi_1d", stencils::jacobi_1d::bench),
        ("jacobi_2d", stencils::jacobi_2d::bench),
        ("seidel_2d", stencils::seidel_2d::bench),
    ];
    for (name, func) in benchmarks {
        println!("{:14}: {:>15}", name, func().as_nanos());
    }
}

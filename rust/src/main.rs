#![allow(dead_code)]
#![feature(min_const_generics)]

mod datamining;
mod linear_algebra;
mod ndarray;
mod util;

const NUM_SAMPLES: usize = 8;

fn main() {
    // let elapsed = linear_algebra::kernels::gemm::bench();
    // println!("linear_algebra::kernels::gemm: {:?}", elapsed);

    let elapsed = datamining::correlation::bench();
    println!("datamining::correlation: {:?}", elapsed);
}


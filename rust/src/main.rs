#![allow(dead_code)]
#![feature(min_const_generics)]

mod linear_algebra;
mod ndarray;
mod util;

const NI: usize = 1024;
const NJ: usize = 1024;
const NK: usize = 1024;
const NL: usize = 1024;

const NUM_SAMPLES: usize = 8;

fn main() {
    let elapsed = linear_algebra::kernels::gemm::bench();
    println!("linear_algebra::kernels::gemm: {:?}", elapsed);
}


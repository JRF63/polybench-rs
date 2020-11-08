pub mod datamining {
    pub mod correlation {
        pub type DataType = f64;
        pub const M: usize = 1200;
        pub const N: usize = 1400;
    }
    pub mod covariance {
        pub type DataType = f64;
        pub const M: usize = 1200;
        pub const N: usize = 1400;
    }
}

pub mod linear_algebra {
    pub mod blas {
        pub mod gemm {
            pub type DataType = f64;
            pub const NI: usize = 1000;
            pub const NJ: usize = 1100;
            pub const NK: usize = 1200;
        }
        pub mod gemver {
            pub type DataType = f64;
            pub const N: usize = 2000;
        }
        pub mod gesummv {
            pub type DataType = f64;
            pub const N: usize = 1300;
        }
        pub mod symm {
            pub type DataType = f64;
            pub const M: usize = 1000;
            pub const N: usize = 1200;
        }
        pub mod syr2k {
            pub type DataType = f64;
            pub const M: usize = 1000;
            pub const N: usize = 1200;
        }
        pub mod syrk {
            pub type DataType = f64;
            pub const M: usize = 1000;
            pub const N: usize = 1200;
        }
        pub mod trmm {
            pub type DataType = f64;
            pub const M: usize = 1000;
            pub const N: usize = 1200;
        }
    }
    pub mod kernels {
        pub mod _2mm {
            pub type DataType = f64;
            pub const NI: usize = 800;
            pub const NJ: usize = 900;
            pub const NK: usize = 1100;
            pub const NL: usize = 1200;
        }
        pub mod _3mm {
            pub type DataType = f64;
            pub const NI: usize = 800;
            pub const NJ: usize = 900;
            pub const NK: usize = 1000;
            pub const NL: usize = 1100;
            pub const NM: usize = 1200;
        }
        pub mod atax {
            pub type DataType = f64;
            pub const M: usize = 1900;
            pub const N: usize = 2100;
        }
        pub mod bicg {
            pub type DataType = f64;
            pub const M: usize = 1900;
            pub const N: usize = 2100;
        }
        pub mod doitgen {
            pub type DataType = f64;
            pub const NP: usize = 160;
            pub const NQ: usize = 140;
            pub const NR: usize = 150;
        }
        pub mod mvt {
            pub type DataType = f64;
            pub const N: usize = 2000;
        }
    }
    pub mod solvers {
        pub mod cholesky {
            pub type DataType = f64;
            pub const N: usize = 2000;
        }
        pub mod durbin {
            pub type DataType = f64;
            pub const N: usize = 2000;
        }
        pub mod gramschmidt {
            pub type DataType = f64;
            pub const M: usize = 1000;
            pub const N: usize = 1200;
        }
        pub mod lu {
            pub type DataType = f64;
            pub const N: usize = 2000;
        }
        pub mod ludcmp {
            pub type DataType = f64;
            pub const N: usize = 2000;
        }
        pub mod trisolv {
            pub type DataType = f64;
            pub const N: usize = 2000;
        }
    }
}

pub mod medley {
    pub mod deriche {
        pub type DataType = f32;
        pub const W: usize = 4096;
        pub const H: usize = 2160;
    }
    pub mod floyd_warshall {
        pub type DataType = i32;
        pub const N: usize = 2800;
    }
    pub mod nussinov {
        pub type DataType = i32;
        pub const N: usize = 2500;
    }
}

pub mod stencils {
    pub mod adi {
        pub type DataType = f64;
        pub const N: usize = 1000;
        pub const TSTEPS: usize = 500;
    }
    pub mod fdtd_2d {
        pub type DataType = f64;
        pub const NX: usize = 1000;
        pub const NY: usize = 1200;
        pub const TMAX: usize = 500;
    }
    pub mod heat_3d {
        pub type DataType = f64;
        pub const N: usize = 120;
        pub const TSTEPS: usize = 500;
    }
    pub mod jacobi_1d {
        pub type DataType = f64;
        pub const N: usize = 2000;
        pub const TSTEPS: usize = 500;
    }
    pub mod jacobi_2d {
        pub type DataType = f64;
        pub const N: usize = 1300;
        pub const TSTEPS: usize = 500;
    }
    pub mod seidel_2d {
        pub type DataType = f64;
        pub const N: usize = 2000;
        pub const TSTEPS: usize = 500;
    }
}

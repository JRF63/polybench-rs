pub mod datamining {
    pub mod correlation {
        pub type DataType = f64;
    }
    pub mod covariance {
        pub type DataType = f64;
    }
}

pub mod linear_algebra {
    pub mod blas {
        pub mod gemm {
            pub type DataType = f64;
        }
        pub mod gemver {
            pub type DataType = f64;
        }
        pub mod gesummv {
            pub type DataType = f64;
        }
        pub mod symm {
            pub type DataType = f64;
        }
        pub mod syr2k {
            pub type DataType = f64;
        }
        pub mod syrk {
            pub type DataType = f64;
        }
        pub mod trmm {
            pub type DataType = f64;
        }
    }
    pub mod kernels {
        pub mod _2mm {
            pub type DataType = f64;
        }
        pub mod _3mm {
            pub type DataType = f64;
        }
        pub mod atax {
            pub type DataType = f64;
        }
        pub mod bicg {
            pub type DataType = f64;
        }
        pub mod doitgen {
            pub type DataType = f64;
        }
        pub mod mvt {
            pub type DataType = f64;
        }
    }
    pub mod solvers {
        pub mod cholesky {
            pub type DataType = f64;
        }
        pub mod durbin {
            pub type DataType = f64;
        }
        pub mod gramschmidt {
            pub type DataType = f64;
        }
        pub mod lu {
            pub type DataType = f64;
        }
        pub mod ludcmp {
            pub type DataType = f64;
        }
        pub mod trisolv {
            pub type DataType = f64;
        }
    }
}

pub mod medley {
    pub mod deriche {
        pub type DataType = f32;
    }
    pub mod floyd_warshall {
        pub type DataType = i32;
    }
    pub mod nussinov {
        pub type DataType = i32;
    }
}

pub mod stencils {
    pub mod adi {
        pub type DataType = f64;
    }
    pub mod fdtd_2d {
        pub type DataType = f64;
    }
    pub mod heat_3d {
        pub type DataType = f64;
    }
    pub mod jacobi_1d {
        pub type DataType = f64;
    }
    pub mod jacobi_2d {
        pub type DataType = f64;
    }
    pub mod seidel_2d {
        pub type DataType = f64;
    }
}

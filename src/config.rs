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

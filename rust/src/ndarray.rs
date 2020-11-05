pub struct Array1D<const N: usize>(pub [f64; N]);

#[repr(C, align(32))]
pub struct Array2D<const M: usize, const N: usize>(pub [Array1D<N>; M]);

pub trait AllocUninit: Sized {
    fn uninit() -> Box<Self> {
        let layout = std::alloc::Layout::new::<Self>();
        unsafe {
            let raw = std::alloc::alloc(layout) as *mut Self;
            Box::from_raw(raw)
        }
    }
}

impl<const M: usize, const N: usize> AllocUninit for Array2D<M, N> {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn test_array_sizes() {
        assert_eq!(8388608, size_of::<Array2D<1024, 1024>>());
    }
}
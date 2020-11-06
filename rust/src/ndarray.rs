use std::ops::{Index, IndexMut};

pub struct Array1D<const M: usize>(pub [f64; M]);

impl<const M: usize> Index<usize> for Array1D<M> {
    type Output = f64;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < M);
        unsafe {
            self.0.get_unchecked(index)
        }
    }
}

impl<const M: usize> IndexMut<usize> for Array1D<M> {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < M);
        unsafe {
            self.0.get_unchecked_mut(index)
        }
    }
}

// `pub [[f64; N]; M]` causes a stack overflow when accessed
#[repr(C, align(32))]
pub struct Array2D<const M: usize, const N: usize>(pub [Array1D<N>; M]);

impl<const M: usize, const N: usize> Index<(usize, usize)> for Array2D<M, N> {
    type Output = f64;

    #[inline(always)]
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        debug_assert!(index.0 < M);
        debug_assert!(index.1 < N);
        unsafe {
            self.0.get_unchecked(index.0).0.get_unchecked(index.1)
        }
    }
}

impl<const M: usize, const N: usize> IndexMut<(usize, usize)> for Array2D<M, N> {
    #[inline(always)]
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        debug_assert!(index.0 < M);
        debug_assert!(index.1 < N);
        unsafe {
            self.0.get_unchecked_mut(index.0).0.get_unchecked_mut(index.1)
        }
    }
}

pub trait AllocUninit: Sized {
    fn uninit() -> Box<Self> {
        let layout = std::alloc::Layout::new::<Self>();
        unsafe {
            let raw = std::alloc::alloc(layout) as *mut Self;
            Box::from_raw(raw)
        }
    }
}

impl<const N: usize> AllocUninit for Array1D<N> {}
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

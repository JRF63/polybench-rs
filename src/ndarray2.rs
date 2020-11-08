use std::ops::{self, Index, IndexMut};

#[repr(C, align(32))]
pub struct Array1D<T, const M: usize>(pub [T; M]);

#[repr(C, align(32))]
pub struct Array2D<T, const M: usize, const N: usize>(pub [Array1D<T, N>; M]);

#[repr(C, align(32))]
pub struct Array3D<T, const P: usize, const Q: usize, const R: usize>(pub [Array2D<T, Q, R>; P]);

impl<T, const M: usize> Index<usize> for Array1D<T, M> {
    type Output = T;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < M);
        unsafe { self.0.get_unchecked(index) }
    }
}

impl<T, const M: usize> IndexMut<usize> for Array1D<T, M> {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < M);
        unsafe { self.0.get_unchecked_mut(index) }
    }
}

impl<T, const M: usize, const N: usize> Index<usize> for Array2D<T, M, N> {
    type Output = Array1D<T, N>;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < M);
        unsafe { self.0.get_unchecked(index) }
    }
}

impl<T, const M: usize, const N: usize> IndexMut<usize> for Array2D<T, M, N> {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < M);
        unsafe { self.0.get_unchecked_mut(index) }
    }
}

impl<T, const P: usize, const Q: usize, const R: usize> Index<usize> for Array3D<T, P, Q, R> {
    type Output = Array2D<T, Q, R>;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < P);
        unsafe { self.0.get_unchecked(index) }
    }
}

impl<T, const P: usize, const Q: usize, const R: usize> IndexMut<usize> for Array3D<T, P, Q, R> {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < P);
        unsafe { self.0.get_unchecked_mut(index) }
    }
}

pub trait ArrayAlloc: Sized {
    fn uninit() -> Box<Self> {
        let layout = std::alloc::Layout::new::<Self>();
        unsafe {
            let raw = std::alloc::alloc(layout) as *mut Self;
            Box::from_raw(raw)
        }
    }

    fn zeroed() -> Box<Self> {
        let layout = std::alloc::Layout::new::<Self>();
        unsafe {
            let raw = std::alloc::alloc_zeroed(layout) as *mut Self;
            Box::from_raw(raw)
        }
    }
}

impl<T, const N: usize> ArrayAlloc for Array1D<T, N> {}
impl<T, const M: usize, const N: usize> ArrayAlloc for Array2D<T, M, N> {}
impl<T, const P: usize, const Q: usize, const R: usize> ArrayAlloc for Array3D<T, P, Q, R> {}

impl<T, const N: usize> Array2D<T, N, N>
where
    T: Copy + ops::Mul<Output = T> + ops::AddAssign<T>,
{
    pub fn make_positive_semi_definite(&mut self) {
        let mut b = Array2D::<T, N, N>::zeroed();
        let n = N;

        for t in 0..n {
            for r in 0..n {
                for s in 0..n {
                    b[r][s] += self[r][t] * self[s][t];
                }
            }
        }
        for r in 0..n {
            for s in 0..n {
                self[r][s] = b[r][s];
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn check_array_sizes() {
        assert_eq!(1024, size_of::<Array1D<u8, 1024>>());
        assert_eq!(8388608, size_of::<Array2D<f64, 1024, 1024>>());
        assert_eq!(67108864, size_of::<Array3D<f32, 256, 256, 256>>());
    }
}

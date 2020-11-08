use std::fmt;
use std::ops::{self, Index, IndexMut};

#[repr(C, align(32))]
pub struct Array1D<T, const M: usize>(pub [T; M]);

#[repr(C, align(32))]
pub struct Array2D<T, const M: usize, const N: usize>(pub [Array1D<T, N>; M]);

#[repr(C, align(32))]
pub struct Array3D<T, const M: usize, const N: usize, const P: usize>(pub [Array2D<T, N, P>; M]);

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

impl<T, const M: usize, const N: usize, const P: usize> Index<usize> for Array3D<T, M, N, P> {
    type Output = Array2D<T, N, P>;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < M);
        unsafe { self.0.get_unchecked(index) }
    }
}

impl<T, const M: usize, const N: usize, const P: usize> IndexMut<usize> for Array3D<T, M, N, P> {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < M);
        unsafe { self.0.get_unchecked_mut(index) }
    }
}

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
impl<T, const M: usize, const N: usize, const P: usize> ArrayAlloc for Array3D<T, M, N, P> {}

impl<T, const N: usize> fmt::Display for Array1D<T, N>
where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for x in &self.0[..(self.0.len() - 1)] {
            write!(f, "{}, ", x)?;
        }
        if let Some(last) = self.0.last() {
            write!(f, "{}]", last)?;
        }
        Ok(())
    }
}

impl<T, const M: usize, const N: usize> fmt::Display for Array2D<T, M, N>
where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for x in &self.0[..(self.0.len() - 1)] {
            write!(f, "{}, ", x)?;
        }
        if let Some(last) = self.0.last() {
            write!(f, "{}]", last)?;
        }
        Ok(())
    }
}

impl<T, const M: usize, const N: usize, const P: usize> fmt::Display for Array3D<T, M, N, P> 
where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for x in &self.0[..(self.0.len() - 1)] {
            write!(f, "{}, ", x)?;
        }
        if let Some(last) = self.0.last() {
            write!(f, "{}]", last)?;
        }
        Ok(())
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

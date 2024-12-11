use core::ops::{BitXor, BitXorAssign};

use generic_array::{ArrayLength, GenericArray};

/// Array that implements [`BitXor`].
#[derive(Debug)]
pub struct XorArray<T, N: ArrayLength>(pub GenericArray<T, N>);

impl<T: Clone, N: ArrayLength> Clone for XorArray<T, N> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: Copy, N: ArrayLength<ArrayType<T>: Copy>> Copy for XorArray<T, N> {}

impl<T, N: ArrayLength> AsRef<[T]> for XorArray<T, N> {
    fn as_ref(&self) -> &[T] {
        &self.0
    }
}

impl<T: Default, N: ArrayLength> Default for XorArray<T, N> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T: BitXorAssign<Rhs>, Rhs, N: ArrayLength> BitXorAssign<XorArray<Rhs, N>> for XorArray<T, N> {
    fn bitxor_assign(&mut self, rhs: XorArray<Rhs, N>) {
        for (a, b) in self.0.iter_mut().zip(rhs.0.into_iter()) {
            *a ^= b;
        }
    }
}

impl<T: BitXorAssign<Rhs>, Rhs, N: ArrayLength> BitXor<XorArray<Rhs, N>> for XorArray<T, N> {
    type Output = XorArray<T, N>;

    fn bitxor(mut self, rhs: XorArray<Rhs, N>) -> Self::Output {
        self ^= rhs;
        self
    }
}

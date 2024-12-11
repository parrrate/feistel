use std::{marker::PhantomData, ops::BitXor};

use crate::{Network, Round};

/// [`Network`] with [`Network::L`] equal to [`Network::R`] and [`Network::forward`] equivalent to
/// [`Network::backward`].
pub trait SymmetricNetwork:
    Sized + IntoIterator<Item: Round<Self::T, L = Self::T>, IntoIter: DoubleEndedIterator>
{
    /// Half of a block.
    type T: BitXor<Output = Self::T>;
    /// `swap(swap(block)) == block`
    fn swap(block: (Self::T, Self::T)) -> (Self::T, Self::T) {
        let (left, right) = block;
        (right, left)
    }
}

impl<I: SymmetricNetwork> Network for I {
    type L = I::T;
    type R = I::T;
    fn forward(block: (Self::R, Self::L)) -> (Self::L, Self::R) {
        I::swap(block)
    }
    fn backward(block: (Self::L, Self::R)) -> (Self::L, Self::R) {
        I::swap(block)
    }
}

/// [`SymmetricNetwork`] wrapper around an [`IntoIterator`].
pub struct Symmetric<I, T>(I, PhantomData<T>);

impl<I: Clone, T> Clone for Symmetric<I, T> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}

impl<I: Copy, T> Copy for Symmetric<I, T> {}

impl<I, T: BitXor<Output = T>> Symmetric<I, T> {
    /// Make a [`SymmetricNetwork`] out of an [`IntoIterator`].
    pub fn new(rounds: I) -> Self {
        Self(rounds, PhantomData)
    }
}

impl<I: IntoIterator, T> IntoIterator for Symmetric<I, T> {
    type Item = I::Item;
    type IntoIter = I::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<
        T: BitXor<Output = T>,
        I: IntoIterator<Item: Round<T, L = T>, IntoIter: DoubleEndedIterator>,
    > SymmetricNetwork for Symmetric<I, T>
{
    type T = T;
}

mod private {
    pub trait Sealed<T> {}
}

/// Extension trait for creating [`SymmetricNetwork`]s.
pub trait SymmetricExt<T: BitXor<Output = T>>:
    Sized + IntoIterator<Item: Round<T, L = T>, IntoIter: DoubleEndedIterator> + private::Sealed<T>
{
    /// Make a [`SymmetricNetwork`] from an [`IntoIterator`].
    fn feistel_symmetric(self) -> Symmetric<Self, T> {
        Symmetric::new(self)
    }
}

impl<
        T: BitXor<Output = T>,
        I: IntoIterator<Item: Round<T, L = T>, IntoIter: DoubleEndedIterator>,
    > private::Sealed<T> for I
{
}

impl<
        T: BitXor<Output = T>,
        I: IntoIterator<Item: Round<T, L = T>, IntoIter: DoubleEndedIterator> + private::Sealed<T>,
    > SymmetricExt<T> for I
{
}

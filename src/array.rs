use core::{
    marker::PhantomData,
    ops::{Add, BitXor, BitXorAssign, Sub},
};

use generic_array::{
    sequence::{Concat, Split},
    typenum::Sum,
    ArrayLength, GenericArray,
};

use crate::{Network, Reverse, Round, XorArray};

/// [`Network`] dealing with [`GenericArray`]s.
pub trait ArrayNetwork:
    Network<L = XorArray<Self::T, Self::LN>, R = XorArray<Self::T, Self::RN>>
{
    /// Length of the left half of a block.
    type LN: ArrayLength
        + Add<
            Self::RN,
            Output: ArrayLength
                        + Sub<Self::LN, Output = Self::RN>
                        + Sub<Self::RN, Output = Self::LN>,
        >;
    /// Length of the right half of a block.
    type RN: ArrayLength + Add<Self::LN, Output = <Self::LN as Add<Self::RN>>::Output>;
    /// Element of an array.
    type T: BitXor;
    /// Encrypt an array.
    fn array_encrypt(self, block: Block<Self>) -> Block<Self> {
        let (left, right) = Split::split(block);
        let (left, right) = (XorArray(left), XorArray(right));
        let (left, right) = self.encrypt((left, right));
        Concat::concat(left.0, right.0)
    }
    /// Decrypt an array.
    fn array_decrypt(self, block: Block<Self>) -> Block<Self> {
        Reverse(self).array_encrypt(block)
    }
}

type Block<I> =
    GenericArray<<I as ArrayNetwork>::T, Sum<<I as ArrayNetwork>::LN, <I as ArrayNetwork>::RN>>;

impl<
        T: BitXor,
        L: ArrayLength + Add<R, Output: ArrayLength + Sub<L, Output = R> + Sub<R, Output = L>>,
        R: ArrayLength + Add<L, Output = <L as Add<R>>::Output>,
        I: Network<L = XorArray<T, L>, R = XorArray<T, R>>,
    > ArrayNetwork for I
{
    type LN = L;
    type RN = R;
    type T = T;
}

/// [`ArrayNetwork`] wrapper around an [`IntoIterator`].
pub struct Array<I, T, R>(I, PhantomData<(T, R)>);

impl<I: Clone, T, R> Clone for Array<I, T, R> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}

impl<I: Copy, T, R> Copy for Array<I, T, R> {}

impl<
        T: BitXorAssign,
        L: ArrayLength + Add<R, Output: ArrayLength + Sub<L, Output = R> + Sub<R, Output = L>>,
        R: ArrayLength + Add<L, Output = <L as Add<R>>::Output>,
        I: IntoIterator<
            Item: Round<XorArray<T, R>, L = XorArray<T, L>>,
            IntoIter: DoubleEndedIterator,
        >,
    > Array<I, T, R>
{
    /// Make an [`ArrayNetwork`] out of an [`IntoIterator`].
    pub fn new(rounds: I) -> Self {
        Self(rounds, PhantomData)
    }
}

impl<I: IntoIterator, T, R> IntoIterator for Array<I, T, R> {
    type Item = I::Item;
    type IntoIter = I::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<
        T: BitXorAssign,
        L: ArrayLength + Add<R, Output: ArrayLength + Sub<L, Output = R> + Sub<R, Output = L>>,
        R: ArrayLength + Add<L, Output = <L as Add<R>>::Output>,
        I: IntoIterator<
            Item: Round<XorArray<T, R>, L = XorArray<T, L>>,
            IntoIter: DoubleEndedIterator,
        >,
    > Network for Array<I, T, R>
{
    type L = XorArray<T, L>;
    type R = XorArray<T, R>;
    fn forward(block: (Self::L, Self::R)) -> (Self::L, Self::R) {
        let (left, right) = block;
        let (left, right) = Split::split(Concat::concat(right.0, left.0));
        let (left, right) = (XorArray(left), XorArray(right));
        (left, right)
    }
    fn backward(block: (Self::L, Self::R)) -> (Self::L, Self::R) {
        let (left, right) = block;
        let (right, left) = Split::split(Concat::concat(left.0, right.0));
        let (right, left) = (XorArray(right), XorArray(left));
        (left, right)
    }
}

mod private {
    pub trait Sealed<T, R> {}
}

/// Extension trait for creating [`ArrayNetwork`]s.
pub trait ArrayExt<
    T: BitXorAssign,
    R: ArrayLength
        + Add<Self::L, Output: ArrayLength + Sub<R, Output = Self::L> + Sub<Self::L, Output = R>>,
>:
    Sized
    + IntoIterator<
        Item: Round<XorArray<T, R>, L = XorArray<T, Self::L>>,
        IntoIter: DoubleEndedIterator,
    > + private::Sealed<T, R>
{
    /// Length of the left half of a block.
    type L: ArrayLength + Add<R, Output = <R as Add<Self::L>>::Output>;

    /// Make an [`ArrayNetwork`] from an [`IntoIterator`].
    fn feistel_array(self) -> Array<Self, T, R> {
        Array::new(self)
    }
}

impl<
        T: BitXorAssign,
        L: ArrayLength + Add<R, Output = <R as Add<L>>::Output>,
        R: ArrayLength + Add<L, Output: ArrayLength + Sub<R, Output = L> + Sub<L, Output = R>>,
        I: IntoIterator<
            Item: Round<XorArray<T, R>, L = XorArray<T, L>>,
            IntoIter: DoubleEndedIterator,
        >,
    > private::Sealed<T, R> for I
{
}

impl<
        T: BitXorAssign,
        L: ArrayLength + Add<R, Output = <R as Add<L>>::Output>,
        R: ArrayLength + Add<L, Output: ArrayLength + Sub<R, Output = L> + Sub<L, Output = R>>,
        I: IntoIterator<
            Item: Round<XorArray<T, R>, L = XorArray<T, L>>,
            IntoIter: DoubleEndedIterator,
        >,
    > ArrayExt<T, R> for I
{
    type L = L;
}

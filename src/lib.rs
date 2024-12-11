//! Generic implementation of a Feistel Cipher.
//!
//! ```rust
//! # #[cfg(feature = "array")]
//! # fn main() {
//! # use sha2::{Digest, Sha256};
//! # use generic_array::ConstArrayLength;
//! # use feistel::*;
//! # let key: [u8; 32] = core::array::from_fn(|i| i as _);
//! let network = key
//!     .chunks_exact(4)
//!     .map(|chunk| {
//!         move |half: &XorArray<u8, ConstArrayLength<32>>| {
//!             let mut hasher = Sha256::new();
//!             hasher.update(half);
//!             hasher.update(chunk);
//!             let value: [u8; 32] = hasher.finalize().into();
//!             XorArray(value.into())
//!         }
//!     })
//!     .feistel_symmetric();
//! let original = [0; 64].into();
//! let encrypted = network.clone().array_encrypt(original);
//! assert_ne!(original, encrypted);
//! let decrypted = network.clone().array_decrypt(encrypted);
//! assert_eq!(original, decrypted);
//! # }
//! # #[cfg(not(feature = "array"))]
//! # fn main() {}
//! ```

#![no_std]

use core::ops::BitXor;

#[cfg(feature = "array")]
pub use self::{
    array::{Array, ArrayExt, ArrayNetwork},
    xor_array::XorArray,
};
pub use self::{
    round::Round,
    symmetric::{Symmetric, SymmetricExt, SymmetricNetwork},
};

#[cfg(feature = "array")]
mod array;
mod round;
mod symmetric;
#[cfg(feature = "array")]
mod xor_array;

/// Feistel network.
pub trait Network:
    Sized + IntoIterator<Item: Round<Self::R, L = Self::L>, IntoIter: DoubleEndedIterator>
{
    /// The left half of a block.
    type L: BitXor<Output = Self::L>;
    /// The right half of a block.
    type R;
    /// Rearrange halves coming from hashing and xoring the previous block into the next block.
    ///
    /// Note: returning `(block.0, block.1)` is generally incorrect.
    fn forward(block: (Self::L, Self::R)) -> (Self::L, Self::R);
    /// Inverse of [`Network::forward`].
    fn backward(block: (Self::L, Self::R)) -> (Self::L, Self::R);
    /// Encrypt a block represented as halves.
    fn encrypt(self, block: (Self::L, Self::R)) -> (Self::L, Self::R) {
        let (mut left, mut right) = block;
        let mut prev = false;
        for round in self {
            if prev {
                (left, right) = Self::forward((left, right));
            }
            left = round.run(&right) ^ left;
            prev = true
        }
        (left, right)
    }
    /// Decrypt a block represented as halves.
    fn decrypt(self, block: (Self::L, Self::R)) -> (Self::L, Self::R) {
        Reverse(self).encrypt(block)
    }
}

struct Reverse<I>(pub I);

impl<I: IntoIterator<IntoIter: DoubleEndedIterator>> IntoIterator for Reverse<I> {
    type Item = I::Item;
    type IntoIter = core::iter::Rev<I::IntoIter>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter().rev()
    }
}

impl<I: Network> Network for Reverse<I> {
    type L = I::L;
    type R = I::R;

    fn forward(block: (Self::L, Self::R)) -> (Self::L, Self::R) {
        I::backward(block)
    }

    fn backward(block: (Self::L, Self::R)) -> (Self::L, Self::R) {
        I::forward(block)
    }
}

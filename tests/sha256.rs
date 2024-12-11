#![cfg(feature = "array")]

use feistel::{ArrayExt, ArrayNetwork, SymmetricExt, XorArray};
use generic_array::ConstArrayLength;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Copy)]
struct Iterable<F>(F);

impl<F: Fn() -> I, I: IntoIterator> IntoIterator for Iterable<F> {
    type Item = I::Item;

    type IntoIter = I::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0().into_iter()
    }
}

fn key() -> [u8; 32] {
    core::array::from_fn(|i| i as _)
}

#[test]
fn symmetric() {
    type BlockHalf = XorArray<u8, ConstArrayLength<32>>;

    let key = key();

    let network = Iterable(|| {
        key.chunks_exact(4).map(|chunk| {
            move |half: &BlockHalf| {
                let mut hasher = Sha256::new();
                hasher.update(half);
                hasher.update(chunk);
                let value: [u8; 32] = hasher.finalize().into();
                XorArray(value.into())
            }
        })
    })
    .feistel_symmetric();
    let original = [0; 64].into();
    let encrypted = network.array_encrypt(original);
    assert_ne!(original, encrypted);
    let decrypted = network.array_decrypt(encrypted);
    assert_eq!(original, decrypted);
}

#[test]
fn array_symmetric() {
    type BlockHalf = XorArray<u8, ConstArrayLength<32>>;

    let key = key();

    let network = Iterable(|| {
        key.chunks_exact(4).map(|chunk| {
            move |half: &BlockHalf| {
                let mut hasher = Sha256::new();
                hasher.update(half);
                hasher.update(chunk);
                let value: [u8; 32] = hasher.finalize().into();
                XorArray(value.into())
            }
        })
    })
    .feistel_array();
    let original = [0; 64].into();
    let encrypted = network.array_encrypt(original);
    assert_ne!(original, encrypted);
    let decrypted = network.array_decrypt(encrypted);
    assert_eq!(original, decrypted);
}

#[test]
fn array_asymmetric() {
    type BlockHalf = XorArray<u8, ConstArrayLength<16>>;

    let key = key();

    let network = Iterable(|| {
        key.chunks_exact(4).map(|chunk| {
            move |half: &BlockHalf| {
                let mut hasher = Sha256::new();
                hasher.update(half);
                hasher.update(chunk);
                let value: [u8; 32] = hasher.finalize().into();
                XorArray(value.into())
            }
        })
    })
    .feistel_array();
    let original = [0; 48].into();
    let encrypted = network.array_encrypt(original);
    assert_ne!(original, encrypted);
    let decrypted = network.array_decrypt(encrypted);
    assert_eq!(original, decrypted);
}

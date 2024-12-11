use std::num::Wrapping;

use feistel::{Network, Round, SymmetricNetwork};

#[derive(Debug, Clone, Copy)]
struct Rnd(Wrapping<u64>);

impl Round<Wrapping<u64>> for Rnd {
    type L = Wrapping<u64>;

    fn run(self, right: &Wrapping<u64>) -> Self::L {
        self.0 * *right + Wrapping(991)
    }
}

#[derive(Debug, Clone, Copy)]
struct Net([Rnd; 4]);

impl IntoIterator for Net {
    type Item = Rnd;
    type IntoIter = std::array::IntoIter<Rnd, 4>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl SymmetricNetwork for Net {
    type T = Wrapping<u64>;

    fn swap(block: (Self::T, Self::T)) -> (Self::T, Self::T) {
        let (left, right) = block;
        let left = Wrapping(23) - left;
        let right = Wrapping(101) - right;
        (left, right)
    }
}

#[test]
fn weird_symmetric() {
    let network = Net([
        Rnd(Wrapping(2)),
        Rnd(Wrapping(3)),
        Rnd(Wrapping(5)),
        Rnd(Wrapping(7)),
    ]);
    let original = (Wrapping(426), Wrapping(216));
    let encrypted = network.encrypt(original);
    assert_ne!(original, encrypted);
    let decrypted = network.decrypt(encrypted);
    assert_eq!(original, decrypted);
}

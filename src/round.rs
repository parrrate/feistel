/// Hash function for a specific round. Usually depends on a key.
pub trait Round<R> {
    /// Type of the right half of the block.
    type L;
    /// Inspect the left half to generate a value to be xored with the right half.
    fn run(self, right: &R) -> Self::L;
}

impl<L, R, F: FnOnce(&L) -> R> Round<L> for F {
    type L = R;

    fn run(self, right: &L) -> Self::L {
        self(right)
    }
}

# Generic implementation of a Feistel Cipher

```rust
let network = key
    .chunks_exact(4)
    .map(|chunk| {
        move |half: &XorArray<u8, ConstArrayLength<32>>| {
            let mut hasher = Sha256::new();
            hasher.update(half);
            hasher.update(chunk);
            let value: [u8; 32] = hasher.finalize().into();
            XorArray(value.into())
        }
    })
    .feistel_symmetric();
let original = [0; 64].into();
let encrypted = network.clone().array_encrypt(original);
assert_ne!(original, encrypted);
let decrypted = network.clone().array_decrypt(encrypted);
assert_eq!(original, decrypted);
```

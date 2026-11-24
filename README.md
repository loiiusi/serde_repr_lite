# serde_repr_lite
Liteweight replacement of [`serde_repr`](https://docs.rs/serde_repr/).

The crate provide [`serde_repr`] macro for implement [`serde::Serialize`](serde_core::Serialize) and
[`serde::Deserialize`](serde_core::Deserialize) together.

Unlike [`serde_repr`](https://docs.rs/serde_repr/) crate. This crate does not allow `#[serde(other)]`.

# Examples
```rust
serde_repr_lite::serde_repr! {
    // `repr` must be the first attribute.
    #[repr(u8)]
    #[derive(Debug, PartialEq)]
    enum SmallPrime {
        Two = 2,
        Three = 3,
        Five = 5,
        Seven = 7,
    }
}

let j = serde_json::to_string(&SmallPrime::Seven)?;
assert_eq!(j, "7");

let p: SmallPrime = serde_json::from_str("2")?;
assert_eq!(p, SmallPrime::Two);
# Ok::<_, serde_json::Error>(())
```

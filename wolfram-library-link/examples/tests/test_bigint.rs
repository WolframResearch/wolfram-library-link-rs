use num_bigint::{BigInt, BigUint};
use wolfram_library_link::export;

#[export]
fn test_bigint_roundtrip(x: BigInt) -> BigInt {
    x + 1
}

#[export]
fn test_biguint_roundtrip(x: BigUint) -> BigUint {
    x + 1u32
}

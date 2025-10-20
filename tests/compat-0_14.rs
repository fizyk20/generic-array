#![cfg(feature = "compat-0_14")]
#![deny(deprecated)]

use aes::{cipher::KeyInit as _, Aes128};
use generic_array::GenericArray;

#[test]
fn test_compat() {
    let _ = Aes128::new(GenericArray::from_slice(&[0u8; 16]).as_ref());
}

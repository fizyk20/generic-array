use generic_array::arr;
use generic_array::typenum;
use generic_array::GenericArray;
use std::str::from_utf8;
use typenum::{U2048, U32};

#[test]
fn short_lower_hex() {
    let ar = arr![10u8, 20, 30];
    assert_eq!(format!("{:x}", ar), "0a141e");
}

#[test]
fn short_upper_hex() {
    let ar = arr![30u8, 20, 10];
    assert_eq!(format!("{:X}", ar), "1E140A");
}

// Exercises the medium-array branch (16 <= N <= 1024) of `generic_hex`,
// which the short (< 16) and long (> 1024) tests skip.
#[test]
fn medium_lower_hex() {
    let ar = GenericArray::<u8, U32>::default();
    assert_eq!(format!("{:x}", ar), from_utf8(&[b'0'; 64]).unwrap());
}

#[test]
fn medium_upper_hex_truncated() {
    let mut ar = GenericArray::<u8, U32>::default();
    ar[0] = 0xAB;
    assert_eq!(format!("{:.5X}", ar), "AB000");
}

#[test]
fn long_lower_hex() {
    let ar = GenericArray::<u8, U2048>::default();
    assert_eq!(format!("{:x}", ar), from_utf8(&[b'0'; 4096]).unwrap());
}

#[test]
fn long_lower_hex_truncated() {
    let ar = GenericArray::<u8, U2048>::default();
    assert_eq!(format!("{:.3001x}", ar), from_utf8(&[b'0'; 3001]).unwrap());
}

#[test]
fn long_upper_hex() {
    let ar = GenericArray::<u8, U2048>::default();
    assert_eq!(format!("{:X}", ar), from_utf8(&[b'0'; 4096]).unwrap());
}

#[test]
fn long_upper_hex_truncated() {
    let ar = GenericArray::<u8, U2048>::default();
    assert_eq!(format!("{:.2777X}", ar), from_utf8(&[b'0'; 2777]).unwrap());
}

#[test]
fn truncated_lower_hex() {
    let ar = arr![10u8, 20, 30, 40, 50];
    assert_eq!(format!("{:.2x}", ar), "0a");
    assert_eq!(format!("{:.3x}", ar), "0a1");
    assert_eq!(format!("{:.4x}", ar), "0a14");
}

#[test]
fn truncated_upper_hex() {
    let ar = arr![30u8, 20, 10, 17, 0];
    assert_eq!(format!("{:.4X}", ar), "1E14");
    assert_eq!(format!("{:.5X}", ar), "1E140");
    assert_eq!(format!("{:.6X}", ar), "1E140A");
    assert_eq!(format!("{:.7X}", ar), "1E140A1");
    assert_eq!(format!("{:.8X}", ar), "1E140A11");
}

#[macro_use]
extern crate generic_array;
extern crate typenum;

use std::str::from_utf8;
use generic_array::GenericArray;
use typenum::U2048;


#[test]
fn short_lower_hex() {
    let ar = arr![u8; 10, 20, 30];
    assert_eq!(format!("{:x}", ar), "0a141e");
}

#[test]
fn short_upper_hex() {
    let ar = arr![u8; 30, 20, 10];
    assert_eq!(format!("{:X}", ar), "1E140A");
}

#[test]
fn long_lower_hex() {
    let ar = GenericArray::<u8, U2048>::new();
    assert_eq!(format!("{:x}", ar),
        from_utf8(&[b'0'; 4096]).unwrap());
}

#[test]
fn long_upper_hex() {
    let ar = GenericArray::<u8, U2048>::new();
    assert_eq!(format!("{:X}", ar),
        from_utf8(&[b'0'; 4096]).unwrap());
}

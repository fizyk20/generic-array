#[cfg(feature = "alloc")]
extern crate alloc;

use generic_array::{arr, GenericArray};

#[test]
fn empty_without_trailing_comma() {
    let ar: GenericArray<u8, _> = arr![];
    assert_eq!(format!("{:x}", ar), "");
}

#[test]
fn empty_with_trailing_comma() {
    let ar: GenericArray<u8, _> = arr![, ];
    assert_eq!(format!("{:x}", ar), "");
}

#[test]
fn without_trailing_comma() {
    let ar = arr![10, 20, 30];
    assert_eq!(format!("{:x}", ar), "0a141e");
}

#[test]
fn with_trailing_comma() {
    let ar = arr![10u8, 20, 30,];
    assert_eq!(format!("{:x}", ar), "0a141e");
}

#[test]
fn const_context() {
    const AR: GenericArray<u8, typenum::U3> = arr![10, 20, 30];
    assert_eq!(format!("{:x}", AR), "0a141e");
}

#[test]
fn repeat_expression() {
    let ar = arr![0xc0u8; typenum::U4];
    assert_eq!(format!("{:x}", ar), "c0c0c0c0");

    _ = arr![0; 12];
    _ = arr![1; 't' as usize];
}

#[cfg(feature = "alloc")]
#[test]
fn alloc_arr() {
    use generic_array::box_arr;

    let ar = box_arr![0xc0u8; typenum::U4];
    assert_eq!(format!("{:x}", &*ar), "c0c0c0c0");

    _ = box_arr![0; 12];
    _ = box_arr![1; 't' as usize];
}

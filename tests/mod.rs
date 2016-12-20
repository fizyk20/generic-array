#[macro_use]
extern crate generic_array;
use generic_array::typenum::{U3, U97};
use generic_array::GenericArray;
use std::ops::Drop;

#[test]
fn test() {
    let mut list97 = [0; 97];
    for i in 0..97 {
        list97[i] = i as i32;
    }
    let l : GenericArray<i32, U97> = GenericArray::from_slice(&list97);
    assert_eq!(l[0], 0);
    assert_eq!(l[1], 1);
    assert_eq!(l[32], 32);
    assert_eq!(l[56], 56);
}

static mut drop_counter: u32 = 0;

#[derive(Clone)]
struct TestDrop(u32);

impl Drop for TestDrop {
    fn drop(&mut self) {
        unsafe {
            drop_counter += 1;
        }
    }
}

#[test]
fn test_drop() {
    unsafe {
        drop_counter = 0;
    }
    {
        // This clones the structs, so we will have 6 drops, not 3
        let _: GenericArray<TestDrop, U3> = arr![TestDrop; TestDrop(1), TestDrop(2), TestDrop(3)];
    }
    unsafe {
        assert_eq!(drop_counter, 6);    // 6 drops, as explained above
    }
}

#[test]
fn test_arr() {
    let test: GenericArray<u32, U3> = arr![u32; 1, 2, 3];
    assert_eq!(test[1], 2);
}

#[test]
fn test_copy() {
    let test = arr![u32; 1, 2, 3];
    let test2 = test;
    // if GenericArray is not copy, this should fail as a use of a moved value
    assert_eq!(test[1], 2);
    assert_eq!(test2[0], 1);
}

#[test]
fn test_iter_flat_map() {
    assert!((0..5).flat_map(|i| arr![i32; 2 * i, 2 * i + 1]).eq(0..10));
}

#[test]
fn test_unit_macro(){
    let arr = arr![f32; 3.14];
    assert_eq!(arr[0], 3.14);
}

#[test]
fn test_empty_macro(){
    let arr = arr![f32;];
}

/// This test should cause a helpful compile error if uncommented.
// #[test]
// fn test_empty_macro2(){
//     let arr = arr![];
// }

#[cfg(feature="serde")]
mod impl_serde {
    extern crate serde_json;

    use generic_array::GenericArray;
    use generic_array::typenum::U6;

    #[test]
    fn test_serde_implementation() {
        let array: GenericArray<f64, U6> = arr![f64; 0.0, 5.0, 3.0, 7.07192, 76.0, -9.0];
        let string = serde_json::to_string(&array).unwrap();
        assert_eq!(string, "[0.0,5.0,3.0,7.07192,76.0,-9.0]");

        let test_array: GenericArray<f64, U6> = serde_json::from_str(&string).unwrap();
        assert_eq!(test_array, array);
    }
}

#![no_std]
#[macro_use]
extern crate generic_array;
use generic_array::typenum::{U3, U97};
use generic_array::GenericArray;
use core::ops::Drop;

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

#[allow(non_upper_case_globals)]
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
fn test_from_slice() {
    let arr = [1,2,3,4];
    let gen_arr = generic_array::from_slice::<_, U3>(&arr[..3]);
    assert_eq!(&arr[..3], gen_arr.as_slice());
}

#[test]
fn test_from_mut_slice() {
    let mut arr = [1,2,3,4];
    {
        let mut gen_arr = generic_array::from_mut_slice::<_, U3>(&mut arr[..3]);
        gen_arr[2] = 10;
    }
    assert_eq!(arr, [1,2,10,4]);
}

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

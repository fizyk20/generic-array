#[macro_use]
extern crate generic_array;
extern crate typenum;
use typenum::consts::{U3, U97};
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

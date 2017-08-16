#![no_std]

#[macro_use]
extern crate generic_array;

use core::ops::Mul;
use generic_array::GenericArray;
use generic_array::builder::RecursiveArrayBuilder;

use generic_array::typenum::U4;

#[test]
fn builder_generator() {
    let b: GenericArray<i32, U4> = GenericArray::generate(|i| i as i32 * 4);

    assert_eq!(b, arr![i32; 0, 4, 8, 12]);
}

#[test]
fn builder_map() {
    let b: GenericArray<i32, U4> = GenericArray::generate(|i| i as i32 * 4).map(|x| x - 3);

    assert_eq!(b, arr![i32; -3, 1, 5, 9]);
}

#[test]
fn builder_zip() {
    let a: GenericArray<_, U4> = GenericArray::generate(|i| i + 1);
    let b: GenericArray<_, U4> = GenericArray::generate(|i| i as i32 * 4);

    let c = a.zip(b, |r, l| r as i32 + l);

    assert_eq!(c, arr![i32; 1, 6, 11, 16]);
}

#[test]
fn recursive_builder() {
    let b = RecursiveArrayBuilder::<_, U4, _>::new()
        .next(|| 1)
        .next(|| 2)
        .next(|| 3)
        .next(|| 4)
        .finish();

    assert_eq!(b, arr![i32; 1, 2, 3, 4]);
}

#[test]
fn recursive_generator() {
    let b: GenericArray<i32, U4> = RecursiveArrayBuilder::generate(|i| i as i32 * 2);

    assert_eq!(b, arr![i32; 0, 2, 4, 6]);
}

#[test]
fn recursive_map() {
    let a = arr![i32; 1, 2, 3, 4];

    let b = RecursiveArrayBuilder::map_ref(&a, |x| x * 2);
    let c = RecursiveArrayBuilder::map(a, |x| x * 3);

    assert_eq!(b, arr![i32; 2, 4, 6, 8]);
    assert_eq!(c, arr![i32; 3, 6, 9, 12]);
}

#[test]
fn recursive_zip() {
    let a = arr![i32; 1, 3, 5, 7];
    let b = arr![i32; 2, 4, 6, 8];

    let sum = RecursiveArrayBuilder::zip_ref(&a, &b, |lhs, rhs| *lhs + *rhs);
    let muls = RecursiveArrayBuilder::zip(a, b, Mul::mul);

    assert_eq!(sum, arr![i32; 3, 7, 11, 15]);
    assert_eq!(muls, arr![i32; 2, 12, 30, 56]);
}

#[test]
fn from_iter() {
    use core::iter::repeat;

    let a: GenericArray<_, U4> = repeat(11).take(3).collect();

    assert_eq!(a, arr![i32; 11, 11, 11, 0]);
}

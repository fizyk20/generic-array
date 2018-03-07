#![recursion_limit="128"]

extern crate generic_array;

use std::fmt::Debug;
use std::ops::Add;

//use generic_array::GenericArray;
use generic_array::sequence::*;
use generic_array::functional::*;

pub fn test_generic<S>(s: S)
where
    S: FunctionalSequence<i32>,            // `.map`
    SequenceItem<S>: Add<i32, Output=i32>, // `+`
    S: MappedGenericSequence<i32, i32>,    // `i32` -> `i32`
    MappedSequence<S, i32, i32>: Debug     // println!
{
    let a = s.map(|x| x + 1);

    println!("{:?}", a);
}
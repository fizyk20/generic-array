//! This crate implements a structure that can be used as a generic array type.use
//! Core Rust array types `[T; N]` can't be used generically with respect to `N`, so for example this:
//!
//! ```ignore
//! struct Foo<T, N> {
//!     data: [T; N]
//! }
//! ```	
//!
//! won't work.
//!
//! **generic-array** exports a `GenericArray<T,N>` type, which lets the above be implemented as:
//!
//! ```ignore
//! struct Foo<T, N: ArrayLength<T>> {
//!     data: GenericArray<T,N>
//! }
//! ```	
//!
//! The `ArrayLength<T>` trait is implemented by default for [unsigned integer types](../typenum/uint/index.html) from [typenum](../typenum/index.html).
extern crate typenum;
use typenum::uint::{Unsigned, UTerm, UInt};
use typenum::bit::{B0, B1};
use std::marker::PhantomData;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::slice;

/// Trait making GenericArray work, marking types to be used as length of an array
pub unsafe trait ArrayLength<T> : Unsigned {
	/// Associated type representing the array type for the number
	type ArrayType;
}

unsafe impl<T> ArrayLength<T> for UTerm {
	type ArrayType = ();
}

/// Internal type used to generate a struct of appropriate size
#[allow(dead_code)]
#[repr(C)]
pub struct GenericArrayImplEven<T, U> {
	parent1: U,
	parent2: U,
	_marker: PhantomData<T>
}

/// Internal type used to generate a struct of appropriate size
#[allow(dead_code)]
#[repr(C)]
pub struct GenericArrayImplOdd<T, U> {
	parent1: U,
	parent2: U,
	data: T
}

unsafe impl<T, N: ArrayLength<T>> ArrayLength<T> for UInt<N, B0> {
	type ArrayType = GenericArrayImplEven<T, N::ArrayType>;
}

unsafe impl<T, N: ArrayLength<T>> ArrayLength<T> for UInt<N, B1> {
	type ArrayType = GenericArrayImplOdd<T, N::ArrayType>;
}

/// Struct representing a generic array - GenericArray<T, N> works like [T; N]
#[allow(dead_code)]
pub struct GenericArray<T, U: ArrayLength<T>> {
	data: U::ArrayType
}

impl<T, N> Deref for GenericArray<T, N> where N: ArrayLength<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        unsafe {
            slice::from_raw_parts(self as *const Self as *const T, N::to_usize())
        }
    }
}

impl<T, N> DerefMut for GenericArray<T, N> where N: ArrayLength<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe {
            slice::from_raw_parts_mut(self as *mut Self as *mut T, N::to_usize())
        }
    }
}

impl<T: Default, N> GenericArray<T, N> where N: ArrayLength<T> {

	/// Function constructing an array filled with default values
	pub fn new() -> GenericArray<T, N> {
		let mut res: GenericArray<T, N> = unsafe { mem::zeroed() };
		for i in 0..N::to_usize() {
			res[i] = T::default();
		}
		res
	}

}

impl<T: Clone, N> GenericArray<T, N> where N: ArrayLength<T> {

	/// Function constructing an array from a slice; the length of the slice must be equal to the length of the array
	pub fn from_slice(list: &[T]) -> GenericArray<T, N> {
		assert_eq!(list.len(), N::to_usize());
		let mut res: GenericArray<T, N> = unsafe { mem::zeroed() };
		for i in 0..N::to_usize() {
			res[i] = list[i].clone();
		}
		res
	}

}

#[cfg(test)]
mod test {
	extern crate typenum;
	use typenum::uint::U97;
	use super::GenericArray;

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
}

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
//! The `ArrayLength<T>` trait is implemented by default for tuples consisting of binary structs `_1` and `_0`, like `((_1, _0), _1)`.
//! This allows one to define length types as, for example, `type _6 = ((_1, _1), _0)`.
//!
//! Currently, the crate implements its own numeric types. This might change in the future.
use std::marker::PhantomData;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::slice;

/// Struct representing bit O
#[derive(Debug, Copy, Clone)]
pub struct _0;
/// Struct representing bit 1
#[derive(Debug, Copy, Clone)]
pub struct _1;

/// Nonnegative type-level integer, e.g., `((_1,_0),_1) = 0b101 = 5`.
/// Copied from shoggoth.rs
pub trait Nat {
    fn reify() -> u64;
}
impl Nat for _0 { fn reify() -> u64 { 0 } }
impl Nat for _1 { fn reify() -> u64 { 1 } }
impl<N: Nat> Nat for (N, _0) {
    fn reify() -> u64 { N::reify() << 1 }
}
impl<N: Nat> Nat for (N, _1) {
    fn reify() -> u64 { (N::reify() << 1) | 1 }
}

/// Trait making GenericArray work, marking types to be used as length of an array
pub unsafe trait ArrayLength<T> : Nat {
	/// Associated type representing the array type for the number
	type ArrayType;
}

unsafe impl<T> ArrayLength<T> for _0 {
	type ArrayType = ();
}
unsafe impl<T> ArrayLength<T> for _1 {
	type ArrayType = T;
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

unsafe impl<T, N: ArrayLength<T>> ArrayLength<T> for (N, _0) {
	type ArrayType = GenericArrayImplEven<T, N::ArrayType>;
}

unsafe impl<T, N: ArrayLength<T>> ArrayLength<T> for (N, _1) {
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
            slice::from_raw_parts(self as *const Self as *const T, N::reify() as usize)
        }
    }
}

impl<T, N> DerefMut for GenericArray<T, N> where N: ArrayLength<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe {
            slice::from_raw_parts_mut(self as *mut Self as *mut T, N::reify() as usize)
        }
    }
}

impl<T: Default, N> GenericArray<T, N> where N: ArrayLength<T> {

	/// Function constructing an array filled with default values
	pub fn new() -> GenericArray<T, N> {
		let mut res: GenericArray<T, N> = unsafe { mem::zeroed() };
		for i in 0..N::reify() as usize {
			res[i] = T::default();
		}
		res
	}

}

impl<T: Clone, N> GenericArray<T, N> where N: ArrayLength<T> {

	/// Function constructing an array from a slice; the length of the slice must be equal to the length of the array
	pub fn from_slice(list: &[T]) -> GenericArray<T, N> {
		assert_eq!(list.len(), N::reify() as usize);
		let mut res: GenericArray<T, N> = unsafe { mem::zeroed() };
		for i in 0..N::reify() as usize {
			res[i] = list[i].clone();
		}
		res
	}

}

#[cfg(test)]
mod test {
	use super::{_0, _1, GenericArray};

	type P97 = ((((((_1, _1), _0), _0), _0), _0), _1);

	#[test]
	fn test() {
		let mut list97 = [0; 97];
		for i in 0..97 {
			list97[i] = i as i32;
		}
	    let l : GenericArray<i32, P97> = GenericArray::from_slice(&list97);
	    assert_eq!(l[0], 0);
	    assert_eq!(l[1], 1);
	    assert_eq!(l[32], 32);
	    assert_eq!(l[56], 56);
	}	
}

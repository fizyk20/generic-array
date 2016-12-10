//! This crate implements a structure that can be used as a generic array type.use
//! Core Rust array types `[T; N]` can't be used generically with respect to `N`, so for example this:
//!
//! ```{should_fail}
//! struct Foo<T, N> {
//!     data: [T; N]
//! }
//! ```
//!
//! won't work.
//!
//! **generic-array** exports a `GenericArray<T,N>` type, which lets the above be implemented as:
//!
//! ```
//! # use generic_array::{ArrayLength, GenericArray};
//! struct Foo<T, N: ArrayLength<T>> {
//!     data: GenericArray<T,N>
//! }
//! ```
//!
//! The `ArrayLength<T>` trait is implemented by default for [unsigned integer types](../typenum/uint/index.html) from [typenum](../typenum/index.html).
//!
//! For ease of use, an `arr!` macro is provided - example below:
//!
//! ```
//! # #[macro_use]
//! # extern crate generic_array;
//! # extern crate typenum;
//! # fn main() {
//! let array = arr![u32; 1, 2, 3];
//! assert_eq!(array[2], 3);
//! # }
//! ```
#![no_std]
extern crate core as std;
pub extern crate typenum;
extern crate nodrop;
#[cfg(feature="serde")]
extern crate serde;
pub mod arr;
pub mod iter;
pub use iter::GenericArrayIter;

#[cfg(feature="serde")]
pub mod impl_serde;

use nodrop::NoDrop;
use typenum::uint::{Unsigned, UTerm, UInt};
use typenum::bit::{B0, B1};
use std::fmt::Debug;
use std::marker::PhantomData;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::ptr;
use std::slice;

/// Trait making `GenericArray` work, marking types to be used as length of an array
pub unsafe trait ArrayLength<T>: Unsigned {
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
    _marker: PhantomData<T>,
}

impl<T: Clone, U: Clone> Clone for GenericArrayImplEven<T, U> {
    fn clone(&self) -> GenericArrayImplEven<T, U> {
        GenericArrayImplEven {
            parent1: self.parent1.clone(),
            parent2: self.parent2.clone(),
            _marker: PhantomData,
        }
    }
}

impl<T: Copy, U: Copy> Copy for GenericArrayImplEven<T, U> {}

/// Internal type used to generate a struct of appropriate size
#[allow(dead_code)]
#[repr(C)]
pub struct GenericArrayImplOdd<T, U> {
    parent1: U,
    parent2: U,
    data: T,
}

impl<T: Clone, U: Clone> Clone for GenericArrayImplOdd<T, U> {
    fn clone(&self) -> GenericArrayImplOdd<T, U> {
        GenericArrayImplOdd {
            parent1: self.parent1.clone(),
            parent2: self.parent2.clone(),
            data: self.data.clone(),
        }
    }
}

impl<T: Copy, U: Copy> Copy for GenericArrayImplOdd<T, U> {}

unsafe impl<T, N: ArrayLength<T>> ArrayLength<T> for UInt<N, B0> {
    type ArrayType = GenericArrayImplEven<T, N::ArrayType>;
}

unsafe impl<T, N: ArrayLength<T>> ArrayLength<T> for UInt<N, B1> {
    type ArrayType = GenericArrayImplOdd<T, N::ArrayType>;
}

/// Struct representing a generic array - `GenericArray<T, N>` works like [T; N]
#[allow(dead_code)]
pub struct GenericArray<T, U: ArrayLength<T>> {
    data: U::ArrayType,
}

impl<T, N> Deref for GenericArray<T, N>
    where N: ArrayLength<T>
{
    type Target = [T];

    fn deref(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self as *const Self as *const T, N::to_usize()) }
    }
}

impl<T, N> DerefMut for GenericArray<T, N>
    where N: ArrayLength<T>
{
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self as *mut Self as *mut T, N::to_usize()) }
    }
}

impl<T, N> GenericArray<T, N>
    where N: ArrayLength<T>
{
    /// map a function over a  slice to a `GenericArray`.
    /// The length of the slice *must* be equal to the length of the array
    pub fn map_slice<S, F: Fn(&S) -> T>(s: &[S], f: F) -> GenericArray<T, N> {
        assert_eq!(s.len(), N::to_usize());
        map_inner(s, f)
    }

    /// map a function over a `GenericArray`.
    pub fn map<U, F>(self, f: F) -> GenericArray<U, N>
        where F: Fn(&T) -> U,
              N: ArrayLength<U>
    {
        map_inner(&self, f)
    }

    /// Extracts a slice containing the entire array
    pub fn as_slice(&self) -> &[T] {
        self.deref()
    }

    /// Extracts a mutable slice containing the entire array
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.deref_mut()
    }

    /// Allows access to the data in the array as a slice of bytes
    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self as *const Self as *const u8,
                                  N::to_usize() * mem::size_of::<T>())
        }
    }
}

#[inline]
fn map_inner<S, F, T, N>(list: &[S], f: F) -> GenericArray<T, N>
    where F: Fn(&S) -> T,
          N: ArrayLength<T>
{
    unsafe {
        let mut res: NoDrop<GenericArray<T, N>> = NoDrop::new(mem::uninitialized());
        for (s, r) in list.iter().zip(res.iter_mut()) {
            std::ptr::write(r, f(s))
        }
        res.into_inner()
    }
}

impl<T: Default, N> Default for GenericArray<T, N>
    where N: ArrayLength<T>
{
    fn default() -> Self {
        unsafe {
            let mut res: NoDrop<GenericArray<T, N>> = NoDrop::new(mem::uninitialized());
            for r in res.iter_mut() {
                ptr::write(r, T::default())
            }
            res.into_inner()
        }
    }
}

impl<T: Clone, N> GenericArray<T, N>
    where N: ArrayLength<T>
{
    /// Function constructing an array from a slice; the length of the slice must be equal to the length of the array
    pub fn from_slice(list: &[T]) -> GenericArray<T, N> {
        assert_eq!(list.len(), N::to_usize());
        map_inner(list, |x: &T| x.clone())
    }
}

impl<T: Clone, N> Clone for GenericArray<T, N>
    where N: ArrayLength<T>
{
    fn clone(&self) -> GenericArray<T, N> {
        unsafe {
            let mut res: NoDrop<GenericArray<T, N>> = NoDrop::new(mem::uninitialized());
            for i in 0..N::to_usize() {
                ptr::write(&mut res[i], self[i].clone())
            }
            res.into_inner()
        }
    }
}
impl<T: Copy, N> Copy for GenericArray<T, N>
    where N: ArrayLength<T>,
          N::ArrayType: Copy
{
}

impl<T: PartialEq, N> PartialEq for GenericArray<T, N>
    where N: ArrayLength<T>
{
    fn eq(&self, other: &Self) -> bool {
        **self == **other
    }
}
impl<T: Eq, N> Eq for GenericArray<T, N> where N: ArrayLength<T> {}

impl<T: Debug, N> Debug for GenericArray<T, N>
    where N: ArrayLength<T>
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        self[..].fmt(fmt)
    }
}

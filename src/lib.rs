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

#![deny(missing_docs)]
#![no_std]
pub extern crate typenum;
extern crate nodrop;
#[cfg(feature = "serde")]
extern crate serde;
pub mod arr;
pub mod iter;
pub use iter::GenericArrayIter;
mod hex;
mod impls;

#[cfg(feature = "serde")]
pub mod impl_serde;

pub mod builder;

pub use self::builder::{IterableArrayLength, RecursiveArrayBuilder};

use core::marker::PhantomData;
pub use core::mem::transmute;
use core::ops::{Deref, DerefMut};
use core::slice;
use typenum::bit::{B0, B1};
use typenum::uint::{UInt, UTerm, Unsigned};

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
where
    N: ArrayLength<T>,
{
    type Target = [T];

    fn deref(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self as *const Self as *const T, N::to_usize()) }
    }
}

impl<T, N> DerefMut for GenericArray<T, N>
where
    N: ArrayLength<T>,
{
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self as *mut Self as *mut T, N::to_usize()) }
    }
}

impl<T, N> GenericArray<T, N>
where
    N: ArrayLength<T>,
{
    /// Initializes a new `GenericArray` instance using the given function.
    ///
    /// If the generator function panics while initializing the array,
    /// any already initialized elements will be dropped.
    #[inline]
    pub fn generate<F>(f: F) -> GenericArray<T, N>
    where
        F: Fn(usize) -> T,
    {
        builder::generate(f)
    }

    /// Map a function over a slice to a `GenericArray`.
    ///
    /// The length of the slice *must* be equal to the length of the array.
    #[inline]
    pub fn map_slice<S, F: Fn(&S) -> T>(s: &[S], f: F) -> GenericArray<T, N> {
        assert_eq!(s.len(), N::to_usize());

        builder::generate(|i| f(unsafe { s.get_unchecked(i) }))
    }

    /// Maps a `GenericArray` to another `GenericArray`.
    ///
    /// If the mapping function panics, any already initialized elements in the new array
    /// will be dropped, AND any unused elements in the source array will also be dropped.
    #[inline]
    pub fn map<U, F>(self, f: F) -> GenericArray<U, N>
    where
        F: Fn(T) -> U,
        N: ArrayLength<U>,
    {
        builder::map(self, f)
    }

    /// Maps a `GenericArray` to another `GenericArray` by reference.
    ///
    /// If the mapping function panics, any already initialized elements will be dropped.
    #[inline]
    pub fn map_ref<U, F>(&self, f: F) -> GenericArray<U, N>
    where
        F: Fn(&T) -> U,
        N: ArrayLength<U>,
    {
        builder::generate(|i| f(unsafe { self.get_unchecked(i) }))
    }

    /// Combines two `GenericArray` instances and iterates through both of them,
    /// initializing a new `GenericArray` with the result of the zipped mapping function.
    ///
    /// If the mapping function panics, any already initialized elements in the new array
    /// will be dropped, AND any unused elements in the source arrays will also be dropped.
    #[inline]
    pub fn zip<B, U, F>(self, rhs: GenericArray<B, N>, f: F) -> GenericArray<U, N>
    where
        F: Fn(T, B) -> U,
        N: ArrayLength<B> + ArrayLength<U>,
    {
        builder::zip(self, rhs, f)
    }

    /// Combines two `GenericArray` instances and iterates through both of them by reference,
    /// initializing a new `GenericArray` with the result of the zipped mapping function.
    ///
    /// If the mapping function panics, any already initialized elements will be dropped.
    pub fn zip_ref<B, U, F>(&self, rhs: &GenericArray<B, N>, f: F) -> GenericArray<U, N>
    where
        F: Fn(&T, &B) -> U,
        N: ArrayLength<B> + ArrayLength<U>,
    {
        builder::generate(|i| unsafe {
            f(self.get_unchecked(i), rhs.get_unchecked(i))
        })
    }

    /// Extracts a slice containing the entire array.
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        self.deref()
    }

    /// Extracts a mutable slice containing the entire array.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.deref_mut()
    }

    /// Converts slice to a generic array reference with inferred length;
    ///
    /// Length of the slice must be equal to the length of the array.
    #[inline]
    pub fn from_slice(slice: &[T]) -> &GenericArray<T, N> {
        assert_eq!(slice.len(), N::to_usize());
        unsafe { &*(slice.as_ptr() as *const GenericArray<T, N>) }
    }

    /// Converts mutable slice to a mutable generic array reference
    ///
    /// Length of the slice must be equal to the length of the array.
    #[inline]
    pub fn from_mut_slice(slice: &mut [T]) -> &mut GenericArray<T, N> {
        assert_eq!(slice.len(), N::to_usize());
        unsafe { &mut *(slice.as_mut_ptr() as *mut GenericArray<T, N>) }
    }

    /// Recursively converts one `GenericArray` to another via the `From` trait.
    ///
    /// If `From::from` panics, any already initialized elements will be dropped.
    #[inline]
    pub fn convert<U>(self) -> GenericArray<U, N>
    where
        U: From<T>,
        N: ArrayLength<U>,
    {
        self.map(From::from)
    }
}

impl<T, N> GenericArray<T, N>
where
    N: IterableArrayLength<T, N>,
{
    /// Recursively initializes a new `GenericArray` instance using the given function.
    ///
    /// If the generator function panics while initializing the array,
    /// any already initialized elements will be dropped.
    #[inline]
    pub fn generate_recursive<F>(f: F) -> GenericArray<T, N>
    where
        F: Fn(usize) -> T,
    {
        RecursiveArrayBuilder::generate(f)
    }

    /// Recursively map a function over a  slice to a `GenericArray`.
    ///
    /// The length of the slice *must* be equal to the length of the array.
    ///
    /// If the mapping function panics, any already initialized elements will be dropped.
    #[inline]
    pub fn map_slice_recursive<S, F: Fn(&S) -> T>(s: &[S], f: F) -> GenericArray<T, N> {
        assert_eq!(s.len(), N::to_usize());

        RecursiveArrayBuilder::generate(|i| f(unsafe { s.get_unchecked(i) }))
    }

    /// Recursively maps a `GenericArray` to another `GenericArray`.
    ///
    /// If the mapping function panics, any already initialized elements in the new array
    /// will be dropped, AND any unused elements in the source array will also be dropped.
    #[inline]
    pub fn map_recursive<U, F>(self, f: F) -> GenericArray<U, N>
    where
        F: Fn(T) -> U,
        N: IterableArrayLength<U, N>,
    {
        RecursiveArrayBuilder::map(self, f)
    }

    /// Recursively maps a `GenericArray` to another `GenericArray` by reference.
    ///
    /// If the mapping function panics, any already initialized elements will be dropped.
    #[inline]
    pub fn map_ref_recursive<U, F>(&self, f: F) -> GenericArray<U, N>
    where
        F: Fn(&T) -> U,
        N: IterableArrayLength<U, N>,
    {
        RecursiveArrayBuilder::map_ref(self, f)
    }

    /// Recursively ombines two `GenericArray` instances and iterates through both of them,
    /// initializing a new `GenericArray` with the result of the zipped mapping function.
    ///
    /// If the mapping function panics, any already initialized elements in the new array
    /// will be dropped, AND any unused elements in the source arrays will also be dropped.
    #[inline]
    pub fn zip_recursive<B, U, F>(self, rhs: GenericArray<B, N>, f: F) -> GenericArray<U, N>
    where
        F: Fn(T, B) -> U,
        N: ArrayLength<B> + IterableArrayLength<U, N>,
    {
        RecursiveArrayBuilder::zip(self, rhs, f)
    }

    /// Recursively combines two `GenericArray` instances and iterates through both of them by reference,
    /// initializing a new `GenericArray` with the result of the zipped mapping function.
    ///
    /// If the mapping function panics, any already initialized elements will be dropped.
    #[inline]
    pub fn zip_ref_recursive<B, U, F>(&self, rhs: &GenericArray<B, N>, f: F) -> GenericArray<U, N>
    where
        F: Fn(&T, &B) -> U,
        N: ArrayLength<B> + IterableArrayLength<U, N>,
    {
        RecursiveArrayBuilder::zip_ref(self, rhs, f)
    }

    /// Recursively converts one `GenericArray` to another via the `From` trait.
    ///
    /// If `From::from` panics, any already initialized elements will be dropped.
    #[inline]
    pub fn convert_recursive<U>(self) -> GenericArray<U, N>
    where
        U: From<T>,
        N: IterableArrayLength<U, N>,
    {
        self.map_recursive(From::from)
    }
}

impl<T: Clone, N> GenericArray<T, N>
where
    N: ArrayLength<T>,
{
    /// Construct a `GenericArray` from a slice by cloning its content
    ///
    /// Length of the slice must be equal to the length of the array
    #[inline]
    pub fn clone_from_slice(list: &[T]) -> GenericArray<T, N> {
        GenericArray::map_slice(list, |x: &T| x.clone())
    }
}

impl<T: Clone, N> GenericArray<T, N>
where
    N: IterableArrayLength<T, N>,
{
    /// Recursively Construct a `GenericArray` from a slice by cloning its content
    ///
    /// Length of the slice must be equal to the length of the array
    #[inline]
    pub fn clone_from_slice_recursive(list: &[T]) -> GenericArray<T, N> {
        assert_eq!(list.len(), N::to_usize());

        GenericArray::map_slice(list, |x: &T| x.clone())
    }
}

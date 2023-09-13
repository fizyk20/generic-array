//! Functional programming with generic sequences
//!
//! Please see `tests/generics.rs` for examples of how to best use these in your generic functions.

use core::iter::FromIterator;

use crate::sequence::*;

/// Defines the relationship between one generic sequence and another,
/// for operations such as `map` and `zip`.
pub trait MappedGenericSequence<T, U>: GenericSequence<T> {
    /// Mapped sequence type
    type Mapped: GenericSequence<U, Length = Self::Length>;
}

impl<'a, T, U, S: MappedGenericSequence<T, U>> MappedGenericSequence<T, U> for &'a S
where
    &'a S: GenericSequence<T>,
    S: GenericSequence<T, Length = <&'a S as GenericSequence<T>>::Length>,
{
    type Mapped = <S as MappedGenericSequence<T, U>>::Mapped;
}

impl<'a, T, U, S: MappedGenericSequence<T, U>> MappedGenericSequence<T, U> for &'a mut S
where
    &'a mut S: GenericSequence<T>,
    S: GenericSequence<T, Length = <&'a mut S as GenericSequence<T>>::Length>,
{
    type Mapped = <S as MappedGenericSequence<T, U>>::Mapped;
}

/// Accessor type for a mapped generic sequence
pub type MappedSequence<S, T, U> =
    <<S as MappedGenericSequence<T, U>>::Mapped as GenericSequence<U>>::Sequence;

/// Defines functional programming methods for generic sequences
pub trait FunctionalSequence<T>: GenericSequence<T> {
    /// Maps a `GenericSequence` to another `GenericSequence`.
    ///
    /// If the mapping function panics, any already initialized elements in the new sequence
    /// will be dropped, AND any unused elements in the source sequence will also be dropped.
    #[inline]
    fn map<U, F>(self, f: F) -> MappedSequence<Self, T, U>
    where
        Self: MappedGenericSequence<T, U>,
        F: FnMut(Self::Item) -> U,
    {
        FromIterator::from_iter(self.into_iter().map(f))
    }

    /// Combines two `GenericSequence` instances and iterates through both of them,
    /// initializing a new `GenericSequence` with the result of the zipped mapping function.
    ///
    /// If the mapping function panics, any already initialized elements in the new sequence
    /// will be dropped, AND any unused elements in the source sequences will also be dropped.
    ///
    /// **WARNING**: If using the `alloc` crate feature, mixing stack-allocated
    /// `GenericArray<T, N>` and heap-allocated `Box<GenericArray<T, N>>` within [`zip`](FunctionalSequence::zip)
    /// should be done with care or avoided.
    ///
    /// For copy-types, it could be easy to accidentally move the array
    /// out of the `Box` when zipping with a stack-allocated array, which could cause a stack-overflow
    /// if the array is sufficiently large. However, that being said, the second where clause
    /// ensuring they map to the same sequence type will catch common errors, such as:
    ///
    /// ```compile_fail
    /// # use generic_array::{*, functional::FunctionalSequence};
    /// # #[cfg(feature = "alloc")]
    /// fn test() {
    ///     let stack = arr![1, 2, 3, 4];
    ///     let heap = box_arr![5, 6, 7, 8];
    ///     let mixed = stack.zip(heap, |a, b| a + b);
    ///     //                --- ^^^^ expected struct `GenericArray`, found struct `Box`
    /// }
    /// # #[cfg(not(feature = "alloc"))]
    /// # compile_error!("requires alloc feature to test this properly");
    /// ```
    #[inline]
    fn zip<B, Rhs, U, F>(self, rhs: Rhs, f: F) -> MappedSequence<Self, T, U>
    where
        Self: MappedGenericSequence<T, U>,
        Rhs: MappedGenericSequence<B, U, Mapped = MappedSequence<Self, T, U>>,
        Rhs: GenericSequence<B, Length = Self::Length>,
        F: FnMut(Self::Item, Rhs::Item) -> U,
    {
        rhs.inverted_zip2(self, f)
    }

    /// Folds (or reduces) a sequence of data into a single value.
    ///
    /// If the fold function panics, any unused elements will be dropped.
    #[inline]
    fn fold<U, F>(self, init: U, f: F) -> U
    where
        F: FnMut(U, Self::Item) -> U,
    {
        self.into_iter().fold(init, f)
    }
}

impl<'a, T, S: GenericSequence<T>> FunctionalSequence<T> for &'a S where &'a S: GenericSequence<T> {}

impl<'a, T, S: GenericSequence<T>> FunctionalSequence<T> for &'a mut S where
    &'a mut S: GenericSequence<T>
{
}

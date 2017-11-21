//! Useful traits for manipulating sequences of data stored in `GenericArray`s
#![allow(missing_docs)]

use super::*;
use core::{mem, ptr};
use core::ops::{Add, Sub};
use typenum::consts::*;
use typenum::marker_traits::*;
use typenum::operator_aliases::*;

/// Defines some `GenericArray` sequence with an associated length.
///
/// This is useful for passing N-length generic arrays as generics.
pub unsafe trait GenericSequence<T>: Sized {
    /// `GenericArray` associated length
    type Length: ArrayLength<T>;
}

unsafe impl<T, N: ArrayLength<T>> GenericSequence<T> for GenericArray<T, N> {
    type Length = N;
}

/// Defines any `GenericSequence` which can be lengthened or extended by appending
/// an element to the end of it.
///
/// Any lengthened sequence can be shortened back to the original
/// by removed the last element.
pub unsafe trait Lengthen<T>: GenericSequence<T> {
    /// `GenericSequence` that has one more element than `Self`
    type Longer: Shorten<T, Shorter = Self>;

    /// Moves all the current elements into a new array with one more element than the current one.
    ///
    /// The last element of the new array is set to `last`
    ///
    /// Example:
    ///
    /// ```ignore
    /// let a = arr![i32; 1, 2, 3, 4];
    /// let b = arr![i32; 1, 2, 3];
    ///
    /// assert_eq!(a, b.lengthen(4));
    /// ```
    fn lengthen(self, last: T) -> Self::Longer;
}

/// Defines a `GenericSequence` which can be shortened by removing the last element in it.
///
/// Additionally, any shortened sequence can be lengthened by
/// adding an element to the end of it.
pub unsafe trait Shorten<T>: GenericSequence<T> {
    /// `GenericSequence` that has one less element than `Self`
    type Shorter: Lengthen<T, Longer = Self>;

    /// Moves all but the last element into a `GenericArray` with one
    /// less element than the current one.
    ///
    /// Example:
    ///
    /// ```ignore
    /// let a = arr![i32; 1, 2, 3, 4];
    /// let b = arr![i32; 1, 2, 3];
    ///
    /// let (init, last) = a.shorten();
    ///
    /// assert_eq!(init, b);
    /// assert_eq!(last, 4);
    /// ```
    fn shorten(self) -> (Self::Shorter, T);
}

unsafe impl<T, N: ArrayLength<T>> Lengthen<T> for GenericArray<T, N>
where
    N: Add<B1>,
    Add1<N>: ArrayLength<T>,
    Add1<N>: Sub<B1, Output = N>,
    Sub1<Add1<N>>: ArrayLength<T>,
{
    type Longer = GenericArray<T, Add1<N>>;

    fn lengthen(self, last: T) -> Self::Longer {
        let mut longer: ManuallyDrop<GenericArray<T, Add1<N>>> =
            ManuallyDrop::new(unsafe { mem::uninitialized() });

        unsafe {
            ptr::write(longer.as_mut_ptr() as *mut _, self);
            ptr::write(&mut longer[N::to_usize()], last);
        }

        ManuallyDrop::into_inner(longer)
    }
}


unsafe impl<T, N: ArrayLength<T>> Shorten<T> for GenericArray<T, N>
where
    N: Sub<B1>,
    Sub1<N>: ArrayLength<T>,
    Sub1<N>: Add<B1, Output = N>,
    Add1<Sub1<N>>: ArrayLength<T>,
{
    type Shorter = GenericArray<T, Sub1<N>>;

    fn shorten(self) -> (Self::Shorter, T) {
        let head_ptr = self.as_ptr();
        let last_ptr = unsafe { head_ptr.offset(Sub1::<N>::to_usize() as isize) };

        let head = unsafe { ptr::read(head_ptr as _) };
        let last = unsafe { ptr::read(last_ptr as _) };

        mem::forget(self);

        (head, last)
    }
}

pub unsafe trait Split<T, K>: GenericSequence<T>
where
    K: ArrayLength<T>,
{
    type Head: GenericSequence<T>;
    type Tail: GenericSequence<T>;

    fn split(self) -> (Self::Head, Self::Tail);
}

unsafe impl<T, N, K> Split<T, K> for GenericArray<T, N>
where
    N: ArrayLength<T>,
    K: ArrayLength<T>,
    N: Sub<K>,
    Diff<N, K>: ArrayLength<T>,
{
    type Head = GenericArray<T, K>;
    type Tail = GenericArray<T, Diff<N, K>>;

    fn split(self) -> (Self::Head, Self::Tail) {
        let head_ptr = self.as_ptr();
        let tail_ptr = unsafe { head_ptr.offset(K::to_usize() as isize) };

        let head = unsafe { ptr::read(head_ptr as _) };
        let tail = unsafe { ptr::read(tail_ptr as _) };

        mem::forget(self);

        (head, tail)
    }
}

pub unsafe trait Concat<T, M>: GenericSequence<T>
where
    M: ArrayLength<T>,
{
    type Rest: GenericSequence<T, Length = M>;
    type Output: GenericSequence<T>;

    fn concat(self, rest: Self::Rest) -> Self::Output;
}

unsafe impl<T, N, M> Concat<T, M> for GenericArray<T, N>
where
    N: ArrayLength<T> + Add<M>,
    M: ArrayLength<T>,
    Sum<N, M>: ArrayLength<T>,
{
    type Rest = GenericArray<T, M>;
    type Output = GenericArray<T, Sum<N, M>>;

    fn concat(self, rest: Self::Rest) -> Self::Output {
        let mut output: Self::Output = unsafe { mem::uninitialized() };

        let output_ptr = output.as_mut_ptr();

        unsafe {
            ptr::write(output_ptr as *mut _, self);
            ptr::write(output_ptr.offset(N::to_usize() as isize) as *mut _, rest);
        }

        output
    }
}
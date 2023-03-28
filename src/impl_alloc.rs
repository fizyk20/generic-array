extern crate alloc;

use alloc::{boxed::Box, vec::Vec};

use crate::{ArrayLength, GenericArray, LengthError};

impl<T, N: ArrayLength> TryFrom<Vec<T>> for GenericArray<T, N> {
    type Error = crate::LengthError;

    fn try_from(v: Vec<T>) -> Result<Self, Self::Error> {
        if v.len() != N::USIZE {
            return Err(crate::LengthError);
        }

        unsafe {
            let mut destination = crate::ArrayBuilder::new();

            let (dst_iter, position) = destination.iter_position();

            dst_iter.zip(v).for_each(|(dst, src)| {
                dst.write(src);
                *position += 1;
            });

            Ok(destination.into_inner())
        }
    }
}

impl<T, N: ArrayLength> GenericArray<T, N> {
    /// Converts a `Box<GenericArray<T, N>>` into `Box<[T]>` without reallocating.
    ///
    /// This operation is O(1)
    pub fn into_boxed_slice(self: Box<GenericArray<T, N>>) -> Box<[T]> {
        unsafe {
            // SAFETY: Box ensures the array is properly aligned
            Box::from_raw(core::ptr::slice_from_raw_parts_mut(
                Box::into_raw(self) as *mut T,
                N::USIZE,
            ))
        }
    }

    /// Converts a `Box<GenericArray<T, N>>` into `Vec<T>` without reallocating.
    ///
    /// This operation is O(1)
    pub fn into_vec(self: Box<GenericArray<T, N>>) -> Vec<T> {
        Vec::from(self.into_boxed_slice())
    }

    /// Attempts to convert a `Box<[T]>` into `Box<GenericArray<T, N>>` without reallocating.
    ///
    /// This operation is O(1)
    pub fn try_from_boxed_slice(slice: Box<[T]>) -> Result<Box<GenericArray<T, N>>, LengthError> {
        if slice.len() != N::USIZE {
            return Err(LengthError);
        }

        Ok(unsafe { Box::from_raw(Box::into_raw(slice) as *mut _) })
    }

    /// Attempts to convert a `Vec<T>` into `Box<GenericArray<T, N>>` without reallocating.
    ///
    /// This operation is O(1)
    pub fn try_from_vec(vec: Vec<T>) -> Result<Box<GenericArray<T, N>>, LengthError> {
        Self::try_from_boxed_slice(vec.into_boxed_slice())
    }
}

impl<T, N: ArrayLength> TryFrom<Box<[T]>> for GenericArray<T, N> {
    type Error = crate::LengthError;

    #[inline]
    fn try_from(value: Box<[T]>) -> Result<Self, Self::Error> {
        Vec::from(value).try_into()
    }
}

impl<T, N: ArrayLength> From<GenericArray<T, N>> for Box<[T]> {
    fn from(value: GenericArray<T, N>) -> Self {
        Box::new(value).into_boxed_slice()
    }
}

impl<T, N: ArrayLength> From<GenericArray<T, N>> for Vec<T> {
    #[inline]
    fn from(value: GenericArray<T, N>) -> Self {
        Box::<[T]>::from(value).into()
    }
}

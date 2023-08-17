use alloc::{boxed::Box, vec::Vec};

use crate::{ArrayLength, GenericArray, LengthError, TryFromIterError};

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
    #[inline]
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
    #[inline]
    pub fn into_vec(self: Box<GenericArray<T, N>>) -> Vec<T> {
        Vec::from(self.into_boxed_slice())
    }

    /// Attempts to convert a `Box<[T]>` into `Box<GenericArray<T, N>>` without reallocating.
    ///
    /// This operation is O(1)
    #[inline]
    pub fn try_from_boxed_slice(slice: Box<[T]>) -> Result<Box<GenericArray<T, N>>, LengthError> {
        if slice.len() != N::USIZE {
            return Err(LengthError);
        }

        Ok(unsafe { Box::from_raw(Box::into_raw(slice) as *mut _) })
    }

    /// Attempts to convert a `Vec<T>` into `Box<GenericArray<T, N>>` without reallocating.
    ///
    /// This operation is O(1) **if the `Vec` has the same length and capacity as `N`**,
    /// otherwise it will be forced to call `Vec::shrink_to_fit` which is O(N)
    #[inline]
    pub fn try_from_vec(vec: Vec<T>) -> Result<Box<GenericArray<T, N>>, LengthError> {
        Self::try_from_boxed_slice(vec.into_boxed_slice())
    }

    /// Alternative to `Box::<GenericArray<T, N>>::default()` that won't overflow the stack for very large arrays.
    ///
    /// The standard `Box::default()` calls `default` on the inner type, creating it on the stack,
    /// and then moves it onto the heap. Optimized release builds often remove this step, but debug builds
    /// may have issues.
    #[inline]
    pub fn default_boxed() -> Box<GenericArray<T, N>>
    where
        T: Default,
    {
        Box::<GenericArray<T, N>>::generate(|_| T::default())
    }

    /// Like [`GenericArray::try_from_iter`] but returns a `Box<GenericArray<T, N>>` instead.
    pub fn try_boxed_from_iter<I>(iter: I) -> Result<Box<GenericArray<T, N>>, TryFromIterError>
    where
        I: IntoIterator<Item = T>,
    {
        let mut iter = iter.into_iter();

        // pre-checks
        match iter.size_hint() {
            (n, _) if n < N::USIZE => return Err(TryFromIterError::TooShort),
            (_, Some(n)) if n > N::USIZE => return Err(TryFromIterError::TooLong),
            _ => {}
        }

        let mut v = Vec::with_capacity(N::USIZE);
        v.extend((&mut iter).take(N::USIZE));

        if v.len() != N::USIZE {
            return Err(TryFromIterError::TooShort);
        }

        if iter.next().is_some() {
            return Err(TryFromIterError::TooLong);
        }

        Ok(GenericArray::try_from_vec(v).unwrap())
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

impl<T, N: ArrayLength> IntoIterator for Box<GenericArray<T, N>> {
    type IntoIter = alloc::vec::IntoIter<T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        GenericArray::into_vec(self).into_iter()
    }
}

impl<T, N: ArrayLength> FromIterator<T> for Box<GenericArray<T, N>> {
    /// Create a `Box<GenericArray>` from an iterator.
    ///
    /// Will panic if the number of elements is not exactly the array length.
    ///
    /// See [`GenericArray::try_boxed_from_iter]` for a fallible alternative.
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        match GenericArray::try_boxed_from_iter(iter) {
            Ok(res) => res,
            Err(err) => crate::from_iter_length_fail(N::USIZE, err),
        }
    }
}

use crate::functional::{FunctionalSequence, MappedGenericSequence};
use crate::GenericSequence;

unsafe impl<T, N: ArrayLength> GenericSequence<T> for Box<GenericArray<T, N>> {
    type Length = N;
    type Sequence = Box<GenericArray<T, N>>;

    fn generate<F>(mut f: F) -> Self::Sequence
    where
        F: FnMut(usize) -> T,
    {
        let mut v = Vec::with_capacity(N::USIZE);
        for i in 0..N::USIZE {
            v.push(f(i));
        }
        GenericArray::try_from_vec(v).unwrap()
    }
}

impl<T, U, N: ArrayLength> MappedGenericSequence<T, U> for Box<GenericArray<T, N>> {
    type Mapped = Box<GenericArray<U, N>>;
}

impl<T, N: ArrayLength> FunctionalSequence<T> for Box<GenericArray<T, N>> where
    Self: GenericSequence<T, Item = T, Length = N>
{
}

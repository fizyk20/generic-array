use core::borrow::{Borrow, BorrowMut};
use core::cmp::Ordering;
use core::fmt::{self, Debug};
use core::hash::{Hash, Hasher};

use super::{ArrayLength, GenericArray};

use crate::functional::*;
use crate::sequence::*;

impl<T: Default, N: ArrayLength> Default for GenericArray<T, N> {
    #[inline(always)]
    fn default() -> Self {
        Self::generate(|_| T::default())
    }
}

impl<T: Clone, N: ArrayLength> Clone for GenericArray<T, N> {
    fn clone(&self) -> GenericArray<T, N> {
        self.map(Clone::clone)
    }
}

impl<T: Copy, N: ArrayLength> Copy for GenericArray<T, N> where N::ArrayType<T>: Copy {}

impl<T: PartialEq, N: ArrayLength> PartialEq for GenericArray<T, N> {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        **self == **other
    }
}
impl<T: Eq, N: ArrayLength> Eq for GenericArray<T, N> {}

impl<T: PartialOrd, N: ArrayLength> PartialOrd for GenericArray<T, N> {
    #[inline(always)]
    fn partial_cmp(&self, other: &GenericArray<T, N>) -> Option<Ordering> {
        PartialOrd::partial_cmp(self.as_slice(), other.as_slice())
    }
}

impl<T: Ord, N: ArrayLength> Ord for GenericArray<T, N> {
    #[inline(always)]
    fn cmp(&self, other: &GenericArray<T, N>) -> Ordering {
        Ord::cmp(self.as_slice(), other.as_slice())
    }
}

impl<T: Debug, N: ArrayLength> Debug for GenericArray<T, N> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.as_slice().fmt(fmt)
    }
}

impl<T, N: ArrayLength> Borrow<[T]> for GenericArray<T, N> {
    #[inline(always)]
    fn borrow(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T, N: ArrayLength> BorrowMut<[T]> for GenericArray<T, N> {
    #[inline(always)]
    fn borrow_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T, N: ArrayLength> AsRef<[T]> for GenericArray<T, N> {
    #[inline(always)]
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T, N: ArrayLength> AsMut<[T]> for GenericArray<T, N> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T: Hash, N: ArrayLength> Hash for GenericArray<T, N> {
    #[inline]
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        Hash::hash(self.as_slice(), state)
    }
}

use typenum::{Const, ToUInt, U};

impl<T, const N: usize> From<[T; N]> for GenericArray<T, U<N>>
where
    Const<N>: ToUInt,
    U<N>: ArrayLength,
{
    #[inline(always)]
    fn from(value: [T; N]) -> Self {
        unsafe { crate::transmute(value) }
    }
}

impl<T, const N: usize> From<GenericArray<T, U<N>>> for [T; N]
where
    Const<N>: ToUInt,
    U<N>: ArrayLength,
{
    #[inline(always)]
    fn from(value: GenericArray<T, U<N>>) -> Self {
        unsafe { crate::transmute(value) }
    }
}

impl<'a, T, const N: usize> From<&'a [T; N]> for &'a GenericArray<T, U<N>>
where
    Const<N>: ToUInt,
    U<N>: ArrayLength,
{
    #[inline(always)]
    fn from(slice: &'a [T; N]) -> Self {
        unsafe { &*(slice.as_ptr() as *const GenericArray<T, U<N>>) }
    }
}

impl<'a, T, const N: usize> From<&'a mut [T; N]> for &'a mut GenericArray<T, U<N>>
where
    Const<N>: ToUInt,
    U<N>: ArrayLength,
{
    #[inline(always)]
    fn from(slice: &'a mut [T; N]) -> Self {
        unsafe { &mut *(slice.as_mut_ptr() as *mut GenericArray<T, U<N>>) }
    }
}

impl<T, const N: usize> AsRef<[T; N]> for GenericArray<T, U<N>>
where
    Const<N>: ToUInt,
    U<N>: ArrayLength,
{
    #[inline(always)]
    fn as_ref(&self) -> &[T; N] {
        unsafe { crate::transmute(self) }
    }
}
impl<T, const N: usize> AsMut<[T; N]> for GenericArray<T, U<N>>
where
    Const<N>: ToUInt,
    U<N>: ArrayLength,
{
    #[inline(always)]
    fn as_mut(&mut self) -> &mut [T; N] {
        unsafe { crate::transmute(self) }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_from_inference() {
        let a = arr![1, 2, 3, 4];
        let _: [i8; 4] = a.into();
    }
}

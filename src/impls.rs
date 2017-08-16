use super::{ArrayLength, GenericArray};
use core::borrow::{Borrow, BorrowMut};
use core::cmp::Ordering;
use core::fmt::{self, Debug};
use core::hash::{Hash, Hasher};

impl<T: Default, N> Default for GenericArray<T, N>
where
    N: ArrayLength<T>,
{
    #[inline]
    fn default() -> Self {
        Self::generate(|_| T::default())
    }
}

impl<T: Clone, N> Clone for GenericArray<T, N>
where
    N: ArrayLength<T>,
{
    fn clone(&self) -> GenericArray<T, N> {
        self.map_ref(|x| x.clone())
    }
}
impl<T: Copy, N> Copy for GenericArray<T, N>
where
    N: ArrayLength<T>,
    N::ArrayType: Copy,
{
}

impl<T: PartialEq, N> PartialEq for GenericArray<T, N>
where
    N: ArrayLength<T>,
{
    fn eq(&self, other: &Self) -> bool {
        **self == **other
    }
}
impl<T: Eq, N> Eq for GenericArray<T, N>
where
    N: ArrayLength<T>,
{
}

impl<T: PartialOrd, N> PartialOrd for GenericArray<T, N>
where
    N: ArrayLength<T>,
{
    fn partial_cmp(&self, other: &GenericArray<T, N>) -> Option<Ordering> {
        PartialOrd::partial_cmp(&self, &other)
    }
}

impl<T: Ord, N> Ord for GenericArray<T, N>
where
    N: ArrayLength<T>,
{
    fn cmp(&self, other: &GenericArray<T, N>) -> Ordering {
        Ord::cmp(&self, &other)
    }
}

impl<T: Debug, N> Debug for GenericArray<T, N>
where
    N: ArrayLength<T>,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self[..].fmt(fmt)
    }
}

impl<T, N> Borrow<[T]> for GenericArray<T, N>
where
    N: ArrayLength<T>,
{
    fn borrow(&self) -> &[T] {
        &self[..]
    }
}

impl<T, N> BorrowMut<[T]> for GenericArray<T, N>
where
    N: ArrayLength<T>,
{
    fn borrow_mut(&mut self) -> &mut [T] {
        &mut self[..]
    }
}

impl<T, N> AsRef<[T]> for GenericArray<T, N>
where
    N: ArrayLength<T>,
{
    fn as_ref(&self) -> &[T] {
        &self[..]
    }
}

impl<T, N> AsMut<[T]> for GenericArray<T, N>
where
    N: ArrayLength<T>,
{
    fn as_mut(&mut self) -> &mut [T] {
        &mut self[..]
    }
}

impl<T: Hash, N> Hash for GenericArray<T, N>
where
    N: ArrayLength<T>,
{
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        Hash::hash(&self[..], state)
    }
}

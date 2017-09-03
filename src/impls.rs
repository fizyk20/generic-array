use super::{ArrayLength, GenericArray};
use core::{mem, ptr};
use core::borrow::{Borrow, BorrowMut};
use core::cmp::Ordering;
use core::fmt::{self, Debug};
use core::hash::{Hash, Hasher};
use nodrop::NoDrop;

impl<T: Default, N> Default for GenericArray<T, N>
where
    N: ArrayLength<T>,
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

impl<T: Clone, N> Clone for GenericArray<T, N>
where
    N: ArrayLength<T>,
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
        PartialOrd::partial_cmp(self.as_slice(), other.as_slice())
    }
}

impl<T: Ord, N> Ord for GenericArray<T, N>
where
    N: ArrayLength<T>,
{
    fn cmp(&self, other: &GenericArray<T, N>) -> Ordering {
        Ord::cmp(self.as_slice(), other.as_slice())
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

macro_rules! impl_from {
    ($n: expr, $ty: ty) => {
        impl<T> From<[T; $n]> for GenericArray<T, $ty> {
            fn from(arr: [T; $n]) -> Self {
                use core::mem::{forget, transmute_copy};
                let x = unsafe { transmute_copy(&arr) };
                forget(arr);
                x
            }
        }

    }
}

impl_from!(1, ::typenum::U1);
impl_from!(2, ::typenum::U2);
impl_from!(3, ::typenum::U3);
impl_from!(4, ::typenum::U4);
impl_from!(5, ::typenum::U5);
impl_from!(6, ::typenum::U6);
impl_from!(7, ::typenum::U7);
impl_from!(8, ::typenum::U8);
impl_from!(9, ::typenum::U9);
impl_from!(10, ::typenum::U10);
impl_from!(11, ::typenum::U11);
impl_from!(12, ::typenum::U12);
impl_from!(13, ::typenum::U13);
impl_from!(14, ::typenum::U14);
impl_from!(15, ::typenum::U15);
impl_from!(16, ::typenum::U16);
impl_from!(17, ::typenum::U17);
impl_from!(18, ::typenum::U18);
impl_from!(19, ::typenum::U19);
impl_from!(20, ::typenum::U20);
impl_from!(21, ::typenum::U21);
impl_from!(22, ::typenum::U22);
impl_from!(23, ::typenum::U23);
impl_from!(24, ::typenum::U24);
impl_from!(25, ::typenum::U25);
impl_from!(26, ::typenum::U26);
impl_from!(27, ::typenum::U27);
impl_from!(28, ::typenum::U28);
impl_from!(29, ::typenum::U29);
impl_from!(30, ::typenum::U30);
impl_from!(31, ::typenum::U31);
impl_from!(32, ::typenum::U32);

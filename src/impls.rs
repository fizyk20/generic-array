use super::{ArrayLength, GenericArray};
use core::cmp::Ordering;
use core::fmt;
use core::fmt::Debug;
use core::mem;
use core::ptr;
use nodrop::NoDrop;

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

impl<T: PartialOrd, N> PartialOrd for GenericArray<T, N>
    where N: ArrayLength<T>
{
    fn partial_cmp(&self, other: &GenericArray<T, N>) -> Option<Ordering> {
        PartialOrd::partial_cmp(&self, &other)
    }
}

impl<T: Ord, N> Ord for GenericArray<T, N>
    where N: ArrayLength<T>
{
    fn cmp(&self, other: &GenericArray<T, N>) -> Ordering {
        Ord::cmp(&self, &other)
    }
}

impl<T: Debug, N> Debug for GenericArray<T, N>
    where N: ArrayLength<T>
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self[..].fmt(fmt)
    }
}

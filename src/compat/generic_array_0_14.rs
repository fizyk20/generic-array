#![allow(deprecated)] // 0.14 is deprecated, but this is specifically for compatibility with it to allow gradual migration.

use generic_array_0_14::{ArrayLength as ArrayLength_0_14, GenericArray as GenericArray_0_14};

use crate::{ArrayLength, GenericArray};

impl<T, N: ArrayLength + ArrayLength_0_14<T>> From<GenericArray_0_14<T, N>> for GenericArray<T, N> {
    #[inline(always)]
    fn from(value: GenericArray_0_14<T, N>) -> Self {
        GenericArray::from_0_14(value)
    }
}

impl<T, N: ArrayLength + ArrayLength_0_14<T>> From<GenericArray<T, N>> for GenericArray_0_14<T, N> {
    #[inline(always)]
    fn from(value: GenericArray<T, N>) -> Self {
        value.into_0_14()
    }
}

impl<T, N: ArrayLength + ArrayLength_0_14<T>> AsRef<GenericArray_0_14<T, N>>
    for GenericArray<T, N>
{
    #[inline(always)]
    fn as_ref(&self) -> &GenericArray_0_14<T, N> {
        self.as_0_14()
    }
}

impl<T, N: ArrayLength + ArrayLength_0_14<T>> AsMut<GenericArray_0_14<T, N>>
    for GenericArray<T, N>
{
    #[inline(always)]
    fn as_mut(&mut self) -> &mut GenericArray_0_14<T, N> {
        self.as_0_14_mut()
    }
}

impl<T, N: ArrayLength + ArrayLength_0_14<T>> GenericArray<T, N> {
    /// From `&self` of this version, create a reference to the [`GenericArray`](GenericArray_0_14) type from `generic-array` 0.14.
    #[inline(always)]
    pub const fn as_0_14(&self) -> &GenericArray_0_14<T, N> {
        unsafe { core::mem::transmute(self) }
    }

    /// From `&mut self` of this version, create a mutable reference to the [`GenericArray`](GenericArray_0_14) type from `generic-array` 0.14.
    ///
    /// This method is `const` since Rust 1.83.0, but non-`const` before.
    #[rustversion::attr(since(1.83), const)]
    #[inline(always)]
    pub fn as_0_14_mut(&mut self) -> &mut GenericArray_0_14<T, N> {
        unsafe { core::mem::transmute(self) }
    }

    /// From `self` of this version, create the [`GenericArray`](GenericArray_0_14) type from `generic-array` 0.14.
    #[inline(always)]
    pub const fn into_0_14(self) -> GenericArray_0_14<T, N> {
        unsafe { crate::const_transmute(self) }
    }

    /// From the [`GenericArray`](GenericArray_0_14) type from `generic-array` 0.14, create a [`GenericArray`] of this version.
    #[inline(always)]
    pub const fn from_0_14(value: GenericArray_0_14<T, N>) -> Self {
        unsafe { crate::const_transmute(value) }
    }
}

#[cfg(test)]
mod tests {
    use super::GenericArray_0_14;
    use crate::{arr, typenum::U4, GenericArray};

    #[test]
    fn owned_round_trip() {
        let ga = arr![1u8, 2, 3, 4];

        let g14: GenericArray_0_14<u8, U4> = ga.into_0_14();
        assert_eq!(g14.as_slice(), &[1, 2, 3, 4]);

        let back = GenericArray::from_0_14(g14);
        assert_eq!(back, arr![1u8, 2, 3, 4]);

        // via From impls
        let g14b: GenericArray_0_14<u8, U4> = arr![5u8, 6, 7, 8].into();
        let gab: GenericArray<u8, U4> = g14b.into();
        assert_eq!(gab, arr![5u8, 6, 7, 8]);
    }

    #[test]
    fn reference_conversions() {
        let ga = arr![1u8, 2, 3, 4];

        assert_eq!(ga.as_0_14().as_slice(), &[1, 2, 3, 4]);
        let r: &GenericArray_0_14<u8, U4> = ga.as_ref();
        assert_eq!(r.as_slice(), &[1, 2, 3, 4]);

        let mut ga = arr![1u8, 2, 3, 4];
        ga.as_0_14_mut()[0] = 9;
        AsMut::<GenericArray_0_14<u8, U4>>::as_mut(&mut ga)[1] = 8;
        assert_eq!(ga, arr![9u8, 8, 3, 4]);
    }
}

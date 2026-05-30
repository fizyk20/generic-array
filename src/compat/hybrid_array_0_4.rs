use hybrid_array_0_4::{Array as HybridArray, ArraySize, AsArrayMut, AsArrayRef, AssocArraySize};

use crate::{ArrayLength, GenericArray};

impl<T, N: ArrayLength + ArraySize> AssocArraySize for GenericArray<T, N> {
    type Size = N;
}

impl<T, N: ArrayLength + ArraySize> AsArrayRef<T> for GenericArray<T, N> {
    #[inline(always)]
    fn as_array_ref(&self) -> &HybridArray<T, N> {
        self.as_ha0_4()
    }
}

impl<T, N: ArrayLength + ArraySize> AsArrayMut<T> for GenericArray<T, N> {
    #[inline(always)]
    fn as_array_mut(&mut self) -> &mut HybridArray<T, N> {
        self.as_ha0_4_mut()
    }
}

impl<T, N: ArrayLength + ArraySize> From<HybridArray<T, N>> for GenericArray<T, N> {
    #[inline(always)]
    fn from(value: HybridArray<T, N>) -> Self {
        GenericArray::from_ha0_4(value)
    }
}

impl<T, N: ArrayLength + ArraySize> From<GenericArray<T, N>> for HybridArray<T, N> {
    #[inline(always)]
    fn from(value: GenericArray<T, N>) -> Self {
        value.into_ha0_4()
    }
}

impl<T, N: ArrayLength + ArraySize> AsRef<HybridArray<T, N>> for GenericArray<T, N> {
    #[inline(always)]
    fn as_ref(&self) -> &HybridArray<T, N> {
        self.as_ha0_4()
    }
}

impl<T, N: ArrayLength + ArraySize> AsMut<HybridArray<T, N>> for GenericArray<T, N> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut HybridArray<T, N> {
        self.as_ha0_4_mut()
    }
}

impl<T, N: ArrayLength + ArraySize> GenericArray<T, N> {
    /// From `&self` of this version, create a reference to a [`Array`](hybrid_array_0_4::Array) from `hybrid-array` 0.4.
    #[inline(always)]
    pub const fn as_ha0_4(&self) -> &HybridArray<T, N> {
        unsafe { core::mem::transmute(self) }
    }

    /// From `&mut self` of this version, create a mutable reference to a [`Array`](hybrid_array_0_4::Array) from `hybrid-array` 0.4.
    ///
    /// This method is `const` since Rust 1.83.0, but non-`const` before.
    #[rustversion::attr(since(1.83), const)]
    #[inline(always)]
    pub fn as_ha0_4_mut(&mut self) -> &mut HybridArray<T, N> {
        unsafe { core::mem::transmute(self) }
    }

    /// From `self` of this version, create a [`Array`](hybrid_array_0_4::Array) from `hybrid-array` 0.4.
    #[inline(always)]
    pub const fn into_ha0_4(self) -> HybridArray<T, N> {
        unsafe { crate::const_transmute(self) }
    }

    /// From a [`Array`](hybrid_array_0_4::Array) from `hybrid-array` 0.4, create a [`GenericArray`] of this version.
    #[inline(always)]
    pub const fn from_ha0_4(value: HybridArray<T, N>) -> Self {
        unsafe { crate::const_transmute(value) }
    }
}

#[cfg(test)]
mod tests {
    use super::HybridArray;
    use crate::{arr, typenum::U4, GenericArray};
    use hybrid_array_0_4::{AsArrayMut, AsArrayRef};

    #[test]
    fn owned_round_trip() {
        let ga = arr![1u8, 2, 3, 4];

        let ha: HybridArray<u8, U4> = ga.into_ha0_4();
        assert_eq!(ha.as_slice(), &[1, 2, 3, 4]);

        let back = GenericArray::from_ha0_4(ha);
        assert_eq!(back, arr![1u8, 2, 3, 4]);

        // via From impls
        let ha2: HybridArray<u8, U4> = arr![5u8, 6, 7, 8].into();
        let ga2: GenericArray<u8, U4> = ha2.into();
        assert_eq!(ga2, arr![5u8, 6, 7, 8]);
    }

    #[test]
    fn reference_conversions() {
        let ga = arr![1u8, 2, 3, 4];

        assert_eq!(ga.as_ha0_4().as_slice(), &[1, 2, 3, 4]);
        let r: &HybridArray<u8, U4> = ga.as_ref();
        assert_eq!(r.as_slice(), &[1, 2, 3, 4]);
        assert_eq!(ga.as_array_ref().as_slice(), &[1, 2, 3, 4]);

        let mut ga = arr![1u8, 2, 3, 4];
        ga.as_ha0_4_mut()[0] = 9;
        AsMut::<HybridArray<u8, U4>>::as_mut(&mut ga)[1] = 8;
        ga.as_array_mut()[2] = 7;
        assert_eq!(ga, arr![9u8, 8, 7, 4]);
    }
}

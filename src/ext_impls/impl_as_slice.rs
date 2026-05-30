use as_slice::{AsMutSlice, AsSlice};

use crate::{ArrayLength, GenericArray};

impl<T, N: ArrayLength> AsSlice for GenericArray<T, N> {
    type Element = T;

    #[inline(always)]
    fn as_slice(&self) -> &[T] {
        self.as_ref()
    }
}

impl<T, N: ArrayLength> AsMutSlice for GenericArray<T, N> {
    #[inline(always)]
    fn as_mut_slice(&mut self) -> &mut [T] {
        self.as_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::{arr, typenum::U4, GenericArray};
    use as_slice::{AsMutSlice, AsSlice};

    #[test]
    fn as_slice_impls() {
        let mut a = arr![1u8, 2, 3, 4];
        assert_eq!(<GenericArray<u8, U4> as AsSlice>::as_slice(&a), &[1, 2, 3, 4]);
        <GenericArray<u8, U4> as AsMutSlice>::as_mut_slice(&mut a)[0] = 9;
        assert_eq!(a[0], 9);
    }
}

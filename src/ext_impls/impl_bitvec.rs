use bitvec::{
    order::BitOrder,
    ptr::BitSpanError,
    slice::BitSlice,
    store::BitStore,
    view::{BitView, BitViewSized},
};

use crate::{ArrayLength, GenericArray};

impl<T, N: ArrayLength> BitView for GenericArray<T, N>
where
    T: BitStore,
{
    type Store = T;

    #[inline(always)]
    fn view_bits<O>(&self) -> &BitSlice<T, O>
    where
        O: BitOrder,
    {
        BitSlice::from_slice(self)
    }

    #[inline(always)]
    fn try_view_bits<O>(&self) -> Result<&BitSlice<T, O>, BitSpanError<T>>
    where
        O: BitOrder,
    {
        BitSlice::try_from_slice(self)
    }

    #[inline(always)]
    fn view_bits_mut<O>(&mut self) -> &mut BitSlice<T, O>
    where
        O: BitOrder,
    {
        BitSlice::from_slice_mut(self)
    }

    #[inline(always)]
    fn try_view_bits_mut<O>(&mut self) -> Result<&mut BitSlice<T, O>, BitSpanError<T>>
    where
        O: BitOrder,
    {
        BitSlice::try_from_slice_mut(self)
    }
}

use const_default::ConstDefault;

#[repr(transparent)]
struct ZeroHelper<T>(T);

impl<T> ConstDefault for ZeroHelper<T>
where
    T: BitStore,
{
    const DEFAULT: Self = Self(T::ZERO);
}

/// BitViewSized implementation for GenericArray
///
/// Ignore the ZeroHelper; it's just there to provide a way to create a zeroed
/// GenericArray as a const.
impl<T, N: ArrayLength> BitViewSized for GenericArray<T, N>
where
    T: BitStore,
    GenericArray<ZeroHelper<T>, N>: ConstDefault,
{
    // SAFETY: ZeroHelper<T> is transparent over T, so this is the same memory layout,
    // but we can exploit ConstDefault to create a zeroed array from T::ZERO.
    const ZERO: Self = unsafe { crate::const_transmute(GenericArray::<ZeroHelper<T>, N>::DEFAULT) };

    #[inline(always)]
    fn as_raw_slice(&self) -> &[Self::Store] {
        self.as_slice()
    }

    #[inline(always)]
    fn as_raw_mut_slice(&mut self) -> &mut [Self::Store] {
        self.as_mut_slice()
    }
}

#[cfg(test)]
mod tests {
    use crate::{arr, typenum::U2, GenericArray};
    use bitvec::order::Lsb0;
    use bitvec::view::{BitView, BitViewSized};

    #[test]
    fn view_bits() {
        let a = arr![0b0000_1010u8, 0u8];

        let bits = a.view_bits::<Lsb0>();
        assert_eq!(bits.len(), 16);
        assert!(!bits[0]);
        assert!(bits[1]);
        assert!(bits[3]);

        let ok = a.try_view_bits::<Lsb0>();
        assert!(ok.is_ok());
    }

    #[test]
    fn view_bits_mut() {
        let mut a = arr![0u8, 0u8];

        a.view_bits_mut::<Lsb0>().set(0, true);
        a.view_bits_mut::<Lsb0>().set(9, true);
        assert_eq!(a, arr![0b0000_0001u8, 0b0000_0010u8]);

        assert!(a.try_view_bits_mut::<Lsb0>().is_ok());
    }

    #[test]
    fn bit_view_sized_zero() {
        // `ZERO` is built via a `const_transmute` of a `ZeroHelper` array
        let z = GenericArray::<u8, U2>::ZERO;
        assert_eq!(z, arr![0u8, 0u8]);

        let mut z = z;
        assert_eq!(z.as_raw_slice(), &[0u8, 0u8]);
        z.as_raw_mut_slice()[0] = 5;
        assert_eq!(z[0], 5);
    }
}

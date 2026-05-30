use arbitrary::Arbitrary;

use crate::{sequence::FallibleGenericSequence as _, ArrayLength, GenericArray};

impl<'a, T, N: ArrayLength> Arbitrary<'a> for GenericArray<T, N>
where
    T: Arbitrary<'a>,
{
    #[inline]
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        GenericArray::try_generate(|_| T::arbitrary(u)).unwrap_or_else(|e| match e {})
    }

    #[inline]
    fn arbitrary_take_rest(mut u: arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let mut array = Self::arbitrary(&mut u)?;
        if let Some(last) = array.last_mut() {
            *last = T::arbitrary_take_rest(u)?;
        }
        Ok(array)
    }

    fn size_hint(depth: usize) -> (usize, Option<usize>) {
        Self::try_size_hint(depth).unwrap_or_default()
    }

    fn try_size_hint(
        depth: usize,
    ) -> arbitrary::Result<(usize, Option<usize>), arbitrary::MaxRecursionReached> {
        let hint = <T as Arbitrary>::try_size_hint(depth)?;

        // same as `arbitrary::size_hint::and_all(...)` but without allocations
        Ok(core::iter::repeat(hint)
            .take(N::USIZE)
            .fold((0, Some(0)), arbitrary::size_hint::and))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        typenum::{U0, U4},
        GenericArray,
    };
    use arbitrary::{Arbitrary, Unstructured};

    #[test]
    fn arbitrary_and_take_rest() {
        let data = [1u8, 2, 3, 4, 5, 6, 7, 8];

        let mut u = Unstructured::new(&data);
        let a = GenericArray::<u8, U4>::arbitrary(&mut u).unwrap();
        assert_eq!(a.len(), 4);

        let u = Unstructured::new(&data);
        let b = GenericArray::<u8, U4>::arbitrary_take_rest(u).unwrap();
        assert_eq!(b.len(), 4);

        // zero-length array: `last_mut()` is None, exercising the empty branch
        // of `arbitrary_take_rest`.
        let u = Unstructured::new(&data);
        let z = GenericArray::<u8, U0>::arbitrary_take_rest(u).unwrap();
        assert_eq!(z.len(), 0);
    }

    #[test]
    fn size_hints() {
        // four `u8`s, each contributing (1, Some(1))
        let (lo, hi) = <GenericArray<u8, U4> as Arbitrary>::size_hint(0);
        assert_eq!((lo, hi), (4, Some(4)));

        let hint = <GenericArray<u8, U4> as Arbitrary>::try_size_hint(0).unwrap();
        assert_eq!(hint, (4, Some(4)));
    }
}

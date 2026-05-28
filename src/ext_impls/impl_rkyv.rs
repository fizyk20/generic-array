// The `Archive`, `Serialize` and `Deserialize` impls below mirror rkyv's own impls for
// `[T; N]` (`rkyv-0.8/src/impls/core/mod.rs`) 1:1. In particular, they share the same
// behavior on a failed element-wise serialize/deserialize: previously-written entries in
// the resolver/result `MaybeUninit` are leaked rather than dropped. For typical rkyv
// `Resolver`s (often `()`) and `Copy` element types this is a non-issue; staying in sync
// with upstream is preferable to diverging here.

use core::mem;

use rkyv::{
    rancor::Fallible,
    traits::{CopyOptimization, NoUndef},
    Archive, Deserialize, Place, Portable, Serialize,
};

use crate::{ArrayLength, GenericArray};

// SAFETY: `GenericArray<T, N>` is a `T` array and so is portable as long as `T` is also
// `Portable`.
unsafe impl<T: Portable, N: ArrayLength> Portable for GenericArray<T, N> {}
// SAFETY: `GenericArray<T, N>` is a `T` array and so has no uninitialized bytes as long as
// `T` also has no uninitialized bytes.
unsafe impl<T: NoUndef, N: ArrayLength> NoUndef for GenericArray<T, N> {}

/// Gets a `Place` to the `i`-th element of the array.
///
/// # Safety
///
/// `i` must be in-bounds for the array pointed to by this place.
///
/// This is a 1:1 copy of [`Place<[T; N]>::index`]
unsafe fn index_for_place_generic_array<T, N: ArrayLength>(
    place: Place<GenericArray<T, N>>,
    i: usize,
) -> Place<T> {
    // SAFETY: The caller has guaranteed that `i` is in-bounds for the array
    // pointed to by this place.
    let ptr = unsafe { place.ptr().cast::<T>().add(i) };
    // SAFETY: `ptr` is an element of `self`, and so is also properly
    // aligned, dereferenceable, and all of its bytes are initialized.
    unsafe { Place::new_unchecked(place.pos() + i * mem::size_of::<T>(), ptr) }
}

impl<T: Archive, N: ArrayLength> Archive for GenericArray<T, N> {
    const COPY_OPTIMIZATION: CopyOptimization<Self> =
        unsafe { CopyOptimization::enable_if(T::COPY_OPTIMIZATION.is_enabled()) };

    type Archived = GenericArray<T::Archived, N>;
    type Resolver = GenericArray<T::Resolver, N>;

    fn resolve(&self, resolver: Self::Resolver, out: Place<Self::Archived>) {
        for (i, (value, resolver)) in self.iter().zip(resolver).enumerate() {
            let out_i = unsafe { index_for_place_generic_array(out, i) };
            value.resolve(resolver, out_i);
        }
    }
}

impl<T, S, N: ArrayLength> Serialize<S> for GenericArray<T, N>
where
    T: Serialize<S>,
    S: Fallible + ?Sized,
{
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        let mut result = core::mem::MaybeUninit::<Self::Resolver>::uninit();
        let result_ptr = result.as_mut_ptr().cast::<T::Resolver>();
        for (i, value) in self.iter().enumerate() {
            unsafe {
                result_ptr.add(i).write(value.serialize(serializer)?);
            }
        }
        unsafe { Ok(result.assume_init()) }
    }
}

impl<T, D, N: ArrayLength> Deserialize<GenericArray<T, N>, D> for GenericArray<T::Archived, N>
where
    T: Archive,
    T::Archived: Deserialize<T, D>,
    D: Fallible + ?Sized,
{
    fn deserialize(&self, deserializer: &mut D) -> Result<GenericArray<T, N>, D::Error> {
        let mut result = core::mem::MaybeUninit::<GenericArray<T, N>>::uninit();
        let result_ptr = result.as_mut_ptr().cast::<T>();
        for (i, value) in self.iter().enumerate() {
            unsafe {
                result_ptr.add(i).write(value.deserialize(deserializer)?);
            }
        }
        unsafe { Ok(result.assume_init()) }
    }
}

#[cfg(test)]
mod tests {
    use crate::typenum::{U0, U32, U6};
    use crate::{arr, GenericArray};
    use rkyv::rancor::Error;
    use rkyv::traits::{NoUndef, Portable};

    const fn assert_portable_noundef<T: Portable + NoUndef>() {}
    const _: () = assert_portable_noundef::<GenericArray<u8, U32>>();
    const _: () = assert_portable_noundef::<GenericArray<u8, U0>>();

    #[test]
    fn test_rkyv_roundtrip() {
        let array: GenericArray<u32, U6> = arr![1, 2, 3, 4, 5, 6];
        let bytes = rkyv::to_bytes::<Error>(&array).unwrap();
        let archived =
            unsafe { rkyv::access_unchecked::<rkyv::Archived<GenericArray<u32, U6>>>(&bytes) };
        for (i, el) in archived.iter().enumerate() {
            assert_eq!(el.to_native(), array[i]);
        }
        let deserialized: GenericArray<u32, U6> =
            rkyv::deserialize::<GenericArray<u32, U6>, Error>(archived).unwrap();
        assert_eq!(deserialized, array);
    }

    // Exercises a `T` with a non-trivial `Resolver` and `Drop` (`String` archives via an
    // out-of-line buffer, so `Resolver` carries position metadata that owns nothing but
    // the deserialized `T` does). A regression in either Serialize or Deserialize that
    // miscounted indices would surface here as a corrupted string or a leak under Miri.
    #[cfg(feature = "alloc")]
    #[test]
    fn test_rkyv_roundtrip_string() {
        use alloc::string::String;
        use typenum::U3;

        let array: GenericArray<String, U3> = arr![
            String::from("hello"),
            String::from("rkyv"),
            String::from("world")
        ];
        let bytes = rkyv::to_bytes::<Error>(&array).unwrap();
        let archived =
            unsafe { rkyv::access_unchecked::<rkyv::Archived<GenericArray<String, U3>>>(&bytes) };
        for (i, el) in archived.iter().enumerate() {
            assert_eq!(el.as_str(), array[i].as_str());
        }
        let deserialized: GenericArray<String, U3> =
            rkyv::deserialize::<GenericArray<String, U3>, Error>(archived).unwrap();
        assert_eq!(deserialized, array);
    }
}

#[cfg(all(test, feature = "bytecheck-0_8"))]
mod tests_full {
    use crate::typenum::U6;
    use crate::{arr, GenericArray};
    use rkyv::rancor::Error;

    #[test]
    fn test_validated_roundtrip() {
        let array: GenericArray<u32, U6> = arr![10, 20, 30, 40, 50, 60];
        let bytes = rkyv::to_bytes::<Error>(&array).unwrap();
        let deserialized: GenericArray<u32, U6> =
            rkyv::from_bytes::<GenericArray<u32, U6>, Error>(&bytes).unwrap();
        assert_eq!(deserialized, array);
    }

    #[test]
    fn test_validation_rejects_truncated() {
        let array: GenericArray<u32, U6> = arr![1, 2, 3, 4, 5, 6];
        let bytes = rkyv::to_bytes::<Error>(&array).unwrap();
        let truncated = &bytes[..bytes.len() - 1];
        let result = rkyv::access::<rkyv::Archived<GenericArray<u32, U6>>, Error>(truncated);
        assert!(result.is_err());
    }
}

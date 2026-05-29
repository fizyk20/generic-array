use core::fmt;

use bytecheck_0_8::{
    rancor::{Fallible, ResultExt, Trace},
    CheckBytes,
};

use crate::{ArrayLength, GenericArray};

// Mirrors `bytecheck::ArrayCheckContext` and the `CheckBytes` impl for `[T; N]` in
// `bytecheck-0.8/src/lib.rs`, renamed so the trace message identifies the type as a
// `GenericArray`.
#[derive(Debug)]
struct GenericArrayCheckContext {
    index: usize,
}

impl fmt::Display for GenericArrayCheckContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "while checking index '{}' of GenericArray", self.index)
    }
}

// SAFETY: `check_bytes` only returns `Ok` if each element of the array is
// valid. If each element of the array is valid then the whole array is also
// valid.
unsafe impl<T, N: ArrayLength, C> CheckBytes<C> for GenericArray<T, N>
where
    T: CheckBytes<C>,
    C: Fallible + ?Sized,
    C::Error: Trace,
{
    #[inline]
    unsafe fn check_bytes(value: *const Self, context: &mut C) -> Result<(), C::Error> {
        let base = value.cast::<T>();
        for index in 0..N::USIZE {
            // SAFETY: The caller has guaranteed that `value` points to enough
            // bytes for this array and is properly aligned, so we can create
            // pointers to each element and check them.
            unsafe {
                T::check_bytes(base.add(index), context)
                    .with_trace(|| GenericArrayCheckContext { index })?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use bytecheck_0_8 as bytecheck;

    use crate::typenum::{U0, U4};
    use crate::{arr, GenericArray};
    use bytecheck::check_bytes;
    use bytecheck::rancor::Error;

    #[test]
    fn test_check_bytes_valid_u8() {
        let array: GenericArray<u8, U4> = arr![1, 2, 3, 4];
        // SAFETY: pointer is to an aligned, fully-initialized GenericArray<u8, U4>.
        unsafe {
            check_bytes::<GenericArray<u8, U4>, Error>(&array).unwrap();
        }
    }

    #[test]
    fn test_check_bytes_empty() {
        let array: GenericArray<u8, U0> = arr![];
        // SAFETY: empty arrays are always valid.
        unsafe {
            check_bytes::<GenericArray<u8, U0>, Error>(&array).unwrap();
        }
    }

    #[test]
    fn test_check_bytes_invalid_bool() {
        // 0 and 1 are valid bool bit patterns; 2 is not.
        let bytes: [u8; 4] = [1, 0, 1, 2];
        let ptr = &bytes as *const [u8; 4] as *const GenericArray<bool, U4>;
        // SAFETY: pointer is aligned (u8 == bool alignment) and points to 4 initialized bytes.
        let result = unsafe { check_bytes::<GenericArray<bool, U4>, Error>(ptr) };
        assert!(result.is_err());
    }
}

//! Generic array are commonly used as a return value for hash digests, so
//! it's a good idea to allow to hexlify them easily. This module implements
//! `std::fmt::LowerHex` and `std::fmt::UpperHex` traits.
//!
//! Example:
//!
//! ```rust
//! use generic_array::arr;
//! use generic_array::typenum;
//!
//! let array = arr![10u8, 20, 30];
//! assert_eq!(format!("{:x}", array), "0a141e");
//! ```

use core::{cmp::min, fmt, ops::Add, str};

use typenum::*;

use crate::{ArrayLength, GenericArray};

static LOWER_CHARS: [u8; 16] = *b"0123456789abcdef";
static UPPER_CHARS: [u8; 16] = *b"0123456789ABCDEF";

fn generic_hex<N: ArrayLength>(
    arr: &GenericArray<u8, N>,
    alphabet: &[u8; 16], // use fixed-length array to avoid slice index checks
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result
where
    N: Add<N>,
    Sum<N, N>: ArrayLength,
{
    let max_digits = N::USIZE * 2;
    let max_digits = match f.precision() {
        Some(precision) if precision < max_digits => precision,
        _ => max_digits,
    };

    let max_hex = (max_digits >> 1) + (max_digits & 1);

    if N::USIZE <= 1024 {
        // For small arrays use a stack allocated
        // buffer of 2x number of bytes
        let mut res = GenericArray::<u8, Sum<N, N>>::default();

        arr.iter().take(max_hex).enumerate().for_each(|(i, c)| {
            res[i * 2] = alphabet[(c >> 4) as usize];
            res[i * 2 + 1] = alphabet[(c & 0xF) as usize];
        });

        f.write_str(unsafe { str::from_utf8_unchecked(&res[..max_digits]) })?;
    } else {
        // For large array use chunks of up to 1024 bytes (2048 hex chars)
        let mut buf = [0u8; 2048];
        let mut digits_left = max_digits;

        for chunk in arr[..max_hex].chunks(1024) {
            chunk.iter().enumerate().for_each(|(i, c)| {
                buf[i * 2] = alphabet[(c >> 4) as usize];
                buf[i * 2 + 1] = alphabet[(c & 0xF) as usize];
            });

            let n = min(chunk.len() * 2, digits_left);
            f.write_str(unsafe { str::from_utf8_unchecked(&buf[..n]) })?;
            digits_left -= n;
        }
    }
    Ok(())
}

impl<N: ArrayLength> fmt::LowerHex for GenericArray<u8, N>
where
    N: Add<N>,
    Sum<N, N>: ArrayLength,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        generic_hex(self, &LOWER_CHARS, f)
    }
}

impl<N: ArrayLength> fmt::UpperHex for GenericArray<u8, N>
where
    N: Add<N>,
    Sum<N, N>: ArrayLength,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        generic_hex(self, &UPPER_CHARS, f)
    }
}

//! Implementation for `arr!` macro.

/// Macro allowing for easy generation of Generic Arrays.
///
/// Type-inference works similarly to `vec![]`
///
/// **`arr!` can be used in `const` expressions.**
///
/// Example:
/// ```
/// # use generic_array::arr;
/// use generic_array::typenum::U6;
///
/// let test = arr![1, 2, 3]; // implicit length
/// let test = arr![1; 6];    // explicit length via `Const<N>`
/// let test = arr![1; U6];   // explicit length via typenum
/// ```
///
/// NOTE: As of `generic-array 1.0`, [`From`] can be used for a wide range of regular arrays as well.
///
/// The `[T; N]` variants are limited to `Copy` values. Use
/// [`GenericArray::generate(|| value.clone())`](crate::GenericSequence::generate) for non-`Copy` items.
#[macro_export]
macro_rules! arr {
    ($($x:expr),* $(,)*) => ( $crate::GenericArray::from_array([$($x),*]) );
    ($x:expr; $N:ty)     => ( $crate::GenericArray::from_array([$x; <$N as $crate::typenum::Unsigned>::USIZE]) );
    ($x:expr; $n:expr)   => ( $crate::GenericArray::from_array([$x; $n]) );
}

/// Like [`arr!`], but returns a `Box<GenericArray<T, N>>`
///
/// Unlike [`arr!`], this is not limited by stack size, only the heap.
///
/// Example:
/// ```
/// # use generic_array::{box_arr, typenum::{self, *}};
/// // allocate a 16MB Buffer of u128 elements (16 bytes * 10 ^ 6)
/// # #[cfg(not(miri))]
/// let test = box_arr![1u128; typenum::Exp<U10, U6>];
/// //  test: Box<GenericArray<u128, _>>
/// ```
#[cfg(feature = "alloc")]
#[macro_export]
macro_rules! box_arr {
    ($($x:expr),* $(,)*) => ({
        type __OutputLength = $crate::box_arr_helper!(@count_ty $($x),*);
        $crate::GenericArray::<_, __OutputLength>::try_from_vec($crate::alloc::vec![$($x),*]).unwrap()
    });
    ($x:expr; $N:ty) => ( $crate::GenericArray::<_, $N>::try_from_vec($crate::alloc::vec![$x; <$N as $crate::typenum::Unsigned>::USIZE]).unwrap() );
    ($x:expr; $n:expr) => ({
        const __LEN: usize = $n;

        $crate::GenericArray::<_, <$crate::typenum::Const<__LEN> as $crate::IntoArrayLength>::ArrayLength>::try_from_vec($crate::alloc::vec![$x; __LEN]).unwrap()
    });
}

#[cfg(feature = "alloc")]
#[doc(hidden)]
#[macro_export]
macro_rules! box_arr_helper {
    (@count_ty) => { $crate::typenum::U0 };
    (@count_ty $val:expr$(, $vals:expr)* $(,)?) => { $crate::typenum::Add1<$crate::box_arr_helper!(@count_ty $($vals),*)> };
}

mod doctests_only {
    ///
    /// Testing that lifetimes aren't transmuted when they're ellided.
    ///
    /// ```compile_fail
    /// #[macro_use] extern crate generic_array;
    /// fn unsound_lifetime_extension<'a, A>(a: &'a A) -> &'static A {
    ///     arr![a as &A][0]
    /// }
    /// ```
    ///
    /// ```rust
    /// #[macro_use] extern crate generic_array;
    /// fn unsound_lifetime_extension<'a, A>(a: &'a A) -> &'a A {
    ///     arr![a][0]
    /// }
    /// ```
    #[allow(dead_code)]
    pub enum DocTests {}
}

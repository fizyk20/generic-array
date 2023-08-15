//! Implementation for `arr!` macro.

#[doc(hidden)]
#[macro_export]
macro_rules! arr_impl {
    (@replace_expr $e:expr) => { 1 };
    (@count_ty) => { $crate::typenum::U0 };
    (@count_ty $val:expr$(, $vals:expr)* $(,)?) => { $crate::typenum::Add1<$crate::arr_impl!(@count_ty $($vals),*)> };
    ($($x:expr),*) => ({
        const __INPUT_LENGTH: usize = 0 $(+ $crate::arr_impl!(@replace_expr $x) )*;
        type __OutputLength = $crate::arr_impl!(@count_ty $($x),*);

        #[inline(always)]
        const fn __do_transmute<T, N: $crate::ArrayLength>(arr: [T; __INPUT_LENGTH]) -> $crate::GenericArray<T, N> {
            unsafe { $crate::const_transmute(arr) }
        }

        const _: [(); <__OutputLength as $crate::typenum::Unsigned>::USIZE] = [(); __INPUT_LENGTH];

        __do_transmute::<_, __OutputLength>([$($x),*])
    });
    ($x:expr; $N:ty) => ({
        const __INPUT_LENGTH: usize = <$N as $crate::typenum::Unsigned>::USIZE;

        #[inline(always)]
        const fn __do_transmute<T, N: $crate::ArrayLength>(arr: [T; __INPUT_LENGTH]) -> $crate::GenericArray<T, N> {
            unsafe { $crate::const_transmute(arr) }
        }

        __do_transmute::<_, $N>([$x; __INPUT_LENGTH])
    });
}

/// Macro allowing for easy generation of Generic Arrays.
///
/// Type-inference works similarly to `vec![]`
///
/// Can be used in `const` contexts.
///
/// Example:
/// ```
/// # use generic_array::arr;
/// use generic_array::typenum::U6;
///
/// let test = arr![1, 2, 3]; // implicit length
/// let test = arr![1; U6];   // explicit length
/// ```
///
/// NOTE: As of `generic-array 1.0`, [`From`] can be used for a wide range of regular arrays as well.
#[macro_export]
macro_rules! arr {
    ($($x:expr),* $(,)*) => ( $crate::arr_impl!($($x),*) );
    ($x:expr; $N:ty)     => ( $crate::arr_impl!($x; $N) );
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
        type __OutputLength = $crate::arr_impl!(@count_ty $($x),*);
        $crate::GenericArray::<_, __OutputLength>::try_from_vec($crate::alloc::vec![$($x),*]).unwrap()
    });
    ($x:expr; $N:ty) => ( $crate::GenericArray::<_, $N>::try_from_vec($crate::alloc::vec![$x; <$N as $crate::typenum::Unsigned>::USIZE]).unwrap() );
}

mod doctests_only {
    ///
    /// # With ellision
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
    ///
    /// # Without ellision
    ///
    /// Testing that lifetimes aren't transmuted when they're specified explicitly.
    ///
    /// ```compile_fail
    /// #[macro_use] extern crate generic_array;
    /// fn unsound_lifetime_extension<'a, A>(a: &'a A) -> &'static A {
    ///     arr![a][0]
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

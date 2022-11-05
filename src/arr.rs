//! Implementation for `arr!` macro.

use super::ArrayLength;
use core::ops::Add;
use typenum::U1;

/// Helper trait for `arr!` macro
pub trait AddLength<T, N: ArrayLength<T>>: ArrayLength<T> {
    /// Resulting length
    type Output: ArrayLength<T>;
}

impl<T, N1, N2> AddLength<T, N2> for N1
where
    N1: ArrayLength<T> + Add<N2>,
    N2: ArrayLength<T>,
    <N1 as Add<N2>>::Output: ArrayLength<T>,
{
    type Output = <N1 as Add<N2>>::Output;
}

/// Helper type for `arr!` macro
pub type Inc<T, U> = <U as AddLength<T, U1>>::Output;

#[doc(hidden)]
#[macro_export]
macro_rules! arr_impl {
    (@replace_expr $e:expr) => { 1 };
    (@count_ty) => { $crate::typenum::U0 };
    (@count_ty $val:expr$(, $vals:expr)* $(,)?) => { $crate::typenum::Add1<$crate::arr_impl!(@count_ty $($vals),*)> };
    ($T:ty; $($x:expr),*) => ({
        const __INPUT_LENGTH: usize = 0 $(+ $crate::arr_impl!(@replace_expr $x) )*;
        type __OutputLength = $crate::arr_impl!(@count_ty $($x),*);

        #[inline(always)]
        const fn __do_transmute<T, N: $crate::ArrayLength<T>>(arr: [T; __INPUT_LENGTH]) -> $crate::GenericArray<T, N> {
            unsafe { $crate::transmute(arr) }
        }

        const _: [(); <__OutputLength as $crate::typenum::Unsigned>::USIZE] = [(); __INPUT_LENGTH];

        __do_transmute::<$T, __OutputLength>([$($x as $T),*])
    });
}

/// Macro allowing for easy generation of Generic Arrays.
/// Example: `let test = arr![u32; 1, 2, 3];`
#[macro_export]
macro_rules! arr {
    ($T:ty; $(,)*) => ({
        unsafe { $crate::transmute::<[$T; 0], $crate::GenericArray<$T, $crate::typenum::U0>>([]) }
    });
    ($T:ty; $($x:expr),* $(,)*) => (
        $crate::arr_impl!($T; $($x),*)
    );
    ($($x:expr,)+) => (arr![$($x),+]);
    () => ("""Macro requires a type, e.g. `let array = arr![u32; 1, 2, 3];`")
}

mod doctests_only {
    ///
    /// # With ellision
    ///
    /// Testing that lifetimes aren't transmuted when they're ellided.
    ///
    /// ```compile_fail
    /// #[macro_use] extern crate generic_array;
    /// fn main() {
    ///    fn unsound_lifetime_extension<'a, A>(a: &'a A) -> &'static A {
    ///        arr![&A; a][0]
    ///    }
    /// }
    /// ```
    ///
    /// ```rust
    /// #[macro_use] extern crate generic_array;
    /// fn main() {
    ///    fn unsound_lifetime_extension<'a, A>(a: &'a A) -> &'a A {
    ///        arr![&A; a][0]
    ///    }
    /// }
    /// ```
    ///
    /// # Without ellision
    ///
    /// Testing that lifetimes aren't transmuted when they're specified explicitly.
    ///
    /// ```compile_fail
    /// #[macro_use] extern crate generic_array;
    /// fn main() {
    ///    fn unsound_lifetime_extension<'a, A>(a: &'a A) -> &'static A {
    ///        arr![&'a A; a][0]
    ///    }
    /// }
    /// ```
    ///
    /// ```compile_fail
    /// #[macro_use] extern crate generic_array;
    /// fn main() {
    ///    fn unsound_lifetime_extension<'a, A>(a: &'a A) -> &'static A {
    ///        arr![&'static A; a][0]
    ///    }
    /// }
    /// ```
    ///
    /// ```rust
    /// #[macro_use] extern crate generic_array;
    /// fn main() {
    ///    fn unsound_lifetime_extension<'a, A>(a: &'a A) -> &'a A {
    ///        arr![&'a A; a][0]
    ///    }
    /// }
    /// ```
    #[allow(dead_code)]
    pub enum DocTests {}
}

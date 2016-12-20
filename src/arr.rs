use typenum::U1;
use std::ops::Add;
use super::ArrayLength;

/// Helper trait for `arr!` macro
pub trait AddLength<T, N: ArrayLength<T>>: ArrayLength<T> {
    type Output: ArrayLength<T>;
}

impl<T, N1, N2> AddLength<T, N2> for N1
    where N1: ArrayLength<T> + Add<N2>,
          N2: ArrayLength<T>,
          <N1 as Add<N2>>::Output: ArrayLength<T>
{
    type Output = <N1 as Add<N2>>::Output;
}

/// Helper type for `arr!` macro
pub type Inc<T, U> = <U as AddLength<T, U1>>::Output;

/// Proper implementation of `arr!`
#[macro_export]
macro_rules! arr_impl {
    ($T:ty; $N:ty, [$($x:expr),*], []) => ({
        use generic_array::typenum::U0;
        use generic_array::GenericArray;
        use generic_array::arr::Inc;
        GenericArray::<$T, $N>::from_slice(&[$($x),*])
    });
    ($T:ty; $N:ty, [], [$x1:expr]) => (
        arr_impl!($T; Inc<$T, $N>, [$x1], [])
    );
    ($T:ty; $N:ty, [], [$x1:expr, $($x:expr),+]) => (
        arr_impl!($T; Inc<$T, $N>, [$x1], [$($x),*])
    );
    ($T:ty; $N:ty, [$($y:expr),+], [$x1:expr]) => (
        arr_impl!($T; Inc<$T, $N>, [$($y),*, $x1], [])
    );
    ($T:ty; $N:ty, [$($y:expr),+], [$x1:expr, $($x:expr),+]) => (
        arr_impl!($T; Inc<$T, $N>, [$($y),*, $x1], [$($x),*])
    );
}

/// Macro allowing for easy generation of Generic Arrays.
/// Example: `let test = arr![u32; 1, 2, 3];`
#[macro_export]
macro_rules! arr {
    ($T:ty; $($x:expr),*) => (
        arr_impl!($T; U0, [], [$($x),*])
    );
    ($($x:expr,)+) => (arr![$($x),*]);
    () => ("""Macro requires a type, e.g. `let array = arr![u32; 1, 2, 3];`")
}

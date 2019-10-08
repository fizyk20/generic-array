//! Implementation for `arr!` macro.

/// Macro allowing for easy generation of Generic Arrays.
/// Example: `let test = arr![u32; 1, 2, 3];`
#[macro_export]
macro_rules! arr {
    ($T:ty; $($rest:tt)*) => {{
        let out: $crate::GenericArray<$T, _> = $crate::arr![$($rest)*];
        out
    }};
    ($first:expr $(, $rest:expr)* $(,)*) => {
        $crate::sequence::Lengthen::prepend($crate::arr![$($rest),*], $first)
    };
    ($(,)*) => {
        $crate::GenericArray::new()
    };
}

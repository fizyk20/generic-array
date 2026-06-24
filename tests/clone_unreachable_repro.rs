//! Regression test for the Gemini audit's finding #1.
//!
//! The internal `GenericArrayImpl*` containers are `#[doc(hidden)]` but must remain
//! nameable via `<N as ArrayLength>::ArrayType<T>` (a `typenum` limitation). They carry
//! a safe `Clone` impl whose body is unreachable in normal use, because
//! `GenericArray<T, N>::clone` delegates to `self.map(Clone::clone)` and never invokes
//! the container's own `Clone`.
//!
//! Previously that body was `unreachable_unchecked()`, so a caller who explicitly named
//! the internal type and called `.clone()` on it hit UB from 100% safe code (reachable
//! via the `const-default` feature's `ConstDefault::DEFAULT`). It is now `unreachable!()`,
//! turning that misuse into a deterministic panic instead. This test pins that behavior.
//!
//! Requires `const-default` for a fully-safe way to construct the internal type.
#![cfg(feature = "const-default")]

use const_default::ConstDefault;
use generic_array::ArrayLength;

#[test]
#[should_panic(expected = "should never be called")]
fn cloning_internal_array_type_panics_not_ub() {
    type Inner = <typenum::U2 as ArrayLength>::ArrayType<i32>;

    // Fully safe construction of an internal container type.
    let arr = <Inner as ConstDefault>::DEFAULT;

    // Fully safe call into the never-meant-to-be-called Clone impl. Must panic
    // deterministically (was UB via unreachable_unchecked before the fix).
    let _cloned = arr.clone();
}

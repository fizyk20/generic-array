[![Crates.io](https://img.shields.io/crates/v/generic-array.svg)](https://crates.io/crates/generic-array)
[![Build Status](https://github.com/fizyk20/generic-array/actions/workflows/CI.yml/badge.svg)](https://github.com/fizyk20/generic-array/actions/workflows/CI.yml)
# generic-array

This crate implements a structure that can be used as a generic array type.

**Requires minumum Rust version of 1.65.0

[Documentation on GH Pages](https://fizyk20.github.io/generic-array/generic_array/) may be required to view certain types on foreign crates.

## Usage

Before Rust 1.51, arrays `[T; N]` were problematic in that they couldn't be generic with respect to the length `N`, so this wouldn't work:

```rust
struct Foo<N> {
    data: [i32; N],
}
```

Since 1.51, the below syntax is valid:

```rust
struct Foo<const N: usize> {
    data: [i32; N],
}
```

However, the const-generics we have as of writing this are still the minimum-viable product (`min_const_generics`), so many situations still result in errors, such as this example:

```rust
trait Bar {
    const LEN: usize;

    // Error: cannot perform const operation using `Self`
    fn bar(&self) -> Foo<{ Self::LEN }>;
}
```

**generic-array** defines a new trait `ArrayLength` and a struct `GenericArray<T, N: ArrayLength>`, which lets the above be implemented as:

```rust
struct Foo<N: ArrayLength> {
    data: GenericArray<i32, N>
}

trait Bar {
    type LEN: ArrayLength;
    fn bar(&self) -> Foo<Self::LEN>;
}
```

The `ArrayLength` trait is implemented for [unsigned integer types](http://fizyk20.github.io/generic-array/typenum/uint/index.html) from [typenum](http://fizyk20.github.io/generic-array/typenum/index.html) crate. For example, `GenericArray<T, U5>` would work almost like `[T; 5]`:

```rust
use generic_array::typenum::U5;

struct Foo<T, N: ArrayLength> {
    data: GenericArray<T, N>
}

let foo = Foo::<i32, U5> { data: GenericArray::default() };
```

The `arr!` macro is provided to allow easier creation of literal arrays, as shown below:

```rust
let array = arr![1, 2, 3];
//  array: GenericArray<i32, typenum::U3>
assert_eq!(array[2], 3);
```

## Feature flags

```toml
[dependencies.generic-array]
features = [
    "more_lengths",  # Expands From/Into implementation for more array lengths
    "serde",         # Serialize/Deserialize implementation
    "zeroize",       # Zeroize implementation for setting array elements to zero
    "const-default", # Compile-time const default value support via trait
    "alloc",         # Enables From/TryFrom implementations between GenericArray and Vec<T>/Box<[T]>
    "faster-hex"     # Enables internal use of the `faster-hex` crate for faster hex encoding via SIMD
]
```
[![Crates.io](https://img.shields.io/crates/v/generic-array.svg)](https://crates.io/crates/generic-array)
[![Build Status](https://travis-ci.org/fizyk20/generic-array.svg?branch=master)](https://travis-ci.org/fizyk20/generic-array)
# generic-array

This crate implements generic array types for Rust.

**Requires minumum Rust version of 1.65.0

[Documentation](http://fizyk20.github.io/generic-array/generic_array/)

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

However, the const-generics we have as of writing this are still the minimum-viable product (`min_const_generics`), so many situations still result in erors, such as this example:

```rust
trait Bar {
    const LEN: usize;

    // Error: cannot perform const operation using `Self`
    fn bar(&self) -> [i32; Self::LEN];
}
```

**generic-array** defines a new trait `ArrayLength` and a struct `GenericArray<T, N: ArrayLength>`, which let the above be implemented as:

```rust
struct Foo<N: ArrayLength> {
	data: GenericArray<i32, N>
}

trait Bar {
    type LEN: ArrayLength;
    fn bar(&self) -> GenericArray<i32, Self::LEN>;
}
```

The `ArrayLength` trait is implemented by default for [unsigned integer types](http://fizyk20.github.io/generic-array/typenum/uint/index.html) from [typenum](http://fizyk20.github.io/generic-array/typenum/index.html) crate:

```rust
use generic_array::typenum::U5;

struct Foo<N: ArrayLength> {
    data: GenericArray<i32, N>
}

fn main() {
    let foo = Foo::<U5>{data: GenericArray::default()};
}
```

For example, `GenericArray<T, U5>` would work almost like `[T; 5]`:

```rust
use generic_array::typenum::U5;

struct Foo<T, N: ArrayLength> {
    data: GenericArray<T, N>
}

fn main() {
    let foo = Foo::<i32, U5>{data: GenericArray::default()};
}
```

In version 0.1.1 an `arr!` macro was introduced, allowing for creation of arrays as shown below:

```rust
let array = arr![u32; 1, 2, 3];
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
    "alloc"          # Enables TryFrom<Vec<T>> implementation
]
```
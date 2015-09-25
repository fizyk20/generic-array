[![Build Status](https://travis-ci.org/fizyk20/generic-array.svg?branch=master)](https://travis-ci.org/fizyk20/generic-array)
# generic-array

This crate implements generic array types for Rust.

## Usage

The Rust arrays `[T; N]` are problematic in that they can't be used generically with respect to `N`, so for example this won't work:

```rust
struct Foo<N> {
	data: [i32; N]
}
```

**generic-array** defines a new trait `ArrayLength<T>` and a struct `GenericArray<T, N: ArrayLength<T>>`, which let the above be implemented as:

```rust
struct Foo<N: ArrayLength<i32>> {
	data: GenericArray<i32, N>
}
```

To actually define a type implementing `ArrayLength`, you can use two types `_0` and `_1` grouped in tuples to form a binary representation, e.g.:

```
type _5 = ((_1, _0), _1);
```

Then you can use `GenericArray<T, _5>` almost like you would use `[T; 5]` :)